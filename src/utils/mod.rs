pub mod file_handling;

pub fn is_newline(s: &str) -> bool {
    s == "\n" || s == "\r\n" || s == "\r"
}
