#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
  Equal,
  Sublist,
  Superlist,
  Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
  // !todo!()
  let first_len = _first_list.len();
  let second_len = _second_list.len();

  if first_len == 0 && second_len != 0 {
    return Comparison::Sublist;
  }

  if first_len != 0 && second_len == 0 {
    return Comparison::Superlist;
  }

  let (min_len, flag) = min(first_len, second_len);

  for i in 0..min_len {
    if _first_list[i] != _second_list[i] {
      return Comparison::Unequal;
    }
  }

  if flag == 0 {
    return Comparison::Equal;
  }
  if min_len == first_len {
    return Comparison::Sublist;
  }
  return Comparison::Superlist;
}

fn min(a: usize, b: usize) -> (usize, usize) {
  if a == b {
    return (a, 0);
  }
  if a < b {
    return (a, 1);
  }
  return (b, 1);
}
