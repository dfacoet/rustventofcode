pub fn parse_input(input: String) -> Vec<String> {
    // Can't just return Vec<str>
    // input.lines().collect()
    // because input is owned by parse_input
    input.lines().map(|line| line.to_string()).collect()
}
