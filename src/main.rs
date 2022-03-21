use anyhow::{Context, Result};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

mod phoneme;

use phoneme::{Phoneme, Word};

fn make_dict<P: AsRef<Path>>(path: P) -> Result<BTreeSet<Word>> {
    let mut dict = BTreeSet::new();
    let file = fs::File::open(&path)
        .with_context(|| format!("while opening path={}", path.as_ref().to_string_lossy()))?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(spelling) = line {
            let word = Word{
                spelling: spelling.clone(),
                phonemes: spelling.clone().chars().map(|c| {Phoneme{symbol: String::from(c)}}).collect(),
            };

            dict.insert(word);
        }
    }
    Ok(dict)
}

fn find_mirrors(dict: &BTreeSet<Word>, length: usize) -> BTreeSet<Vec<Word>> {
    let mut results = BTreeSet::new();
    find_mirrors_with(dict, length, &Vec::new(), &mut results);
    results
}

fn find_mirrors_with(
    dict: &BTreeSet<Word>,
    length: usize,
    history: &Vec<Word>,
    result: &mut BTreeSet<Vec<Word>>,
) {
    for word in dict {
        if word.phonemes.len() != length {
            continue;
        }
        let mut mirror = history.clone();
        mirror.push(word.clone());
        let state = mirror_state(&mirror);
        match state {
            MirrorState::Complete => {
                result.insert(mirror);
            }
            MirrorState::Invalid => {}
            MirrorState::Partial => find_mirrors_with(dict, length, &mirror, result),
        };
    }
}

#[derive(PartialEq, Debug)]
enum MirrorState {
    Invalid,
    Complete,
    Partial,
}

/*
p o w
o
w
*/
fn mirror_state(mirror: &Vec<Word>) -> MirrorState {
    if mirror.len() == 0 {
        return MirrorState::Partial;
    }
    let len = mirror[0].phonemes.len();
    if mirror.last().unwrap().phonemes.len() != len {
        return MirrorState::Invalid;
    }
    // check that words are levidromes of their counter once selected
    for i in 0..(len / 2) {
        let j = len - 1 - i;
        match (mirror.get(i), mirror.get(j)) {
            (Some(iword), Some(jword)) => {
                let revj: Vec<Phoneme> = jword.phonemes.iter().cloned().rev().collect();
                if *iword.phonemes != revj {
                    return MirrorState::Invalid;
                }
            }
            _ => {}
        }
    }
    for i in 0..len {
        for j in 0..len {
            if i == j {
                continue;
            }
            let iword = mirror.get(i);
            let jword = mirror.get(j);
            match (iword, jword) {
                (Some(iword), Some(jword)) => {
                    let ij = iword.phonemes.iter().nth(j).unwrap();
                    let ji = jword.phonemes.iter().nth(i).unwrap();
                    if ij != ji {
                        return MirrorState::Invalid;
                    }
                }
                _ => {}
            }
        }
    }
    if mirror.len() == len {
        MirrorState::Complete
    } else {
        MirrorState::Partial
    }
}

fn main() -> Result<()> {
    let mut path = String::from("./levidromes.txt");
    let mut length = 5;
    for (idx, argument) in env::args().enumerate() {
        println!("{} {}", idx, argument);
        match idx {
            1 => path = argument,
            2 => length = argument.parse::<usize>()?,
            _ => {}
        };
    }
    let dict = make_dict(&path)?;
    let mirrors = find_mirrors(&dict, length);
    for mirror in mirrors {
        for (idx, word) in mirror.iter().enumerate() {
            if idx != 0 {
                print!(" ");
            }
            print!("{:?}", word);
        }
        println!("");
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{mirror_state, MirrorState, phoneme::{Word, Phoneme}};

    #[test]
    fn test_mirror_state() {
        fn to_word(s: &str) -> Word {
            Word{
                spelling: String::from(s),
                phonemes: s.chars().map(|c| { Phoneme{symbol: String::from(c)}}).collect(),
            }
        }

        let mirror: Vec<Word> = vec!["sator"].iter().map(|&s| to_word(s)).collect();
        assert_eq!(MirrorState::Partial, mirror_state(&mirror));

        let mirror: Vec<Word> = vec!["cow", "win"].iter().map(|&s| to_word(s)).collect();
        assert_eq!(MirrorState::Invalid, mirror_state(&mirror));

        let mirror: Vec<Word> = vec!["sator", "arepo", "tenet", "opera", "rotas"]
            .iter()
            .map(|&s| to_word(s))
            .collect();
        assert_eq!(MirrorState::Complete, mirror_state(&mirror));

        let mirror: Vec<Word> = vec!["sator", "arepo", "tenet", "opera"]
            .iter()
            .map(|&s| to_word(s))
            .collect();
        assert_eq!(MirrorState::Partial, mirror_state(&mirror));

        let mirror: Vec<Word> = vec!["sator", "arepo", "tenet", "opera", "rotbs"]
            .iter()
            .map(|&s| to_word(s))
            .collect();
        assert_eq!(MirrorState::Invalid, mirror_state(&mirror));

        let mirror: Vec<Word> = vec!["acara", "cares", "aroma", "reman", "asana"]
            .iter()
            .map(|&s| to_word(s))
            .collect();
        assert_eq!(MirrorState::Invalid, mirror_state(&mirror));

        let mirror: Vec<Word> = vec!["laet", "amir", "eire", "tres"]
            .iter()
            .map(|&s| to_word(s))
            .collect();
        assert_eq!(MirrorState::Invalid, mirror_state(&mirror));
    }
}

// R O T A S 		S A T O R
// O P E R A 		A R E P O
// T E N E T 		T E N E T
// A R E P O 		O P E R A
// S A T O R 		R O T A Z
