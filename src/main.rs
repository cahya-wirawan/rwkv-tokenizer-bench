mod utils;

use utils::tokenizer;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
struct Jsonline {
    text: String
}

fn main() {
    let tokenizer = tokenizer::Tokenizer::new("data/rwkv_vocab_v20230424.txt").unwrap();
    let file = File::open("data/wiki-en.jsonl").expect("couldn't open file");
    use std::time::Instant;
    let mut counter = 0;
    let mut elapsed: Duration = Duration::new(0, 0);
    for line in BufReader::new(file).lines() {
        let line = line.expect("couldn't get line");
        let ds: Jsonline = serde_json::from_str(&line).unwrap();
        let now = Instant::now();
        let encode = tokenizer.encode(ds.text.as_str());
        elapsed += now.elapsed();
        counter += encode.len();
        /*
        let decode = tokenizer.decode(encode.to_vec());
        if decode != ds.text {
            println!("decode: {:?}", decode);
            println!("text: {:?}", ds.text);
            break
        }
        assert_eq!(decode, ds.text);
        index += 1;
        */
    }
    // println!("Index: {:?}", index);
    println!("Number of tokens: {:?}", counter);
    println!("Elapsed time: {:.2?}", elapsed);
}
