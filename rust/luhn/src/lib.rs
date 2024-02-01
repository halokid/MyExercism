/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let mut len = 0;

    for (item, char) in code.chars().rev().filter(|&c| c != ' ').enumerate() {
        len += 1;

        match (item % 2, char.to_digit(10)) {
            // (1, Some(x)) => { if x > 4 { sum += x * 2 - 9 } },
            (1, Some(x)) if x > 4 => sum += x * 2 - 9,
            (1, Some(x)) => { sum += x * 2 },
            (0, Some(x)) => { sum += x },
            _ =>  return false
        }
    }

    (len > 1) && (sum % 10 == 0)
}
