// Based on https://github.com/SergioBenitez/Rocket/blob/v0.4/examples/pastebin/src/paste_id.rs

use std::{
    fmt,
    borrow::Cow
};
use rocket::{
    request::FromParam,
    http::RawStr
};
use rand::{self, Rng};

/// Table to retrieve chars from.
const VALID_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub struct FileId<'a>(Cow<'a, str>);

impl<'a> FileId<'a> {

    pub fn new(size: usize) -> FileId<'static> {

        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();

        for _ in 0..size {
            id.push(VALID_CHARS[rng.gen::<usize>() % VALID_CHARS.len()] as char);
        }

        FileId(Cow::Owned(id))

    }

    pub fn is_valid(id: &str) -> bool {

        id.chars().all(|c| {
            VALID_CHARS.contains(&(c as u8))
        })

    }

}

impl<'a> fmt::Display for FileId<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> FromParam<'a> for FileId<'a> {

    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<FileId<'a>, &'a RawStr> {

        match FileId::is_valid(param) {
            true => Ok(FileId(Cow::Borrowed(param))),
            false => Err(param)
        }

    }

}

