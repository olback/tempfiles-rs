use {
    super::ApiError,
    crate::{
        config::TempfilesConfig,
        content::Content,
        crypto::Crypto,
        db::{schemas::TempfilesDatabase, TempfilesDatabaseConn},
        file_id::FileId,
        impl_responder,
        password::Password,
    },
    rocket::{data::ToByteUnit, http::ContentType, post, Data, State},
    serde::Serialize,
    serde_json,
};

#[derive(Serialize)]
pub struct ApiUploadResponse {
    pub status: u16,
    pub id: String,
    pub password: String,
    pub delete_password: String,
    pub download_url: String,
}

impl_responder!(ApiUploadResponse);

#[post("/upload?<filename>&<maxviews>", data = "<data>")]
pub async fn upload(
    filename: Option<&str>,
    maxviews: i32,
    content_type: Option<&ContentType>,
    data: Data<'_>,
    tc: &State<TempfilesConfig>,
    mut db: TempfilesDatabaseConn,
) -> Result<ApiUploadResponse, ApiError> {
    let mut content = Content {
        filename: filename.map(|f| f.to_string()),
        content_type: content_type.map(|ct| ct.to_string()),
        data: Vec::<u8>::with_capacity(3 * 1024 * 1024),
    };

    let size = data
        .open(tc.max_file_size.bytes())
        .stream_to(&mut content.data)
        .await?
        .written;

    // Check for invalid data
    if size == 0 {
        return Err(ApiError::new("File may not be empty", 422));
    } else if size == tc.max_file_size as u64 {
        return Err(ApiError::new("File too large", 422));
    } else if maxviews < 1 {
        return Err(ApiError::new("Max views may not be below 1", 422));
    } else if maxviews > tc.max_views {
        return Err(ApiError::new(
            "Max views may not be above the server limit",
            422,
        ));
    }

    let id = FileId::new(16);
    let password = Password::new();
    let delete_password = Password::new();

    let iv = Crypto::gen_iv();
    let content_bytes = bincode::serialize(&content)?;

    let enc = Crypto::encrypt(iv, password.as_array32(), &content_bytes)?;

    TempfilesDatabase::insert(
        &mut db,
        id.clone().into(),
        &iv,
        enc,
        maxviews,
        delete_password.clone().into(),
    )
    .await?;

    let file_id = id.into();
    let file_password = password.into();
    let download_url = format!("{}/d/{}/{}", tc.base_url, file_id, file_password);

    Ok(ApiUploadResponse {
        status: 201,
        id: file_id,
        password: file_password,
        delete_password: delete_password.into(),
        download_url,
    })
}
