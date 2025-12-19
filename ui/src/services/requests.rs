use common::error::*;
use lazy_static::lazy_static;
use serde::{Serialize, de::DeserializeOwned};

use crate::services::auth::get_token;

lazy_static! {
    pub static ref SERVER_ADDRESS: String =
        web_sys::window().expect("API_ROOT is not set").origin();
    static ref API_ROOT: String = format!("{}/api", *SERVER_ADDRESS);
}

/// build all kinds of http request: post/get/delete etc.
pub async fn request<B, T>(method: reqwest::Method, url: &str, body: &B) -> Result<T, AppError>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let url = format!("{}{}", *SERVER_ADDRESS, url);

    log::debug!("Sending {} request to {}", method, url);

    let with_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");

    if let Some(token) = get_token() {
        builder = builder.bearer_auth(token);
    }

    if with_body {
        builder = builder.json(&body);
    }

    let response = builder.send().await;

    if let Ok(data) = response {
        if data.status().is_success() {
            // log::debug!("Got Ok response with data {:?}", data);

            let data: Result<T, _> = data.json::<T>().await;
            if let Ok(data) = data {
                // log::debug!("Response: {:?}", data);

                Ok(data)
            } else {
                log::debug!("Couldn't deserialise response: {:?}", data);
                Err(AppError::DeserializeError)
            }
        } else {
            match data.status().as_u16() {
                401 => Err(AppError::Unauthorized(
                    data.json::<String>().await.unwrap_or_default(),
                )),
                403 => Err(AppError::Forbidden(
                    data.json::<String>().await.unwrap_or_default(),
                )),
                404 => Err(AppError::NotFound),
                500 => Err(AppError::InternalServerError),
                422 => {
                    let data = data.json::<Vec<String>>().await;
                    if let Ok(data) = data {
                        Err(AppError::UnprocessableEntity(data))
                    } else {
                        Err(AppError::DeserializeError)
                    }
                }
                _ => Err(AppError::RequestError),
            }
        }
    } else {
        Err(AppError::RequestError)
    }
}

pub async fn request_api<B, T>(method: reqwest::Method, url: &str, body: &B) -> Result<T, AppError>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(method, format!("/api{url}").as_str(), body).await
}

/// Delete api request
pub async fn request_api_delete<T>(url: &str) -> Result<T, AppError>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request_api(reqwest::Method::DELETE, url, &()).await
}

/// Get api request
pub async fn request_api_get<T>(url: &str) -> Result<T, AppError>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request_api(reqwest::Method::GET, url, &()).await
}

/// Post api request
pub async fn request_api_post<T, B>(url: &str, body: &B) -> Result<T, AppError>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request_api(reqwest::Method::POST, url, body).await
}

/// Put api request with a body
pub async fn request_api_put<B, T>(url: &str, body: &B) -> Result<T, AppError>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request_api(reqwest::Method::PUT, url, body).await
}

/// Get request
pub async fn request_get<T>(url: &str) -> Result<T, AppError>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::GET, url, &()).await
}
