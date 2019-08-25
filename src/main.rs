#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use std::fs::File;
use std::io::Read;

use serde_xml_rs::from_reader;

extern crate xmlJSON;
extern crate rustc_serialize;

use xmlJSON::XmlDocument;
use rustc_serialize::json;
use std::str::FromStr;
use crate::rustc_serialize::json::ToJson;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope {
    Header: Header,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Header {
    // #[serde(rename(serialize = "Header", deserialize = "Header"))]

    pub Header: String,
    Security: Security,
    //pub ВерсФорм: String,
    //pub ИдДок: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Security {
    pub actor: String,
    BinarySecurityToken: BinarySecurityToken,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BinarySecurityToken {
    ValueType: String,
    EncodingType: String,
    Id: String,
    #[serde(rename="$value")]
    content: String,

}


fn main() {
    let mut file = File::open("/home/konstantin/rust/xmltojson/result.xml").unwrap();
    println!("{:#?}", file);
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    //let level_data: Envelope = serde_xml_rs::from_str(&buff).unwrap();
    let document : XmlDocument = XmlDocument::from_str(&buff).unwrap();
    let level_data : json::Json = document.to_json();
    println!("{:#?}", level_data);
}