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
    let mut new_row = String::new();
    for (row_item_number, item) in row.as_bytes().iter().enumerate() {
      if *item == MINE {
        new_row.push(*item as char)
      } else {
        let mine_num = count_mines(minefield, row_item_number, row_line_number);

        if mine_num > 0 {
          new_row.push(char::from_digit(mine_num as u32, 10).unwrap())
        } else {
          new_row.push(' ');
        }
      }
    }

    res.push(new_row);
  }
  res
}


