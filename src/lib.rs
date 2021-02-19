//! A [BullshitGenerator](https://github.com/menzi11/BullshitGenerator) implementation in Rust.  
//! ## Example:  
//! ```
//! let s = BullshitGenerator::new().generate("sth", 1000);
//! ```  
//! ..or with first line indents:  
//! ```
//! let s = BullshitGenerator::with_indent(2).generate("sth", 1000);
//! ```  

use json::JsonValue;
use rand::{seq::IteratorRandom, thread_rng, Rng};

pub struct BullshitGenerator {
    data: JsonValue,
    indent: usize,
    text: String,
    total_length: usize,
    paragraph_length: usize,
}
impl BullshitGenerator {
    /// Create a new instance of `BullshitGenerator`.  
    pub fn new() -> Self {
        Self::with_indent(0)
    }
    /// Create a new instance of `BullshitGenerator`, with first line indent full-width spaces.  
    pub fn with_indent(indent: usize) -> Self {
        BullshitGenerator {
            data: json::parse(include_str!("../data.json")).unwrap(),
            indent,
            text: String::new(),
            total_length: 0,
            paragraph_length: 0,
        }
    }
    /// Generate an article by given topic and length.  
    /// **Note: `len` is an approximate value and it does not equal to the actual length of the result.**  
    pub fn generate(mut self, topic: &str, len: usize) -> String {
        self.text = String::with_capacity(len * 4);
        self.text += &*"　".repeat(self.indent);
        while self.total_length < len || self.paragraph_length < 200 {
            match thread_rng().gen_range(0..100) {
                0..=5 if self.paragraph_length > 200 => self.new_line(),
                0..=20 => self.append_text(&*self.get_famous()),
                _ => self.append_text(&*self.get_bullshit(topic)),
            }
        }
        self.new_line();
        self.text.trim_end().into()
    }
    /// Generate a single `famous` sentence.  
    pub fn get_famous(&self) -> String {
        self.rand_str("famous")
            .to_string()
            .replace("a", self.rand_str("before"))
            .replace("b", self.rand_str("after"))
    }
    /// Generate a single `bullshit` sentence.  
    pub fn get_bullshit(&self, topic: &str) -> String {
        self.rand_str("bosh").replace("x", topic)
    }
    fn rand_str(&self, k: &str) -> &str {
        self.data[k]
            .members()
            .choose(&mut thread_rng())
            .unwrap()
            .as_str()
            .unwrap()
    }
    fn append_text(&mut self, str: &str) {
        let len = str.chars().count();
        self.text += str;
        self.total_length += len;
        self.paragraph_length += len;
    }
    fn new_line(&mut self) {
        self.text.pop();
        self.text += "。\n";
        self.text += &*"　".repeat(self.indent);
        self.paragraph_length = 0;
    }
}

#[test]
fn test_generate() {
    println!(
        "{}",
        BullshitGenerator::with_indent(2).generate("黄油", u16::MAX as usize)
    );
}

#[test]
fn test_single_bullshit() {
    println!("{}", BullshitGenerator::with_indent(2).get_bullshit("黄油"));
}

#[test]
fn test_single_famous() {
    println!("{}", BullshitGenerator::with_indent(2).get_famous());
}
