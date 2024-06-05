mod utils;
use utils::tokenizer;

fn main() {
    let tokenizer = tokenizer::Tokenizer::new("data/rwkv_vocab_v20230424.txt").unwrap();
    let encode = tokenizer.encode("Hello world!");
    println!("{:?}", encode);
    let decode = tokenizer.decode(encode);
    println!("{:?}", decode);
}
