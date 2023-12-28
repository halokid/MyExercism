const MINE: u8 = b'*';

fn count_mines(minefield: &[&str], row_item_number: usize, row_line_number: usize) -> usize {
  let count = (row_line_number.saturating_sub(1)..= row_line_number + 1)
    .filter_map(|row_line_number| minefield.get(row_line_number))
    .flat_map(|line| (row_item_number.saturating_sub(1)..= row_item_number + 1).filter_map(move |x| line.as_bytes().get(x)))
    .filter(|&&c| c == MINE)
    .count();

  count
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
  let mut res = vec![];

  for (row_line_number, row) in minefield.iter().enumerate() {

  }

  ()
}


