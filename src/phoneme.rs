use std::cmp::{Eq, Ord, PartialOrd, Ordering, PartialEq};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct Phoneme {
  pub symbol: String,
}

#[derive(Debug, Clone, Eq)]
pub struct Word {
  pub spelling: String,
  pub phonemes: Vec<Phoneme>,
}

impl Ord for Word {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl PartialOrd for Word {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.phonemes.partial_cmp(&other.phonemes)
  }
}

impl PartialEq for Word {
  fn eq(&self, other: &Self) -> bool {
    self.phonemes.eq(&other.phonemes)
  }
}
