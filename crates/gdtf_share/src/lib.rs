use bytes::Bytes;

use reqwest::{Client, StatusCode};

const AUTH_URL: &'static str = "https://gdtf-share.com/apis/public/login.php";
const GET_LIST_URL: &'static str = "https://gdtf-share.com/apis/public/getList.php";
const DOWNLOAD_FILE_URL: &'static str = "https://gdtf-share.com/apis/public/downloadFile.php";

#[derive(Debug, Clone)]
pub struct GdtfShare {
    client: Client,
}

impl GdtfShare {
    pub async fn auth(user: String, password: String) -> Result<Self, Error> {
        let client = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .build()
            .unwrap();

        let response = send_auth_request(&client, user, password).await?;
        if !response.result {
            return Err(Error::AuthFailed {
                message: response.error.unwrap_or("No error message".to_string()),
            });
        }
        Ok(Self { client })
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

    pub async fn download_file(&self, rid: i32) -> Result<Bytes, Error> {
        send_download_file_request(&self.client, rid).await
    }
}

async fn send_auth_request(
    client: &Client,
    user: String,
    password: String,
) -> Result<AuthResponse, Error> {
    let params = [("user", user), ("password", password)];
    client
        .post(AUTH_URL)
        .form(&params)
        .send()
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

async fn send_get_list_request(client: &Client) -> Result<ListResponse, Error> {
    client
        .get(GET_LIST_URL)
        .send()
        .await
        .map_err(|_| Error::Unauthorized)?
        .json::<ListResponse>()
        .await
        .map_err(|e| Error::GetListFailed {
            message: e.to_string(),
        })
}

async fn send_download_file_request(client: &Client, rid: i32) -> Result<Bytes, Error> {
    let response = client
        .get(DOWNLOAD_FILE_URL)
        .query(&[("rid", rid)])
        .send()
        .await
        .map_err(|_| Error::Unauthorized)?;

    match response.status() {
        StatusCode::OK => match response.bytes().await {
            Ok(bytes) => Ok(bytes),
            Err(e) => {
                return Err(Error::DownloadFailed {
                    message: e.to_string(),
                });
            }
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

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{Error, GdtfShare};

    #[tokio::test]
    async fn test_auth_success() {
        dotenv::dotenv().ok();
        let user = env::var("TEST_GDTF_SHARE_API_USER").unwrap();
        let password = env::var("TEST_GDTF_SHARE_API_PASSWORD").unwrap();

        let gdtf_share = GdtfShare::auth(user, password).await;
        assert!(gdtf_share.is_ok());
    }

    #[tokio::test]
    async fn test_auth_fail() {
        dotenv::dotenv().ok();
        let user = "wrong_name".to_string();
        let password = "password123".to_string();

        let gdtf_share = GdtfShare::auth(user, password).await;
        assert!(matches!(gdtf_share, Err(Error::AuthFailed { .. })));
    }

    #[tokio::test]
    async fn test_get_list_success() {
        dotenv::dotenv().ok();
        let user = env::var("TEST_GDTF_SHARE_API_USER").unwrap();
        let password = env::var("TEST_GDTF_SHARE_API_PASSWORD").unwrap();

        let gdtf_share = GdtfShare::auth(user, password).await.unwrap();
        let list = gdtf_share.get_list().await;
        assert!(list.is_ok());
    }

    #[tokio::test]
    async fn test_download_file_success() {
        dotenv::dotenv().ok();
        let user = env::var("TEST_GDTF_SHARE_API_USER").unwrap();
        let password = env::var("TEST_GDTF_SHARE_API_PASSWORD").unwrap();

        let gdtf_share = GdtfShare::auth(user, password).await.unwrap();
        let bytes = gdtf_share.download_file(347).await;
        dbg!(&bytes);
        assert!(bytes.is_ok());
    }
}
