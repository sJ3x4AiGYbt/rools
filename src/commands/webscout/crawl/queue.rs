use crate::commands::webscout::url;
use std::collections::{HashSet, VecDeque};

pub struct CrawlQueue {
  pending: VecDeque<String>,
  visited: HashSet<String>,
  base_host: String,
  max_pages: usize,
}

impl CrawlQueue {
  pub fn new(seed_url: &str, base_host: &str, max_pages: usize) -> Self {
    let mut pending = VecDeque::new();
    pending.push_back(seed_url.to_string());

    Self {
      pending,
      visited: HashSet::new(),
      base_host: base_host.to_string(),
      max_pages,
    }
  }

  pub fn next(&mut self) -> Option<String> {
    while let Some(url) = self.pending.pop_front() {
      if self.visited.contains(&url) {
        continue;
      }
      self.visited.insert(url.clone());
      return Some(url);
    }
    None
  }

  pub fn enqueue_links(&mut self, links: &[String]) {
    for link in links {
      if self.visited.len() + self.pending.len() >= self.max_pages {
        break;
      }
      if !self.visited.contains(link) && url::same_domain(link, &self.base_host) {
        self.pending.push_back(link.clone());
      }
    }
  }

  pub fn visited(&self) -> &HashSet<String> {
    &self.visited
  }

  pub fn visited_count(&self) -> usize {
    self.visited.len()
  }
}
