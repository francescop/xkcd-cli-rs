use crate::models;

const BASE_URL: &str = "https://xkcd.com";

pub struct Client {
    client: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new() -> Client {
        Client {
            client: reqwest::Client::new(),
            base_url: BASE_URL.to_string(),
        }
    }

    pub async fn fetch(
        &self,
        comic_number: &str,
        _save: bool,
    ) -> Result<models::Comic, reqwest::Error> {
        let final_url = format!("{}/{}/info.0.json", &self.base_url, comic_number);
        let res: models::ComicResponse = self.client.get(&final_url).send().await?.json().await?;

        let comic = models::comic(&res);

        Ok(comic)
    }
}
