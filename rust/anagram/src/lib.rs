use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
  let word_lower = word.to_lowercase();
  let word_sorted = do_sorted(&word_lower);

  possible_anagrams
    .iter()
    .filter(|item| {
      let item_lower = item.to_lowercase();
      let item_sorted = do_sorted(&item_lower);
      condition_one(word_lower.clone(), item_lower.clone()) &&
        condition_two(word_lower.clone(), item_lower.clone()) &&
        condition_three(word_sorted.clone(), item_sorted.clone())
    }).copied().collect()
}

fn do_sorted(s: &str) -> Vec<char> {
  let mut schars: Vec<char> = s.chars().collect();
  schars.sort_unstable();
  schars
}

fn condition_one(s: String, d: String) -> bool {
  s.len() == d.len()
}

fn condition_two(s_low: String, d_low: String) -> bool {
  s_low != d_low
}

fn condition_three(s_sorted: Vec<char>, d_sorted: Vec<char>) -> bool {
  println!("condition_three s_sorted: {:?}", s_sorted);
  println!("condition_three d_sorted: {:?}", d_sorted);
  s_sorted == d_sorted
}

