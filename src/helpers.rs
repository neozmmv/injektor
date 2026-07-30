
pub fn kill_quotes(value: String) -> String {
    let value: String = value.trim().to_string();
    if value.starts_with('"') && value.ends_with('"') || value.starts_with('\'') && value.ends_with('\'') {
        value[1..value.len() - 1].to_string()
    } 
    else {
        value
    }
}