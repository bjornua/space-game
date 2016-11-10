use std::io;
use std::io::Read;
use std::str::{from_utf8};

// use std::error::Error as StdError;

// use parser::{Command, get_command};

// pub struct ConsoleInput<'a> {
//     iterator: Box<Iterator<Item = char> + 'a>,
// }
// impl<'a> ConsoleInput<'a> {
//     pub fn new<R: Read + 'a>(stream: R) -> Self {
//         let stream = stream.chars().take_while(|x| x.is_ok()).filter_map(|x| x.ok());

//         return ConsoleInput {
//             iterator: Box::new(stream)
//         }
//     }
// }


// impl<'a> Iterator for ConsoleInput<'a> {
//     type Item = Command;
//     fn next(&mut self) -> Option<Self::Item> {
//         return get_command(&mut self.iterator)
//     }
// }


#[derive(Debug)]
pub struct CharIterator<R: Read> {
    buffer: [u8; 6],
    buffer_start: usize,
    buffer_end: usize,
    buffer_invalid: [u8; 6],
    buffer_invalid_end: usize,
    reader: R,
}

impl<R: Read> CharIterator<R> {
    pub fn new(reader: R) -> Self {
        CharIterator {
            buffer: [255; 6],
            buffer_start: 0,
            buffer_end: 0,
            buffer_invalid: [0; 6],
            buffer_invalid_end: 0,
            reader: reader,
        }
    }
    fn buffer_empty(&self) -> bool {
        self.buffer_start == self.buffer_end
    }
    pub fn fill_buffer(&mut self) -> io::Result<()> {
        if !self.buffer_empty() {
            return Ok(());
        }

        self.buffer[..self.buffer_invalid_end]
            .copy_from_slice(&self.buffer_invalid[..self.buffer_invalid_end]);

        let bytes_read = try!(self.reader.read(&mut self.buffer[self.buffer_invalid_end..]));

        self.buffer_start = 0;
        self.buffer_end = bytes_read + self.buffer_invalid_end;

        let buffer_invalid_begin = self.find_invalid();
        self.buffer_invalid_end = self.buffer_end - buffer_invalid_begin;
        self.buffer_invalid[..self.buffer_invalid_end].copy_from_slice(&self.buffer[buffer_invalid_begin..self.buffer_end]);
        self.buffer_end = buffer_invalid_begin;
        Ok(())
    }
    pub fn find_invalid(&self) -> usize {
        use std::str::Utf8Error;
        match from_utf8(&self.buffer[self.buffer_start..self.buffer_end]) {
            Err(e@ Utf8Error { .. }) => {
                self.buffer_start + e.valid_up_to()
            }
            Ok(_) => {
                self.buffer_end
            }
        }

    }
    pub fn as_str<'a>(&'a self) -> &'a str {
        from_utf8(&self.buffer[self.buffer_start..self.buffer_end]).unwrap()
    }
    pub fn consume_char(&mut self) -> Option<char> {
        self.fill_buffer().unwrap();
        match self.as_str().chars().next() {
            Some(c) => {
                self.buffer_start += c.len_utf8();
                Some(c)
            }
            None => None
        }
    }
    pub fn debug(&self) {
        println!("\
            buffer: {:?}\n\
            buffer_valid_until: {:?}\n\
            buffer_start: {}\n\
            buffer_end: {}\n\
            buffer_invalid: {:?}\n\
            buffer_invalid_end: {:?}\
            \n
        ",
                 self.buffer,
                 self.find_invalid(),
                 self.buffer_start,
                 self.buffer_end,
                 self.buffer_invalid,
                 self.buffer_invalid_end);
    }
}

impl<R: Read> Iterator for CharIterator<R> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        return self.consume_char();
    }
}

// impl<'a> ConsoleInput<'a> {
//     fn close(&mut self) -> Option<R> {
//         self.stream.take()
//     }
// }
