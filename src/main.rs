mod utils;

use utils::tokenizer;
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde::Deserialize;


#[derive(Deserialize, Debug)]
struct Jsonline {
    text: String
}

fn main() {
    let tokenizer = tokenizer::Tokenizer::new("data/rwkv_vocab_v20230424.txt").unwrap();
    let encode = tokenizer.encode("Hello world!");
    println!("{:?}", encode);
    let decode = tokenizer.decode(encode);
    println!("{:?}", decode);

    let mut dataset: Vec<String> = Vec::new();
    let file = File::open("data/wiki-en.jsonl").expect("couldn't open file");
    //let reader = BufReader::new(file);
    for line in BufReader::new(file).lines() {
        let line = line.expect("couldn't get line");
        let ds: Jsonline = serde_json::from_str(&line).unwrap();
        dataset.push(ds.text);
    }
    use std::time::Instant;
    let now = Instant::now();
    let mut counter = 0;
    // let mut index = 0;
    for text in dataset {
        let encode = tokenizer.encode(text.as_str());
        counter += encode.len();
        /*
        let decode = tokenizer.decode(encode.to_vec());
        if decode != text {
            println!("decode: {:?}", decode);
            println!("text: {:?}", text);
            break
        }
        assert_eq!(decode, text);
        index += 1;
        */
    }
    let elapsed = now.elapsed();
    // println!("Index: {:?}", index);
    println!("Counter: {:?}", counter);
    println!("Elapsed: {:.2?}", elapsed);
}
