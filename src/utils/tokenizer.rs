extern crate unescape;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use unescape::unescape;
use std::str;

#[derive(Default, Debug)]
struct TrieNode {
    children: [[Option<Box<TrieNode>>; 16]; 16],
    id: u16
}


impl TrieNode {
    fn new() -> Self {
        let mut trinode = TrieNode {
            children: Default::default(),
            id: 0
        };
        for index in 0..256 {
            trinode.children[index >> 4][index & 15] = None;
        }
        trinode
    }
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
}


impl Trie {
    pub(crate) fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub(crate) fn insert(&mut self, word: &Vec<u8>, id: u16) {
        let mut node = &mut self.root;
        for ch in word {
            let ch = u8::from_be(*ch) as usize;
            let index_a = ch >> 4;
            let index_b = ch & 15;
            if node.children[index_a][index_b].is_none() {
                node.children[index_a][index_b] = Option::from(Box::new(TrieNode::new()));
            }
            match &mut node.children[index_a][index_b] {
                Some(next_node) => node = next_node,
                None => unreachable!(),  // We've just checked that it's not None
            }
        }
        node.id = id
    }

    fn search_the_longest(&self, word: &[u8]) -> (u16, u16) {
        let mut node = &self.root;
        let mut old_node: &TrieNode = &self.root;
        let mut index = 0;
        let mut old_index = 0;
        for ch in word {
            let ch = u8::from_be(*ch) as usize;
            let index_a = ch >> 4;
            let index_b = ch & 15;
            if let Some(next_node) = &node.children[index_a][index_b]{
                if node.id != 0 {
                    old_node = node;
                    old_index = index;
                }
                node = &next_node;
                index += 1;
            } else {
                if node.id == 0 {
                    return (old_index, old_node.id);
                }
                else {
                    return (index, node.id);
                }
            }
        }
        if node.id == 0 {
            return (old_index, old_node.id);
        }
        else {
            return (index, node.id);
        }
    }

    pub(crate) fn tokenize(&self, text: &str) -> Vec<u16> {
        let mut vec: Vec<u16> = Vec::new();
        let text_length = text.len();
        let mut index: usize = 0;
        loop {
            let result = self.search_the_longest(&text.as_bytes()[index..]);
            if result.0 != 0 {
                vec.push(result.1.into());
                index += <u16 as Into<usize>>::into(result.0);
            } else {
                return vec;
            }
            if index >= text_length {
                return vec;
            }
        }
    }
}


#[derive(Debug)]
pub struct Tokenizer {
    tokens: Vec<Vec<u8>>,
    trie: Trie
}


impl Tokenizer {
    pub(crate) fn new(filename: &str) -> io::Result<Self> {
        let mut tokenizer = Tokenizer {
            tokens: Vec::new(),
            trie: Trie::new()
        };
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);

        let re = Regex::new(r"(\d+)\s+(b?)(.+)\s+(\d+)").unwrap();
        tokenizer.tokens.push(vec![0]);
        for line in reader.lines() {
            let line = line?;
            if let Some(captures) = re.captures(&line) {
                let id = captures[1].parse::<u16>().unwrap();
                let is_byte = captures[2].to_string();
                let length = captures[4].parse::<usize>().unwrap();
                let mut string: String = captures[3].to_string();
                string = string[1..string.len()-1].parse().unwrap();
                let sbytes: Vec<u8>;
                if is_byte.len() == 0 {
                    string = unescape(string.as_str()).unwrap();
                    sbytes = string.clone().into_bytes();
                    tokenizer.tokens.push(Vec::from(string.as_bytes()));
                } else {
                    sbytes = hex_to_bytes(string.as_str()).unwrap();
                    tokenizer.tokens.push(sbytes.clone());
                }
                assert_eq!(sbytes.len(), length);
                tokenizer.trie.insert(&sbytes, id);
            }
            else {
                println!("Line with issue: {:?}", line)
            }
        }
        Ok(tokenizer)
    }

    pub(crate) fn encode(&self, word: &str) -> Vec<u16> {
        self.trie.tokenize(word)
    }

    #[allow(dead_code)]
    pub(crate) fn decode(&self, vec: Vec<u16>) -> String {
        let mut result: Vec<u8> = Vec::new();
        for index in vec.iter() {
            let mut current_tokens = self.tokens[*index as usize].clone();
            result.append(&mut current_tokens);
        }
        return str::from_utf8(&*result).unwrap().to_string();
    }
}

fn hex_to_bytes(hex: &str) -> Option<Vec<u8>> {
    let hex = hex.replace("\\x", "");
    if hex.len() % 2 == 0 {
        (0..hex.len())
            .step_by(2)
            .map(|i| hex.get(i..i + 2)
                .and_then(|sub| u8::from_str_radix(sub, 16).ok()))
            .collect()
    } else {
        None
    }
}
