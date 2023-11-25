pub fn reverse(input: &str) -> String {
    if input == "" {
        return input.to_string();
    }
    let mut chars: Vec<char> = input.chars().collect();
    let (mut left, mut right) = (0, chars.len() - 1);
    while left < right {
        chars.swap(left, right);
        left += 1;
        right -= 1;
    }
    chars.into_iter().collect()
}
