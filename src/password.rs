use {
    crate::file_id::FileId,
    rocket::request::FromParam,
    std::{convert::TryInto, fmt},
};

#[derive(Clone)]
pub struct Password(FileId);

impl Password {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Password(FileId::new(32))
    }

    pub fn as_array32(&self) -> [u8; 32] {
        self.0.as_bytes().try_into().unwrap()
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> FromParam<'a> for Password {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Password, &'a str> {
        match FileId::is_valid(param) && param.len() == 32 {
            true => Ok(Password(FileId::new_from_str(param))),
            false => Err(param),
        }
    }
}

impl From<Password> for String {
    fn from(val: Password) -> Self {
        val.0.into()
    }
}
