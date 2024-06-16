use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Duration;
use std::time::Instant;
use serde::Deserialize;
use rwkv_tokenizer::WorldTokenizer;
use web_rwkv::tokenizer::{Tokenizer, TokenizerError};

#[derive(Deserialize, Debug)]
struct Jsonline {
    text: String
}


fn load_tokenizer(path: impl AsRef<Path>) -> Result<Tokenizer, TokenizerError> {
    let contents = fs::read_to_string(path).
        expect("Should have been able to read the file");
    Ok(Tokenizer::new(&contents)?)
}

fn main() {
    let tokenizer = WorldTokenizer::new("data/rwkv_vocab_v20230424.txt").unwrap();
    let file = File::open("data/wiki-en.jsonl").expect("couldn't open file");
    let mut counter = 0;
    let mut elapsed: Duration = Duration::new(0, 0);
    for line in BufReader::new(file).lines() {
        let line = line.expect("couldn't get line");
        let ds: Jsonline = serde_json::from_str(&line).unwrap();
        let now = Instant::now();
        let encode = tokenizer.encode(ds.text.as_str());
        elapsed += now.elapsed();
        counter += encode.len();
    }
    println!("Number of tokens: {:?}", counter);
    println!("Elapsed time rwkv_tokenizer: {:.2?}", elapsed);

    let vocabfile = "/Users/cahya/Work/MachineLearning/web-rwkv/assets/rwkv_vocab_v20230424.json";
    let tokenizer_web = load_tokenizer(vocabfile).unwrap();
    let file = File::open("data/wiki-en.jsonl").expect("couldn't open file");
    let mut counter = 0;
    let mut elapsed: Duration = Duration::new(0, 0);
    for line in BufReader::new(file).lines() {
        let line = line.expect("couldn't get line");
        let ds: Jsonline = serde_json::from_str(&line).unwrap();
        let now = Instant::now();
        let encode = tokenizer_web.encode(ds.text.as_ref()).unwrap();
        elapsed += now.elapsed();
        counter += encode.len();
    }
    println!("Number of tokens: {:?}", counter);
    println!("Elapsed time web_rwkv: {:.2?}", elapsed);
}
