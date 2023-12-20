mod lib2;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
  Equal,
  Sublist,
  Superlist,
  Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
  let first_len = _first_list.len();
  let second_len = _second_list.len();

  if first_len == 0 && second_len != 0 {
    return Comparison::Sublist;
  }

  if first_len != 0 && second_len == 0 {
    return Comparison::Superlist;
  }

  if first_len <= second_len {
    let start_position = find_position(_first_list.get(0), _second_list);

    if (start_position + first_len > second_len) || start_position == 0 {
      return Comparison::Unequal;
    }

    // if _first_list == _second_list[start_position..second_len]
  }

  Comparison::Sublist
}

// find start position for item in collection
fn find_position<T: PartialEq>(item: T, list: &[T]) -> usize {
  for i in 0..list.len() {
    if list[i] == item {
      return i;
    }
  }
  return 0;
}




