use std::error::Error;

pub mod file_handling;

pub fn is_newline(s: &str) -> bool {
    s == "\n" || s == "\r\n" || s == "\r"
}

pub fn unescape_unicode(s: &[u8]) -> Result<String, Box<dyn Error>> {
    let mut output = Vec::new();
    let mut i = 0;
    
    while i < s.len() {
        match s[i] {
            b'\\' => {
                i += 1;
                match s[i] {
                    b'u' => {
                        let num = u8::from_str_radix(std::str::from_utf8(&s[i+1..][..4])?, 16)?;
                        output.push(num);
                        i += 4;
                    }
                    byte => output.push(byte),
                }
            },
            byte => output.push(byte),
        }
        i += 1;
    }

    Ok(String::from_utf8(output)?)
}
