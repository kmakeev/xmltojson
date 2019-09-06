extern crate encoding_rs;
extern crate xmlJSON;
extern crate rustc_serialize;
extern crate glob;
extern crate rayon;
extern crate reql;
extern crate reql_types;
extern crate futures;

extern crate serde_json;
extern crate ql2;

use rayon::prelude::*;
use std::process::exit;
use encoding_rs::WINDOWS_1251;
use std::fs;
use std::path::Path;
use xmlJSON::XmlDocument;
use rustc_serialize::json::{self, ToJson, Json};
use std::str::FromStr;
use glob::{MatchOptions, glob_with};
use std::convert::TryInto;
use reql::{Config, Client, Document, Run, Arg, IntoArg};
use reql_types::{ServerStatus, WriteStatus};
use futures::{Stream, Join};

use serde_json :: {Value};

use ql2::proto::{Term, Term_AssocPair as TermPair};

const PATCH: &str = "/Users/konstantin/rust/xmltojson/*.xml";
const DATABASE: &str = "test";
const FILE_TABLE: &str = "file";
const DOCS_TABLE: &str = "docs";


fn parse_xml_file_info(file: String) -> bool {

    let r = Client::new();
    let test_conn = r.connect(Config::default());
    match test_conn {
        Ok(ok) => {
            println!("Connect to rethinkdb in thread {:#?}", ok);
        }
        Err(err) => {
            println!("Error create connect to rethinkdb in thread {:#?}", err);
            exit(1);
        }
    };

    let conn = test_conn.unwrap();

    let buff = fs::read(file).expect("Unable to read file");
    let (res, _enc, errors) = WINDOWS_1251.decode(&buff);
    let test2 = res.to_string();
    //println!("{}", test2);

    //let test = "<note type=\"Reminder\">
    //            test
    //        </note>";

    //let data = XmlDocument::from_str(&test2).unwrap();
    if errors {
        // println!("Failed encode input file");
    false
    } else {

        let document = XmlDocument::from_str(&test2).unwrap();

        //let document : XmlDocument = XmlDocument::from_str(&res.to_string()).unwrap();

        let json_data : json::Json = document.to_json();
        //let json_str: String = json_data.to_string();
        //fs::write("/home/konstantin/rust/xmltojson/output.json", json_str).expect("Unable to write output file");
        // println!("object? {}", json_data.is_object());
        let json_obj = json_data.as_object().unwrap();

        let file = json_obj.get("Файл").unwrap().as_object().unwrap();
        // println!("Save json : {:?}", file);

        let body = file.get("$").unwrap().as_object().unwrap();
        // println!("Save json : {:?}", body.to_json());

        //let str_body = body.to_json().to_string().as_str();
        // let data = r#"{"cmd": "askldjhfasdfa;sldkjfal;skdjflfeuiahfiuseh faiusehf asiuehf aiuseh f", "count": 0}"#;
        // let mut arg = Arg::new();

        let obj: Value = serde_json::from_str(body.to_json().to_string().as_str()).unwrap();
        println!("Save info about file with Id - {:#?}", obj["ИдФайл"]);
        // println!("data - {:#?}", arg);
        let mut arg = Arg::new();
        arg.add_arg(obj.into_arg());
        arg.set_string("{conflict: replace}");
        let opt = Arg::create_term_pair("conflict", "replace");
        match opt {
            Ok(pair) => arg.add_opt(pair),
            Err(err) => arg.set_term(Err(err)),
        }

        let w = r.db(DATABASE)
            .table(FILE_TABLE)
            .insert(arg)
            .run::<ServerStatus>(conn).unwrap().wait().next().unwrap();
        match w {
            // The server returned the response we were expecting
            Ok(Some(Document::Expected(status))) => {
                println!("Ok response - {:?}", status);
            }
            Ok(Some(Document::Unexpected(status))) => {
                println!("response from server: {:?}", status);
            }
            Ok(None) => {
                println!("got no documents in the database");
            }
            // Oops! We ran into an error
            Err(error) => {
                println!("error: {:?}", error);
            }
        }

        let docs = file.get("Документ").unwrap();
        // println!("object? docs {}", docs.is_array());
        if docs.is_array() {
            match docs.as_array() {
                Some(docs) => {
                    for item in docs {
                        // println!("Item - {:#?}", item);
                        let body = item.as_object().unwrap().get("$").unwrap().as_object().unwrap();
                        // println!("Write document {:#?}", body);
                        // let swyl = item.as_object().unwrap().get("СвЮЛ").unwrap().as_object().unwrap();
                        // println!("{:#?}", swyl);
                        // break;
                        let doc = item;
                        let obj: Value = serde_json::from_str(doc.to_string().as_str()).unwrap();
                        println!("Save document with {:#?}", obj["$"]);
                        // println!("data - {:#?}", arg);
                         let mut arg = Arg::new();
                         arg.add_arg(obj.into_arg());
                        // arg.set_string("{conflict: replace}");
                        //let opt = Arg::create_term_pair("conflict", "replace");
                        //match opt {
                        //    Ok(pair) => arg.add_opt(pair),
                        //    Err(err) => arg.set_term(Err(err)),
                        //}

                        let w = r.db(DATABASE)
                            .table(DOCS_TABLE)
                            .insert(arg)
                            .run::<WriteStatus>(conn);
                        match w.unwrap().wait().next().unwrap() {
                            // The server returned the response we were expecting
                            Ok(Some(Document::Expected(status))) => {
                                println!("Ok response - {:?}", status);
                            }
                            Ok(Some(Document::Unexpected(status))) => {
                                println!("response from server: {:?}", status);
                            }
                            Ok(None) => {
                                println!("got no documents in the database");
                            }
                            // Oops! We ran into an error
                            Err(error) => {
                                println!("error: {:?}", error);
                            }
                        }
                    }
                }
                None => {
                    println!("Error unwrap array docs in {:#?}", file);
                }
            }
        } else {
            let body = docs.as_object().unwrap().get("$").unwrap().as_object().unwrap();
            println!("{:#?}", body);
        }

    true
    }

}


