use std::io;

pub fn read_line() -> Result<String, io::Error> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    match buf.pop() {
        Some(c) if c == '\n' => Ok(buf),
        Some(c) => Ok(buf + &c.to_string()),
        None => Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF")),
    }
}
