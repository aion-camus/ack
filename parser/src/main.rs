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
    description: String,
    pipeline: Vec<Stage>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stage {
    transaction: Transaction,
    result: Result
}

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    #[serde(rename = "type")]
    _type: String,
    receiver: String,
    value: Option<String>,
    data: TransactionData,
    nrg: Option<String>,
    #[serde(rename = "nrgPrice")]
    nrg_price: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TransactionData {
    raw: Option<String>,
    code: Option<String>,
    method: Option<String>,
    arguments: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Result {
    status: String,
    #[serde(rename = "returnData")]
    return_data: Option<String>,
}

fn main() {
    // read the json file
    let json = fs::read_to_string("ack/fastvm/basic/testTransfer.json").unwrap();

    // deserialize the JSON string
    let deserialized: Vec<Test> = serde_json::from_str(&json).unwrap();

    for test in deserialized.iter() {
        println!("\n\nname: {}", test.name);
        println!("description: {}", test.description);

        for stage in test.pipeline.iter() {
            let tx = &stage.transaction;
            let res = &stage.result;

            let _type = &tx._type;
            let receiver = parse_address(&tx.receiver);
            let value = parse_value(&tx.value.clone().unwrap_or_else(|| String::from("0")));
            let data = assemble_data(
                &tx.data.raw,
                &tx.data.code,
                &tx.data.method,
                &tx.data.arguments
            );
            let nrg = parse_value(&tx.nrg.clone().unwrap_or_else(|| String::from("1000000")));
            let nrg_price = parse_value(&tx.nrg_price.clone().unwrap_or_else(|| String::from("1")));

            println!("{{\n  type: {:?}", _type);
            println!("  receiver: {:?}", receiver);
            println!("  value: {:?}", value);
            println!("  data: {:?}", data);
            println!("  nrg: {:?}", nrg);
            println!("  nrgPrice: {:?}\n}}", nrg_price);

            let status = &res.status;
            let return_data = parse_hex(&res.return_data.clone().unwrap_or_default());
            println!("{{\n  status: {:?}", status);
            println!("  returnData: {:?}\n}}", return_data);
        }
    }
}

fn parse_address(address: &String) -> Address {
    match address.as_ref() {
        "${ADDRESS_LAST_DEPLOYED}" => Address::default(),
        "${ADDRESS_RANDOM}" => Address::default(),
        _ => {
            let bytes = parse_hex(address);
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

fn assemble_data(raw: &Option<String>, code: &Option<String>, method: &Option<String>, arguments: &Option<String>)
    -> Vec<u8> {
    let mut assmebled: Vec<u8> = Vec::new();

    // concatenate: raw + code + method + arguments
    if raw.is_some() {
        assmebled.append(&mut parse_hex(&raw.clone().unwrap()));
    }
    if code.is_some() {
        assmebled.append(&mut parse_hex(&code.clone().unwrap()));
    }
    if method.is_some() {
        assmebled.append(&mut parse_hex(&method.clone().unwrap()));
    }
    if arguments.is_some() {
        assmebled.append(&mut parse_hex(&arguments.clone().unwrap()));
    }

    assmebled
}

fn parse_hex(hex: &String) -> Vec<u8> {
    if hex.starts_with("0x") {
        let sub_str = hex.chars().skip(2).collect::<String>();
        sub_str.from_hex::<Vec<u8>>().unwrap()
    } else {
        hex.from_hex::<Vec<u8>>().unwrap()
    }
}