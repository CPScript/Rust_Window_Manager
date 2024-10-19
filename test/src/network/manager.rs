use reqwest::Error;

pub struct NetworkManager;

impl NetworkManager {
    pub async fn fetch_data(url: &str) -> Result<String, Error> {
        let response = reqwest::get(url).await?;
        let body = response.text().await?;
        Ok(body)
    }
}
