extern crate blake2b;
extern crate ethcore_transaction as transaction;
extern crate ethereum_types;
extern crate rand;
extern crate rlp;
extern crate rustc_hex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use blake2b::blake2b;
use ethereum_types::{Address, U256};
use rlp::{Encodable, RlpStream};
use rustc_hex::FromHex;
use std::env;
use std::fs;
use std::prelude::v1::Vec;
use transaction::{Action, Transaction};

#[derive(Serialize, Deserialize, Debug)]
struct Test {
    name: String,
    description: String,
    transactions: Vec<TestTransaction>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TestTransaction {
    #[serde(rename = "type")]
    _type: String,
    receiver: String,
    value: Option<String>,
    data: TestTransactionData,
    nrg: Option<String>,
    #[serde(rename = "nrgPrice")]
    nrg_price: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TestTransactionData {
    raw: Option<String>,
    code: Option<String>,
    method: Option<String>,
    arguments: Option<String>,
}

fn main() {
    // read private_key and nonce from arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: ./ack-parser [PRIVATE_KEY] [NONCE]");
        std::process::exit(1);
    }
    let pk = parse_hex(&args[1]);
    let mut nonce = args[2].parse::<i32>().unwrap();
    let mut last_deployed;

    // read the json file
    let json = fs::read_to_string("ack/fastvm/basic/testTransfer.json").unwrap();

    // deserialize the JSON string
    let deserialized: Vec<Test> = serde_json::from_str(&json).unwrap();

    for test in deserialized.iter() {
        println!("\n\nname: {}", test.name);
        println!("description: {}", test.description);

        // reset last_deployed
        last_deployed = Address::default();

        for t in test.transactions.iter() {
            let _type = &t._type;
            let receiver = parse_address(&t.receiver, &last_deployed);
            let value = parse_value(&t.value.clone().unwrap_or_else(|| String::from("0")));
            let data = assemble_data(
                &t.data.raw,
                &t.data.code,
                &t.data.method,
                &t.data.arguments,
            );
            let nrg = parse_value(&t.nrg.clone().unwrap_or_else(|| String::from("1000000")));
            let nrg_price = parse_value(&t.nrg_price.clone().unwrap_or_else(|| String::from("1")));

            println!("{{\n  type: {:?}", _type);
            println!("  receiver: {:?}", receiver);
            println!("  value: {:?}", value);
            println!("  data: {:?}", data);
            println!("  nrg: {:?}", nrg);
            println!("  nrgPrice: {:?}\n}}", nrg_price);

            // assemble the transaction
            let tx = Transaction::new(
                U256::from(nonce),
                nrg_price,
                nrg,
                match _type.as_ref() {
                    "CREATE" => Action::Create,
                    "CALL" => Action::Call(receiver),
                    _ => panic!("Unexpected transaction type: {}", _type)
                },
                value,
                data,
            ).sign(pk.as_slice(), None);
            ;
            println!("Assembled transaction: {:?}", tx);

            // rlp encoding
            let encoded: Vec<u8> = tx.rlp_bytes().into_vec();
            let hex = to_hex_string(encoded);
            println!("{}", hex);

            // update last_deployed if CREATE
            if _type == "CREATE" {
                let mut stream = RlpStream::new_list(2);
                stream.append(&tx.sender());
                stream.append(&U256::from(nonce));
                let origin: [u8; 32] = blake2b(stream.as_raw()).into();
                let mut buffer = [0xa0u8; 32];
                &mut buffer[1..].copy_from_slice(&origin[1..]);
                last_deployed = buffer.into();
            }

            // increase nonce
            nonce = nonce + 1;
        }
    }
}

fn parse_address(address: &String, last_deployed: &Address) -> Address {
    match address.as_ref() {
        "${ADDRESS_LAST_DEPLOYED}" => last_deployed.clone(),
        "${ADDRESS_RANDOM}" => random_address(),
        _ => {
            let bytes = parse_hex(address);
            return Address::from_slice(bytes.as_ref());
        }
    }
}

fn parse_value(value: &String) -> U256 {
    if value.starts_with("0x") {
        let sub_str = value.chars().skip(2).collect::<String>();
        return U256::from_dec_str(sub_str.as_ref()).unwrap();
    } else {
        return U256::from(value.parse::<i32>().unwrap());
    }
}

fn assemble_data(raw: &Option<String>, code: &Option<String>, method: &Option<String>, arguments: &Option<String>)
                 -> Vec<u8> {
    let mut assmebled: Vec<u8> = Vec::new();

    // concatenate: raw + code + method + arguments
    assmebled.append(&mut parse_hex(&raw.clone().unwrap_or_default()));
    assmebled.append(&mut parse_hex(&code.clone().unwrap_or_default()));
    assmebled.append(&mut parse_hex(&method.clone().unwrap_or_default()));
    assmebled.append(&mut parse_hex(&arguments.clone().unwrap_or_default()));

    return assmebled;
}

fn parse_hex(hex: &String) -> Vec<u8> {
    if hex.starts_with("0x") {
        let sub_str = hex.chars().skip(2).collect::<String>();
        return sub_str.from_hex::<Vec<u8>>().unwrap();
    } else {
        return hex.from_hex::<Vec<u8>>().unwrap();
    }
}

fn random_address() -> Address {
    let mut bytes = [0u8; 32];

    for i in bytes.iter_mut() {
        *i = rand::random::<u8>();
    }

    return Address::from_slice(&bytes);
}

pub fn to_hex_string(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    return strs.join("");
}