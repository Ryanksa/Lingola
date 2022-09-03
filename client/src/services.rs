use lazy_static::lazy_static;
use reqwest::{Client, Error};
use serde::de::DeserializeOwned;
use std::env;

lazy_static! {
    static ref SERVER_URL: String = match env::var("SERVER_URL") {
        Ok(url) => url,
        Err(_) => "http://localhost:5000".to_string(),
    };
    static ref CLIENT: Client = reqwest::Client::new();
}

pub async fn http_get<T: DeserializeOwned>(path: &str) -> Result<T, Error> {
    match (*CLIENT)
        .get(format!("{}{}", *SERVER_URL, path))
        .send()
        .await
    {
        Err(err) => return Err(err),
        Ok(res) => match res.json::<T>().await {
            Err(err) => return Err(err),
            Ok(data) => return Ok(data),
        },
    }
}
