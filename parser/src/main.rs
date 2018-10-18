extern crate blake2b;
extern crate crypto;
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
use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;
use std::env;
use std::fs::{self, File};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::prelude::v1::Vec;
use std::process::Command;
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
    if args.len() != 5 {
        println!("Usage: ./ack-parser [PRIVATE_KEY] [NONCE] [SRC_FOLDER] [DEST_FILE]");
        std::process::exit(1);
    }
    let private_key = parse_hex(&args[1]);
    let mut nonce = args[2].parse::<i32>().unwrap();

    // output file
    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(args[4].as_str())
        .unwrap();

    // parse all the JSON files
    let dir: &Path = Path::new(args[3].as_str());
    let files = list_file(dir, ".json");
    for file in files {
        println!("================================================");
        println!("{}", file);
        println!("================================================");
        nonce = process_file(file.as_str(), &private_key, nonce, &mut output);
    }
}

/// Walk through the given path and return a list of files with the specified
/// extension.
///
fn list_file(path: &Path, ext: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            let mut temp = list_file(&path, ext);
            result.append(&mut temp);
        }
    } else {
        let path_str = String::from(path.to_str().unwrap());
        if path_str.ends_with(ext) {
            result.push(path_str);
        }
    }

    return result;
}

/// Process a integration test file and generate corresponding transactions for
/// future test. This method returns the afterward nonce of the account.
///
fn process_file(path: &str, private_key: &Vec<u8>, nonce: i32, output: &mut File) -> i32 {
    // read the json file
    let json = fs::read_to_string(path).unwrap();

    // deserialize the JSON string
    let deserialized: Vec<Test> = serde_json::from_str(&json).unwrap();

    let mut new_nonce = nonce;
    let mut last_deployed = Address::default();

    for test in deserialized.iter() {
        println!("name: {}", test.name);
        println!("description: {}", test.description);

        for t in test.transactions.iter() {
            let _type = &t._type;
            let receiver = parse_address(&t.receiver, &last_deployed);
            let value = parse_value(&t.value.clone().unwrap_or_else(|| String::from("0")));
            let data = assemble_data(
                &t.data.raw,
                &t.data.code,
                &t.data.method,
                &t.data.arguments,
                path,
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
                U256::from(new_nonce),
                nrg_price,
                nrg,
                match _type.as_ref() {
                    "CREATE" => Action::Create,
                    "CALL" => Action::Call(receiver),
                    _ => panic!("Unexpected transaction type: {}", _type)
                },
                value,
                data,
            ).sign(private_key.as_slice(), None);
            println!("{:?}\n", tx);

            // rlp encoding
            let encoded: Vec<u8> = tx.rlp_bytes().into_vec();
            let hex = to_hex_string(encoded);
            output.write(hex.as_ref()).unwrap();
            output.write(b"\n").unwrap();

            // update last_deployed if CREATE
            if _type == "CREATE" {
                let mut stream = RlpStream::new_list(2);
                stream.append(&tx.sender());
                stream.append(&U256::from(new_nonce));
                let origin: [u8; 32] = blake2b(stream.as_raw()).into();
                let mut buffer = [0xa0u8; 32];
                &mut buffer[1..].copy_from_slice(&origin[1..]);
                last_deployed = buffer.into();
            }

            // increase nonce
            new_nonce = new_nonce + 1;
        }
    }

    return new_nonce;
}

/// Parse an AION address from string. It can with a hex string with/out the 0x prefix.
///
/// Pre-defined variables are also supported:
/// - `ADDRESS_LAST_DEPLOYED`
/// - `ADDRESS_RANDOM`
///
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

/// Parse an unsigned 256-bit integer from a string. Note that it should be 128-bit integers
/// to comply with FastVM's specification.
///
fn parse_value(value: &String) -> U256 {
    if value.starts_with("0x") {
        let sub_str = value.chars().skip(2).collect::<String>();
        return U256::from_dec_str(sub_str.as_ref()).unwrap();
    } else {
        return U256::from(value.parse::<i32>().unwrap());
    }
}

/// Assemble the raw data of transaction, using the following formula:
///
/// `data = raw + compile(code) + bytes4(keccak256(method)) + arguments`
///
fn assemble_data(raw: &Option<String>, code: &Option<String>, method: &Option<String>, arguments: &Option<String>, path: &str)
                 -> Vec<u8> {
    let mut assmebled: Vec<u8> = Vec::new();

    assmebled.append(&mut parse_hex(&raw.clone().unwrap_or_default()));
    if code.is_some() {
        let code = code.clone().unwrap_or_default();
        let mut file = String::new();
        if code.starts_with("@") {
            let idx = path.rfind("/").unwrap();
            file.push_str(&path[0..idx + 1]);
            file.push_str(&code[1..code.len()]);
        } else {
            let temp_file = "/tmp/contract.sol";
            file.push_str(temp_file);

            let mut buffer = File::create(temp_file).unwrap();
            buffer.write(code.as_bytes()).unwrap();
        }
        assmebled.append(&mut compile_code(&file));
    }
    if method.is_some() {
        let mut hash = [0u8; 32];
        let mut hasher = Sha3::keccak256();
        hasher.input_str(method.clone().unwrap_or_default().as_str());
        hasher.result(&mut hash);

        assmebled.append(&mut hash[0..4].to_vec());
    }
    assmebled.append(&mut parse_hex(&arguments.clone().unwrap_or_default()));

    return assmebled;
}

/// Compile a solidity source code
fn compile_code(file: &String) -> Vec<u8> {
    let solc_home = env::var("SOLC_HOME").unwrap_or_default();
    let output = Command::new("solc")
        .env("LD_LIBRARY_PATH", solc_home)
        .arg("--combined-json")
        .arg("bin")
        .arg(file)
        .output()
        .expect("solc failed to start");

    if output.status.code().unwrap() == 0 {
        let bin = String::from_utf8_lossy(&output.stdout);
        let start = bin.find("\"bin\":\"").unwrap();
        let end = bin[start + 7..].find("\"").unwrap();
        let b = &bin[start + 7..start + 7 + end];
        return b.from_hex::<Vec<u8>>().unwrap();
    }

    panic!("Failed to compile: {}", file);
}

/// Parse an hex string into byte array. The input string can be with/out the `0x` prefix
///
fn parse_hex(hex: &String) -> Vec<u8> {
    if hex.starts_with("0x") {
        let sub_str = hex.chars().skip(2).collect::<String>();
        return sub_str.from_hex::<Vec<u8>>().unwrap();
    } else {
        return hex.from_hex::<Vec<u8>>().unwrap();
    }
}

/// Generate a random address.
///
fn random_address() -> Address {
    let mut bytes = [0u8; 32];

    for i in bytes.iter_mut() {
        *i = rand::random::<u8>();
    }

    return Address::from_slice(&bytes);
}

/// Convert an u8 vector into it's hex representation.
///
pub fn to_hex_string(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    return strs.join("");
}