fn main() {
  let s = &["he", "lo"];
  for (row) in s.iter() {
    // println!("i: {}, row: {:?}", i, row);
    println!(" row: {:?}", row);
  }

  // let minefield = &[
  let test_case = &[
    "1*1",
    "1*1",
    "111",
  ];

  // for (i, row) in minefield.iter() {
  //   println!("i: {}, row: {:?}", i, row);
  // }

  let cleaned = remove_annotations(test_case);
  let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
  println!("clean_strs -->>> {:?}", cleaned_strs);

  let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
  println!("expected -->>> {:?}", expected);

  // for (row_line_number, row) in minefield.iter().enumerate() {
  // for (row_line_number, row) in &cleaned_strs.iter().enumerate() {
  for (row_line_number, row) in test_case.iter().enumerate() {
    println!("===============================================================");
    println!("row_line_number: {}, row: {}", row_line_number, row);
    for (row_item_number, item) in row.as_bytes().iter().enumerate() {
      println!("------------------------------------------");
      println!("row_item_number: {}, item: {}", row_item_number, item);
      // test_count(minefield, row_item_number, row_line_number);
      let count = test_count(&cleaned_strs, row_item_number, row_line_number);
      println!("count -->>> {}", count);
    }
  }
}

fn test_count(minefield: &[&str], row_item_number: usize, row_line_number: usize) -> usize {
  // !todo!()
  let count = (row_line_number.saturating_sub(1)..=row_line_number + 1)
    // .filter_map(|y|println!("minefield y -->>> {}", minefield.get(row_line_number)));
    // .filter_map(|y|println!(minefield.get(row_line_number)))
    .filter_map(|row_line_number| minefield.get(row_line_number))
    .flat_map(|line| (row_item_number.saturating_sub(1)..=row_item_number + 1).filter_map(move |x| line.as_bytes().get(x)))
    .filter(|&&c| c == b'*')
  .count();
  // println!("line_res -->>> {:?}", line_res.count());

  count
}

fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}
fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}

fn run_test(test_case: &[&str]) {
  // let cleaned = remove_annotations(test_case);
  // let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
  // let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
  // assert_eq!(expected, annotate(&cleaned_strs));
}






