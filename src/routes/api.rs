use rocket::{
    post,
    delete,
    response::Debug,
    Data,
    http::ContentType
};
use std::{
    io,
    env
};
use crate::utils::{FileId, Password};

#[post("/upload", format = "any", data = "<data>")]
pub fn upload(content_type: &ContentType, data: Data) -> Result<String, Debug<io::Error>> {

    let file_id = FileId::new(16);
    let password = Password::new(16);
    let delete_password = Password::new(16);

    println!("file_id: {}", file_id);
    println!("password: {}", password);
    println!("delete_password: {}", delete_password);

    println!("{:#?}", content_type);
    data.stream_to_file(env::temp_dir().join("upload.txt"))
        .map(|n| n.to_string())
        .map_err(Debug)

}

// #[delete("/delete", format = "json", data = "<data>")]
// pub fn delete(data: DeleteFile) -> Result<(), ()> {

//     unimplemented!()

// }

