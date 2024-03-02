use reqwest::{header, Client, ClientBuilder};
use serde::{Deserialize, Serialize};

use crate::models::watchlist_dto::{WatchlistDto, WatchlistSource};

#[derive(Debug)]
pub struct Letterboxd {
    url: String,
    client: Client,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LetterboxdLoginDtoResponse {
    csrf: String,
    result: String,
}

impl Letterboxd {
    pub async fn init(username: String, password: String) -> Letterboxd {
        let letterboxd = Letterboxd {
            url: String::from("https://letterboxd.com"),
            client: Self::create_client(),
            username,
            password,
        };

        let result = letterboxd.login().await;

        if result.is_err() {
            panic!("failed to login to letterboxd account");
        }

        letterboxd
    }

    fn create_client() -> Client {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"),
        );

        ClientBuilder::new()
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .expect("failed to create client")
    }

    fn create_url(&self, path: &str) -> String {
        format!("{}/{}", self.url, path)
    }

    async fn login(&self) -> Result<LetterboxdLoginDtoResponse, reqwest::Error> {
        let url = self.create_url("user/login.do");

        let mut params: Vec<(&str, &str)> = vec![
            ("authenticationCode", ""),
            ("username", self.username.as_str()),
            ("password", self.password.as_str()),
            ("remember", "true"),
        ];

        let response = self
            .client
            .post(&url)
            .form(&params)
            .send()
            .await?
            .json::<LetterboxdLoginDtoResponse>()
            .await?;

        params.push(("__csrf", response.csrf.as_str()));

        let response = self
            .client
            .post(&url)
            .form(&params)
            .send()
            .await?
            .json::<LetterboxdLoginDtoResponse>()
            .await?;

        Ok(response)
    }

    pub async fn fetch_watchlist(&self) -> Result<Vec<WatchlistDto>, reqwest::Error> {
        let url = self.create_url(format!("{}/watchlist/export", self.username).as_str());

        let watchlist_response = self.client.get(&url).send().await?.text().await?;

        let mut watchlist = vec![];
        for movie in watchlist_response.lines().skip(1) {
            let mut split = movie.split(",").collect::<Vec<&str>>();
            if movie.contains("\"") {
                let start = movie.find(",\"").unwrap_or(0) + 2;
                let end = movie.find("\",").unwrap_or(movie.len());
                let movie_name = &movie[start..end];

                let normal_split = movie.split(",").collect::<Vec<&str>>();

                split = vec![
                    normal_split[0],
                    movie_name,
                    normal_split[normal_split.len() - 2],
                    normal_split.last().unwrap(),
                ]
            }

            let year = match split[2].parse::<usize>() {
                Ok(num) => num,
                Err(_) => 0,
            };

            watchlist.push(WatchlistDto {
                title: split[1].to_string(),
                year,
                source: WatchlistSource::Letterboxd,
                plex_movie_id: None,
            })
        }

        Ok(watchlist)
    }
}
