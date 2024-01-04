// Based on https://github.com/SergioBenitez/Rocket/blob/v0.4/examples/pastebin/src/paste_id.rs

use {
    rand::{self, Rng},
    rocket::request::FromParam,
    std::fmt,
};

/// Table to retrieve chars from.
const VALID_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

#[derive(Clone)]
pub struct FileId(String);

impl FileId {
    pub fn new(size: usize) -> FileId {
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();

        for _ in 0..size {
            id.push(VALID_CHARS[rng.gen::<usize>() % VALID_CHARS.len()] as char);
        }

        FileId(id)
    }

    pub fn new_from_str(s: &str) -> FileId {
        FileId(s.into())
    }

    pub fn is_valid(id: &str) -> bool {
        id.chars().all(|c| VALID_CHARS.contains(&(c as u8)))
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> FromParam<'a> for FileId {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<FileId, &'a str> {
        match FileId::is_valid(param) {
            true => Ok(FileId(String::from(param))),
            false => Err(param),
        }
    }
}

impl From<FileId> for String {
    fn from(val: FileId) -> Self {
        val.0
    }
}
