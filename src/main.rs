use anyhow::{Context, Result};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn make_dict<P: AsRef<Path>>(path: P) -> Result<BTreeSet<String>> {
    let mut dict = BTreeSet::new();
    let file = fs::File::open(&path)
        .with_context(|| format!("while opening path={}", path.as_ref().to_string_lossy()))?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(word) = line {
            dict.insert(word);
        }
    }
    Ok(dict)
}

fn find_mirrors(dict: &BTreeSet<String>, length: usize) -> BTreeSet<Vec<String>> {
    let mut results = BTreeSet::new();
    find_mirrors_with(dict, length,  &Vec::new(), &mut results);
    results
}

fn find_mirrors_with(
    dict: &BTreeSet<String>,
    length: usize,
    history: &Vec<String>,
    result: &mut BTreeSet<Vec<String>>,
) {
    for word in dict {
        if word.len() != length {
            continue;
        }
        let mut mirror = history.clone();
        mirror.push(word.clone());
        let state = mirror_state(&mirror);
        match state {
            MirrorState::Complete => {
                println!("found mirror={:?}", &mirror);
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
fn mirror_state(mirror: &Vec<String>) -> MirrorState {
    if mirror.len() == 0 {
        return MirrorState::Partial;
    }
    let len = mirror[0].len();
    if mirror.last().unwrap().len() != len {
        return MirrorState::Invalid;
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
                    let ij = iword.chars().nth(j).unwrap();
                    let ji = jword.chars().nth(i).unwrap();
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
        for word in mirror {
            print!("{}", word);
        }
        println!("");
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{mirror_state, MirrorState};

    #[test]
    fn test_mirror_state() {
        let mirror: Vec<String> = vec!["sator"].iter().map(|&s| s.to_string()).collect();
        assert_eq!(MirrorState::Partial, mirror_state(&mirror));

        let mirror: Vec<String> = vec!["cow", "win"].iter().map(|&s| s.to_string()).collect();
        assert_eq!(MirrorState::Invalid, mirror_state(&mirror));

        let mirror: Vec<String> = vec!["sator", "arepo", "tenet", "opera", "rotas"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        assert_eq!(MirrorState::Complete, mirror_state(&mirror));

        let mirror: Vec<String> = vec!["sator", "arepo", "tenet", "opera"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        assert_eq!(MirrorState::Partial, mirror_state(&mirror));

        let mirror: Vec<String> = vec!["sator", "arepo", "tenet", "opera", "rotbs"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        assert_eq!(MirrorState::Invalid, mirror_state(&mirror));
    }
}

// R O T A S 		S A T O R
// O P E R A 		A R E P O
// T E N E T 		T E N E T
// A R E P O 		O P E R A
// S A T O R 		R O T A Z
