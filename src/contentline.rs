use crate::parameters::Parameter;
use crate::value::write_escaped_bytes;
use std::fmt;
use std::io::{Error, Write};

const LINE_MAX_LEN: usize = 75;
const CAPACITY: usize = LINE_MAX_LEN * 2;

pub trait PropertyWrite {
    fn write<W: Write>(&self, line: &mut ContentLine<W>) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct ContentLine<W: Write>(Writer<W>);

impl<W: Write> ContentLine<W> {
    pub(crate) fn new(inner: W) -> ContentLine<W> {
        Self(Writer::new(inner))
    }

    pub(crate) fn write_name_unchecked(&mut self, name: &str) {
        assert!(name.len() <= CAPACITY);
        self.0.extend_buffer(name.as_bytes());
    }

    pub(crate) fn write_property<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        property.write(self)?;
        self.write_line_break()
    }

    pub(crate) fn write_begin(&mut self, component: &str) -> Result<(), Error> {
        if component.len() <= LINE_MAX_LEN - "BEGIN:".len() {
            self.write_begin_unchecked(component)
        } else {
            write!(self.0, "BEGIN:{}", component)?;
            self.write_line_break()
        }
    }

    pub(crate) fn write_end(&mut self, component: &str) -> Result<(), Error> {
        if component.len() <= LINE_MAX_LEN - "END:".len() {
            self.write_begin_unchecked(component)
        } else {
            write!(self.0, "END:{}", component)?;
            self.write_line_break()
        }
    }

    pub(crate) fn write_begin_unchecked(&mut self, component: &str) -> Result<(), Error> {
        writeln!(self.0.inner, "BEGIN:{}\r", component)
    }

    pub(crate) fn write_end_unchecked(&mut self, component: &str) -> Result<(), Error> {
        writeln!(self.0.inner, "END:{}\r", component)
    }

    pub(crate) fn into_inner(mut self) -> Result<W, Error> {
        self.0.flush()?;
        Ok(self.0.inner)
    }

    fn write_line_break(&mut self) -> Result<(), Error> {
        self.0.write_buffer()?;
        self.0.inner.write_all(b"\r\n")
    }
}

impl<W: Write> ContentLine<W> {
    pub fn write_name(&mut self, name: &str) -> Result<(), Error> {
        write!(self.0, "{}", name)
    }

    pub fn write_parameter(&mut self, parameter: &Parameter) -> Result<(), Error> {
        write!(self.0, ";{}", parameter)
    }

    pub fn write_parameter_pair(&mut self, key: &str, value: &str) -> Result<(), Error> {
        write!(self.0, ";{}={}", key, value)
    }

    pub fn write_value<V>(&mut self, value: &V) -> Result<(), Error>
    where
        V: fmt::Display
    {
        write!(self.0, ":{}", value)
    }

    pub fn write_fmt_value(&mut self, value: fmt::Arguments) -> Result<(), Error> {
        write!(self.0, ":{}", value)
    }

    pub fn write_value_text(&mut self, text: &str) -> Result<(), Error> {
        self.0.write_all(b":")?;
        write_escaped_bytes(&mut self.0, text.as_bytes())
    }
}

struct Writer<W: Write> {
    buffer: Box<[u8; CAPACITY]>,
    len: usize,
    inner: W
}

impl<W: Write> Writer<W> {
    fn new(writer: W) -> Writer<W> {
        Self {
            buffer: Box::new([0; CAPACITY]),
            len: 0,
            inner: writer
        }
    }

    fn write_buffer(&mut self) -> Result<(), Error> {
        if self.len > 0 {
            match lazy_fold(&mut self.inner, &self.buffer[..self.len]) {
                Ok(0) => Ok(()),
                Ok(n) => self.inner.write_all(&self.buffer[self.len - n..self.len]),
                Err(error) => Err(error)
            }?;
            self.len = 0;
        }
        Ok(())
    }

    fn extend_buffer(&mut self, buffer: &[u8]) {
        let end = self.len + buffer.len();
        self.buffer[self.len..end].copy_from_slice(buffer);
        self.len = end;
    }
}

