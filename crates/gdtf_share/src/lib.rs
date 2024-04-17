use isahc::config::Configurable;
use isahc::cookies::CookieJar;
use isahc::http::StatusCode;
use isahc::{AsyncReadResponseExt, HttpClient};
const AUTH_URL: &str = "https://gdtf-share.com/apis/public/login.php";
const GET_LIST_URL: &str = "https://gdtf-share.com/apis/public/getList.php";
const DOWNLOAD_FILE_URL: &str = "https://gdtf-share.com/apis/public/downloadFile.php";

#[derive(Debug, Clone)]
pub struct GdtfShare {
    client: HttpClient,
    user: String,
    password: String,
}

impl GdtfShare {
    pub fn new(user: String, password: String) -> Self {
        let jar = CookieJar::default();
        let client = HttpClient::builder().cookie_jar(jar).build().unwrap();
        Self {
            client,
            user,
            password,
        }
    }

    pub async fn auth(&self) -> Result<(), Error> {
        let response = send_auth_request(&self.client, &self.user, &self.password).await?;
        if !response.result {
            return Err(Error::AuthFailed {
                message: response.error.unwrap_or("No error message".to_string()),
            });
        }

        Ok(())
    }

    pub async fn get_list(&self) -> Result<Vec<FixtureInfo>, Error> {
        let response = send_get_list_request(&self.client).await?;
        if !response.result {
            return Err(Error::GetListFailed {
                message: response.error.unwrap_or("No error message".to_string()),
            });
        }
        match response.list {
            Some(list) => Ok(list),
            None => Err(Error::GetListFailed {
                message: "missing fixture list".to_string(),
            }),
        }
    }

    pub async fn download_file(&self, rid: i32) -> Result<Vec<u8>, Error> {
        send_download_file_request(&self.client, rid).await
    }
}

async fn send_auth_request(
    client: &HttpClient,
    user: &str,
    password: &str,
) -> Result<AuthResponse, Error> {
    let body = format!("user={user}&password={password}");
    client
        .post_async(AUTH_URL, body)
        .await
        .map_err(|e| Error::AuthFailed {
            message: e.to_string(),
        })?
        .json::<AuthResponse>()
        .await
        .map_err(|e| Error::AuthFailed {
            message: e.to_string(),
        })
}

async fn send_get_list_request(client: &HttpClient) -> Result<ListResponse, Error> {
    client
        .get_async(GET_LIST_URL)
        .await
        .map_err(|_| Error::Unauthorized)?
        .json::<ListResponse>()
        .await
        .map_err(|e| Error::GetListFailed {
            message: e.to_string(),
        })
}

async fn send_download_file_request(client: &HttpClient, rid: i32) -> Result<Vec<u8>, Error> {
    let mut response = client
        .get_async(format!("{DOWNLOAD_FILE_URL}?rid={rid}"))
        .await
        .map_err(|_| Error::Unauthorized)?;

    match response.status() {
        StatusCode::OK => match response.bytes().await {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(Error::DownloadFailed {
                message: e.to_string(),
            }),
        },
        _ => {
            let response = match response.json::<DownloadFileResponse>().await {
                Err(e) => {
                    return Err(Error::DownloadFailed {
                        message: e.to_string(),
                    });
                }
                Ok(response) => response,
            };

            let message = match response.error {
                Some(message) => message,
                None => {
                    return Err(Error::DownloadFailed {
                        message: "No error message".to_string(),
                    })
                }
            };
            Err(Error::DownloadFailed { message })
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct AuthResponse {
    pub result: bool,
    pub error: Option<String>,
    // pub notice: Option<String>
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ListResponse {
    pub result: bool,
    pub error: Option<String>,
    pub list: Option<Vec<FixtureInfo>>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixtureInfo {
    pub rid: i32,
    pub fixture: String,
    pub manufacturer: String,
    pub revision: String,
    pub creation_date: i64,
    pub last_modified: i64,
    pub uploader: String,
    pub rating: String,
    pub version: String,
    pub creator: String,
    pub uuid: Option<String>,
    pub filesize: i64,
    pub modes: Vec<FixtureInfoMode>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FixtureInfoMode {
    pub name: String,
    #[serde(rename = "dmxfootprint")]
    pub dmx_footprint: i32,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DownloadFileResponse {
    pub result: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("Authenticaton failed: {message}")]
    AuthFailed { message: String },
    #[error("Failed to authenticate: {message}")]
    InvalidCredentials { message: String },
    #[error("Unauthorized access")]
    Unauthorized,
    #[error("Failed to get list of fixtures: {message}")]
    GetListFailed { message: String },
    #[error("Failed to download file: {message}")]
    DownloadFailed { message: String },
}
