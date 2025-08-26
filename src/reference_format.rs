
// format reference
pub fn format_reference(reference: &str) -> String {
    let mut formatted = String::new();
    let mut lines = reference.lines();
    if let Some(first_line) = lines.next() {
        formatted.push_str(first_line);
    }
    for line in lines {
        formatted.push('\n');
        formatted.push_str(line);
    }
    formatted
}