fn main() {
    let r = Client::new();
    let test_conn = r.connect(Config::default());
    match test_conn {
        Ok(ok) => {
            println!("Connect to rethinkdb {:#?}", ok);
        }
        Err(err) => {
            println!("Error create connect to rethinkdb {:#?}", err);
            exit(1);
        }
    };

    let conn = test_conn.unwrap();
    let statistic = r.db("rethinkdb")
        .table("server_status")
        .run::<ServerStatus>(conn);
    /*
    match statistic.unwrap().wait().next().unwrap() {
        // The server returned the response we were expecting
        Ok(Some(Document::Expected(status))) => {
            println!("{:?}", status);
        }
        Ok(Some(Document::Unexpected(status))) => {
            println!("unexpected response from server: {:?}", status);
        }
        Ok(None) => {
            println!("got no documents in the database");
        }
        // Oops! We ran into an error
        Err(error) => {
            println!("error: {}", error);
        }
    }
    */
    println!("Check database...");
    let table = r.db(DATABASE)
        .table_list()
        .run::<ServerStatus>(conn);

    let is_new = match table.unwrap().wait().next().unwrap() {
        // The server returned the response we were expecting
        Ok(Some(Document::Expected(status))) => {
            println!("{:?}", status);
            true
        }
        Ok(Some(Document::Unexpected(status))) => {
            println!("Done: {:?}", status);
            if status.is_array() {
                let index = status.as_array().unwrap().iter().position(|r| r == FILE_TABLE);
                match index {
                    Some(ok) => false,
                    None => true
                }
            } else {
                true
            }
        }
        Ok(None) => {
            println!("Error");
            exit(1);
            false
        }
        Err(error) => {
            println!("error: {}", error);
            exit(1);
            false
        }
    };
    if is_new {
        println!("Database test don`t have a tables");
        println!("Creating... ");
        //let arg = "{primaryKey: \"ИдФайл\"}";
        let mut arg = Arg::new();
        arg.add_arg(FILE_TABLE.into_arg());
        arg.set_string("{primary_key: ИдФайл}");
        let opt = Arg::create_term_pair("primary_key", "ИдФайл");
        match opt {
            Ok(pair) => arg.add_opt(pair),
            Err(err) => arg.set_term(Err(err)),
        }

        match r.db(DATABASE)
            .table_create(arg)
            .run::<ServerStatus>(conn).unwrap().wait().next().unwrap() {
            // The server returned the response we were expecting
            Ok(Some(Document::Expected(status))) => {
                println!("{:?}", status);
            }
            Ok(Some(Document::Unexpected(status))) => {
                println!("Done: {:?}", status);
            }
            Ok(None) => {
                println!("got no documents in the database");
            }
            // Oops! We ran into an error
            Err(error) => {
                println!("error: {:?}", error);
                exit(1)
            }
        }
        let mut arg = Arg::new();
        arg.add_arg(DOCS_TABLE.into_arg());
        arg.set_string("{durably: soft}");
        let opt = Arg::create_term_pair("durably", "soft");
        match opt {
            Ok(pair) => arg.add_opt(pair),
            Err(err) => arg.set_term(Err(err)),
        }
        match r.db(DATABASE)
            .table_create(arg)
            .run::<ServerStatus>(conn).unwrap().wait().next().unwrap() {
            // The server returned the response we were expecting
            Ok(Some(Document::Expected(status))) => {
                println!("{:?}", status);
            }
            Ok(Some(Document::Unexpected(status))) => {
                println!("Done: {:?}", status);
            }
            Ok(None) => {
                println!("got no documents in the database");
            }
            // Oops! We ran into an error
            Err(error) => {
                println!("error: {:?}", error);
                exit(1)
            }
        }
    } else {
        println!("Database test have a table test");
    }
    println!("Find and parse xml file to database");

    let options: MatchOptions = Default::default();
    let mut files: Vec<_> = vec![];
    match glob_with(PATCH, options) {
        Ok(r) => {
            println!("OK");
            files = r.filter_map(|x| x.ok()).collect();
        },
        Err(e) => {
            println!("Error found input directory");
            exit(1);
        },

    }
    if files.len() == 0 {
        println!("No *.xml files found in input derictory");
        exit(1);
    }
    let mut result:Vec<bool> = vec![];
    files.par_iter().map(|file| {
        parse_xml_file_info(file.display().to_string())
    }).collect_into_vec(&mut result);

    println!("Result parsing {:?}", result);

}