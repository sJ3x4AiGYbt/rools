use std::collections::HashMap;

pub struct Stats {
  counts: HashMap<String, u64>,
}

impl Stats {
  pub fn new() -> Self {
    Self {
      counts: HashMap::new(),
    }
  }

  pub fn increment(&mut self, key: String) {
    *self.counts.entry(key).or_insert(0) += 1;
  }

  pub fn get_counts(&self) -> &HashMap<String, u64> {
    &self.counts
  }
}
