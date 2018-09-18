extern crate ethereum_types;
extern crate rustc_hex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use ethereum_types::{Address, U256};
use rustc_hex::FromHex;
use std::fs;
use std::prelude::v1::Vec;

#[derive(Serialize, Deserialize, Debug)]
struct Test {
    name: String,
    pipeline: Vec<Stage>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stage {
    #[serde(rename = "type")]
    transaction_type: String,
    from: String,
    to: String,
    value: String,
    data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    raw: Option<String>,
    code: Option<String>,
    method: Option<String>,
    arguments: Option<String>,
}

fn main() {
    // read the json file
    let json = fs::read_to_string("ack/fastvm/basic/testTransfer.json").unwrap();

    // deserialize the JSON string
    let deserialized: Vec<Test> = serde_json::from_str(&json).unwrap();

    for test in deserialized.iter() {
        println!("Testing {}", test.name);

        for stage in test.pipeline.iter() {
            let from = parse_address(&stage.from);
            let to = parse_address(&stage.to);
            println!("  from = {:?}", from);
            println!("  to = {:?}", from);
        }
    }
}

fn parse_address(addr: &String) -> Address {
    match addr.as_ref() {
        "ADDRESS_PREMINED" => Address::default(),
        "ADDRESS_LAST_DEPLOYED" => Address::default(),
        "ADDRESS_RANDOM" => Address::default(),
        _ => {
            let bytes = addr.from_hex::<Vec<u8>>().unwrap();
            Address::from_slice(bytes.as_ref())
        }
    }
}

fn parse_value(value: &String) -> U256 {
    if value.starts_with("0x") {
        let sub_str = value.chars().skip(2).collect::<String>();
        U256::from_dec_str(sub_str.as_ref()).unwrap()
    } else {
        U256::from(value.parse::<i32>().unwrap())
    }
}