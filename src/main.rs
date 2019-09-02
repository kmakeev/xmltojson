extern crate encoding_rs;

use encoding_rs::WINDOWS_1251;

use std::fs;

extern crate xmlJSON;
extern crate rustc_serialize;

use xmlJSON::XmlDocument;
use rustc_serialize::json;
use std::str::FromStr;
use crate::rustc_serialize::json::ToJson;


fn main() {
    let buff = fs::read("/home/konstantin/rust/xmltojson/VO_RUGFO_0000_9965_20190701_0fbed829-84be-4892-94ae-74242909726a.xml").expect("Unable to read file");
    let (res, _enc, errors) = WINDOWS_1251.decode(&buff);
    if errors {
        eprintln!("Failed encode input file");
    } else {
        let document : XmlDocument = XmlDocument::from_str(&res.to_string()).unwrap();
        let json_data : json::Json = document.to_json();
        let json_str: String = json_data.to_string();
        fs::write("/home/konstantin/rust/xmltojson/output.json", json_str).expect("Unable to write output file");
    }


}