use anyhow::Context;
use anyhow::Result;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

fn is_levidrome(dict: &BTreeSet<String>, s: &str) -> bool {
    let reverse: String = s.chars().rev().collect();
    dict.contains(&reverse)
}

fn main() -> Result<()> {
    let mut path = String::from("./words.txt");
    for (idx, argument) in env::args().enumerate() {
        println!("{} {}", idx, argument);
        match idx {
            1 => path = argument,
            _ => {}
        };
    }
    let mut dict = BTreeSet::new();
    let file = fs::File::open(&path).with_context(|| format!("while opening path={}", &path))?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(word) = line {
            dict.insert(word);
        }
    }
    let levidromes: Vec<String> = dict
        .iter()
        .filter(|&word| is_levidrome(&dict, word))
        .cloned()
        .collect();
    for l in levidromes {
      println!("{}", l);
    }
    Ok(())
}
