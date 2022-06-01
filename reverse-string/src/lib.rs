pub fn reverse(input: &str) -> String {
    let mut result = String::new();

    result.extend(input.chars().rev());

    result
}
