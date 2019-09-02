extern crate encoding_rs;

use encoding_rs::WINDOWS_1251;

use std::fs;

extern crate xmlJSON;
extern crate rustc_serialize;

use xmlJSON::XmlDocument;
use rustc_serialize::json::{self, ToJson, Json};
use std::str::FromStr;


fn main() {
    let buff = fs::read("/home/konstantin/rust/xmltojson/VO_RUGFO_0000_9965_20180502_3ed8731d-8e50-4078-833b-b7ddc23b564b.xml").expect("Unable to read file");
    let (res, _enc, errors) = WINDOWS_1251.decode(&buff);
    if errors {
        eprintln!("Failed encode input file");
    } else {
        let document : XmlDocument = XmlDocument::from_str(&res.to_string()).unwrap();
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

    }


}