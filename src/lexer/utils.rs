pub fn is_number(char: &str) -> Result<i32, ()> {
    if let Ok(val) = char.parse::<i32>() {
        return Ok(val);
    }
    Err(())
}

pub fn is_whitespace(char: &str) -> bool {
    char == " " || char == "\t" || char == "\n"
}

pub fn is_new_line(char: &str) -> bool {
    char == "\n"
}
