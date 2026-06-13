//! A small sample file to open with lux for trying out syntax highlighting and
//! rust-analyzer diagnostics/completion:
//!
//!     cargo run --release -- samples/demo.rs

use std::collections::HashMap;

/// Count how many times each word appears in `text`.
fn word_counts(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let text = "the quick brown fox the lazy dog THE end";
    let counts = word_counts(text);

    let mut pairs: Vec<(&String, &usize)> = counts.iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));

    for (word, count) in pairs {
        println!("{count:>3}  {word}");
    }
}
