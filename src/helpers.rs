
pub fn kill_quotes(value: String) -> String {
    let value: String = value.trim().to_string();
    if (value.starts_with('"') && value.ends_with('"'))
        || (value.starts_with('\'') && value.ends_with('\''))
    {
        value[1..value.len() - 1].to_string()
    } else {
        value
    }
}

/// if the value is quoted (starts with " or '), '#' is never a comment
/// 
/// otherwise, '#' is only treated as a comment if preceded by whitespace
pub fn strip_inline_comment(value: &str) -> &str {
    if value.starts_with('"') || value.starts_with('\'') {
        return value;
    }

    match value.find(" #") {
        Some(idx) => value[..idx].trim_end(),
        None => value,
    }
}