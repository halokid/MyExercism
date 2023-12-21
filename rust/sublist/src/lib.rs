mod lib2;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
  Equal,
  Sublist,
  Superlist,
  Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
  if _first_list == _second_list {
    return Comparison::Equal;
  }

  if _first_list.len() == 0 && _second_list.len() > 0 {
    return Comparison::Sublist;
  }

  if _second_list.len() == 0 && _first_list.len() > 0 {
    return Comparison::Superlist;
  }

  if _first_list.len() > _second_list.len() {
    if _first_list.windows(_second_list.len())
      .any(|items| items == _second_list) {
      return  Comparison::Superlist;
    }
  }

  if _second_list.len() > _first_list.len() {
    if _second_list.windows(_first_list.len())
      .any(|items| items == _first_list) {
      return  Comparison::Sublist;
    }
  }

  return  Comparison::Unequal;
}



