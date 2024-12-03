use core::fmt::Arguments;
use heapless::String;
#[allow(dead_code)]

/// Makes it easier to format strings in a single line method
pub fn easy_format<const N: usize>(args: Arguments<'_>) -> String<N> {
    let mut formatted_string: String<N> = String::<N>::new();

    let result = core::fmt::write(&mut formatted_string, args);

    match result {
        Ok(_) => formatted_string,
        Err(_) => {
            panic!("Error formatting the string")
        }
    }
}

pub fn easy_format_str<'a>(
    args: Arguments<'_>,
    buffer: &'a mut [u8],
) -> Result<&'a str, core::fmt::Error> {
    let mut writer = BufWriter::new(buffer);
    let result = core::fmt::write(&mut writer, args);

    match result {
        Ok(_) => {
            let len = writer.len();
            let response_str = core::str::from_utf8(&buffer[..len]).unwrap();
            Ok(response_str)
        }
        Err(_) => {
            panic!("Error formatting the string")
        }
    }
}

// A simple wrapper struct to use core::fmt::Write on a [u8] buffer
pub struct BufWriter<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl<'a> BufWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        BufWriter { buf, pos: 0 }
    }

    pub fn len(&self) -> usize {
        self.pos
    }
}

impl<'a> core::fmt::Write for BufWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        if self.pos + bytes.len() > self.buf.len() {
            return Err(core::fmt::Error); // Buffer overflow
        }

        self.buf[self.pos..self.pos + bytes.len()].copy_from_slice(bytes);
        self.pos += bytes.len();
        Ok(())
    }
}
