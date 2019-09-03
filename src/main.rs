extern crate encoding_rs;
extern crate xmlJSON;
extern crate rustc_serialize;
extern crate glob;
extern crate rayon;

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

const PATCH: &str = "/Users/konstantin/rust/xmltojson/*.xml";


fn parse_xml_file_info(file: String) -> bool {


    let buff = fs::read(file).expect("Unable to read file");
    let (res, _enc, errors) = WINDOWS_1251.decode(&buff);
    let test2 = res.to_string();
    //println!("{}", test2);

    let test = "<note type=\"Reminder\">
                test
            </note>";

    let data = XmlDocument::from_str(&test2).unwrap();
    if errors {
        // println!("Failed encode input file");
    false
    } else {

        //let data = XmlDocument::from_str(&test).unwrap();

        //let document : XmlDocument = XmlDocument::from_str(&res.to_string()).unwrap();
        /*
        let json_data : json::Json = document.to_json();
        //let json_str: String = json_data.to_string();
        //fs::write("/home/konstantin/rust/xmltojson/output.json", json_str).expect("Unable to write output file");
        // println!("object? {}", json_data.is_object());
        let json_obj = json_data.as_object().unwrap();

        let file = json_obj.get("Файл").unwrap().as_object().unwrap();

        for (key, value) in file.iter() {
            println!("{}: {}", key, match *value {
                Json::U64(v) => format!("{} (u64)", v),
                Json::String(ref v) => format!("{} (string)", v),
                _ => format!("other")
            });
        }
        let docs = file.get("Документ").unwrap();
        // println!("object? docs {}", docs.is_array());

        for item in docs.as_array().unwrap() {
            for (key, value) in item.as_object().unwrap() {
                println!("{}: {}", key, match *value {
                    Json::U64(v) => format!("{} (u64)", v),
                    Json::String(ref v) => format!("{} (string)", v),
                    _ => format!("other")
                });
            }
            let body = item.as_object().unwrap().get("$").unwrap().as_object().unwrap();
            println!("{:#?}", body);
            let swyl = item.as_object().unwrap().get("СвЮЛ").unwrap().as_object().unwrap();
            println!("{:#?}", swyl);
            break;
        }
    */
    true
    }

}


fn main() {
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