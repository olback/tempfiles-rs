mod file_id;
pub use file_id::FileId;
pub type Password<'a> = file_id::FileId<'a>;