impl<W: Write> Write for Writer<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.write_buffer()?;
        self.inner.flush()
    }

    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), Error> {
        if self.len + buf.len() < CAPACITY {
            self.extend_buffer(buf);
            return Ok(());
        }

        let mut end = CAPACITY - self.len;
        loop {
            self.buffer[self.len..CAPACITY].copy_from_slice(&buf[..end]);
            match lazy_fold(&mut self.inner, self.buffer.as_ref()) {
                Ok(n) => {
                    // SAFETY: The n value can never be bigger than CAPACITY because the input
                    // self.buffer is CAPACITY bytes long!
                    self.buffer.copy_within(CAPACITY - n..CAPACITY, 0);
                    self.len = n;
                    buf = &buf[end..];
                    end = CAPACITY - self.len;
                    if buf.len() < end {
                        self.extend_buffer(buf);
                        break;
                    }
                }
                Err(err) => {
                    self.len = CAPACITY;
                    return Err(err);
                }
            }
        }
        Ok(())
    }
}

/// Folds and writes exactly LIMIT * N bytes and returns number of not written
/// bytes.
fn lazy_fold<W: Write>(writer: &mut W, mut content: &[u8]) -> Result<usize, Error> {
    let mut boundary = next_boundary(&content).unwrap_or(content.len());
    writer.write_all(&content[..boundary])?;

    while boundary < content.len() {
        content = &content[boundary..];
        writer.write_all(b"\r\n ")?;
        match next_boundary(&content) {
            Some(next_boundary) => {
                writer.write_all(&content[..next_boundary])?;
                boundary = next_boundary;
            }
            None => return Ok(content.len())
        }
    }
    Ok(0)
}

fn next_boundary(input: &[u8]) -> Option<usize> {
    if input.len() <= LINE_MAX_LEN {
        return None;
    }

    fn is_char_boundary(&b: &u8) -> bool {
        // In std::is_char_boundary bit magic is used in the form of (b as i8) >= -0x40
        // but this is more understandable for me.
        b < 128 || b >= 192
    }

    match input[..=LINE_MAX_LEN].iter().rposition(is_char_boundary) {
        Some(0) | None => None,
        boundary => boundary
    }
}

impl<W: Write + fmt::Debug> fmt::Debug for Writer<W> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Writer")
            .field("buffer", &&self.buffer[..])
            .field("len", &self.len)
            .field("inner", &self.inner)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::Writer;
    use std::io::Write;

    fn write(content: &[u8]) -> Result<String, std::io::Error> {
        let mut writer = Writer::new(Vec::with_capacity(content.len()));
        writer.write_all(content)?;
        writer.flush()?;
        Ok(String::from_utf8_lossy(&writer.inner).to_string())
    }

    #[test]
    fn no_linebreak() {
        let content = "No line break today.";
        let output = write(content.as_bytes()).unwrap();

        assert_eq!(output, content);
    }

    #[test]
    fn over_limit() {
        let content = "Content lines that have a fixed length over 75 bytes should be line folded with CRLF and whitespace.";
        let expected = "Content lines that have a fixed length over 75 bytes should be line folded \r\n with CRLF and whitespace.";
        let output = write(content.as_bytes()).unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn multibytes() {
        let content = "Content lines shouldn't be folded in the middle of a UTF-8 character! 老虎.";
        let expected =
            "Content lines shouldn't be folded in the middle of a UTF-8 character! 老\r\n 虎.";
        let output = write(content.as_bytes()).unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn multi_lines() {
        let content = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy cog. The quick brown fox jumps over the lazy hog. The quick brown fox jumps over the lazy log. The quick brown fox jumps over the lazy dog. ";
        let expected = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over\r\n  the lazy cog. The quick brown fox jumps over the lazy hog. The quick brown\r\n  fox jumps over the lazy log. The quick brown fox jumps over the lazy dog. ";
        let output = write(content.as_bytes()).unwrap();

        assert_eq!(output, expected);
    }
}
