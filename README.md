# RWKV Tokenizer Benchmark

This is just a simple benchmark to measure the encoding performance of [The Rust RWKV Tokenizer](https://github.com/cahya-wirawan/rwkv-tokenizer).
It will read the simple English Wikipedia dataset in jsonl format. The file is 213MB, and it should be stored as "data/wiki-en.jsonl".
## Usage

```
$ cargo run --release
```
It will print out the number of encoded tokens and the elapsed time for encoding. The number of tokens should be exactly 53619552 for this dataset.
The M2 mac mini took 1.87s to encode the dataset.