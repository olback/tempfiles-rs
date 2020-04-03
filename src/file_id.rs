// Based on https://github.com/SergioBenitez/Rocket/blob/v0.4/examples/pastebin/src/paste_id.rs

use std::{
    fmt,
    convert::TryInto
};
use rocket::{
    request::FromParam,
    http::RawStr
};
use rand::{self, Rng};

/// Table to retrieve chars from.
const VALID_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

#[derive(Clone)]
pub struct FileId(String);
pub type Password = FileId;

impl FileId {

    pub fn new(size: usize) -> FileId {

        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();

        for _ in 0..size {
            id.push(VALID_CHARS[rng.gen::<usize>() % VALID_CHARS.len()] as char);
        }

        FileId(id)

    }

    pub fn is_valid(id: &str) -> bool {

        id.chars().all(|c| {
            VALID_CHARS.contains(&(c as u8))
        })

    }

    pub fn as_array32(&self) -> [u8; 32] {

        self.0.as_bytes().try_into().unwrap()

    }

}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> FromParam<'a> for FileId {

    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<FileId, &'a RawStr> {

        match FileId::is_valid(param) {
            true => Ok(FileId(String::from(param.as_str()))),
            false => Err(param)
        }

    }

}

impl  Into<String> for FileId {

    fn into(self) -> String {

        self.0

    }

}
