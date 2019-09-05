extern crate encoding_rs;
extern crate xmlJSON;
extern crate rustc_serialize;
extern crate glob;
extern crate rayon;
extern crate reql;
extern crate reql_types;
extern crate futures;



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
use reql_types::ServerStatus;
use futures::{Stream, Join};


const PATCH: &str = "/Users/konstantin/rust/xmltojson/*.xml";


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
        println!("Save json : {}", body.to_json().to_string());
        /*
        let statistic = r.db("rethinkdb")
            .table("server_status")
            .run::<ServerStatus>(conn);

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

        let data = r#"{name: "Meri", age: 45, phones: ["1213123", "12ee1231"]}"#;
        let mut arg = Arg::new();
        arg.set_string(data);

        let w = r.db("test")
            .table("test")
            .insert(arg)
            .run::<ServerStatus>(conn).unwrap().wait().next().unwrap();
        match w {
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
                println!("error: {:?}", error);
            }
        }


        /*

        for (key, value) in file.iter() {
            println!("{}: {}", key, match *value {
                Json::U64(v) => format!("{} (u64)", v),
                Json::String(ref v) => format!("{} (string)", v),
                _ => format!("other")
            });
        }

        let docs = file.get("Документ").unwrap();
        // println!("object? docs {}", docs.is_array());
        if docs.is_array() {
            match docs.as_array() {
                Some(docs) => {
                    for item in docs {
                        /*
                        for (key, value) in item.as_object().unwrap() {
                            println!("{}: {}", key, match *value {
                                Json::U64(v) => format!("{} (u64)", v),
                                Json::String(ref v) => format!("{} (string)", v),
                                _ => format!("other")
                            });
                        }
                        */
                        let body = item.as_object().unwrap().get("$").unwrap().as_object().unwrap();
                        println!("{:#?}", body);
                        let swyl = item.as_object().unwrap().get("СвЮЛ").unwrap().as_object().unwrap();
                        // println!("{:#?}", swyl);
                        break;
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
        */
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
    let table = r.db("test")
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
                let index = status.as_array().unwrap().iter().position(|r| r == "test");
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
        println!("Database test don`t have a table test");
        println!("Creating... ");
        match r.db("test")
            .table_create("test")
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
                println!("error: {}", error);
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