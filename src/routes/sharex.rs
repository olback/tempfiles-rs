use rocket::{get, State};
use rocket_contrib::json::Json;
use crate::config::TempfilesConfig;
use serde::Serialize;

#[derive(Serialize)]
pub struct Parameters<'a> {
    filename: &'a str
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct ShareXConfig<'a> {
    Version: &'a str,
    Name: String,
    DestinationType: &'a str,
    RequestMethod: &'a str,
    RequestURL: String,
    Parameters: Parameters<'a>,
    Body: &'a str,
    URL: String,
    DeletionURL: String
}

#[get("/sharex.sxcu")]
pub fn sharex(tc: State<TempfilesConfig>,) -> Json<ShareXConfig> {

    let sharex_config = ShareXConfig {
        Version: "13.1.0",
        Name: tc.name.clone(),
        DestinationType: "ImageUploader, TextUploader, FileUploader",
        RequestMethod: "POST",
        RequestURL: format!("{}/api/upload", tc.base_url),
        Parameters: Parameters {
            filename: "$filename$"
        },
        Body: "Binary",
        URL: format!("{}/d/$json:id$/$json:password$", tc.base_url),
        DeletionURL: format!("{}/api/delete/$json:id$/$json:delete_password$", tc.base_url)
    };

    Json(sharex_config)

}
