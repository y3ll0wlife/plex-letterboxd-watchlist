use reqwest::{header, Client, ClientBuilder};

use crate::models::{
    plex_search_dto::PlexSearchDto,
    watchlist_dto::{WatchlistDto, WatchlistSource},
};

#[derive(Debug)]
pub struct Plex {
    metadata_api_url: String,
    discover_api_url: String,
    client: Client,
}

impl Plex {
    pub fn init(token: String) -> Plex {
        Plex {
            metadata_api_url: String::from("https://metadata.provider.plex.tv"),
            discover_api_url: String::from("https://discover.provider.plex.tv"),
            client: Self::create_client(token.clone()),
        }
    }

    fn create_client(token: String) -> Client {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "X-Plex-Token",
            header::HeaderValue::from_str(token.as_str()).unwrap(),
        );

        ClientBuilder::new()
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .expect("failed to create client")
    }

    fn create_metadata_url(&self, path: &str) -> String {
        format!("{}/{}", self.metadata_api_url, path)
    }

    fn create_discover_url(&self, path: &str) -> String {
        format!("{}/{}", self.discover_api_url, path)
    }

    pub async fn fetch_watchlist(&self) -> Result<Vec<WatchlistDto>, reqwest::Error> {
        let url = self.create_metadata_url("/library/sections/watchlist/all?includeFields=title,type,year,ratingKey&includeElements=Guid&sort=watchlistedAt%3Adesc&type=1");

        let watchlist_response = self.client.get(&url).send().await?.text().await?;

        let mut watchlist = vec![];
        let mut movie = WatchlistDto::init(WatchlistSource::Plex);

        for token in xmlparser::Tokenizer::from(watchlist_response.as_str()) {
            let token = match token {
                Ok(value) => value,
                Err(_) => continue,
            };

            match token {
                xmlparser::Token::ElementStart {
                    prefix: _,
                    local,
                    span: _,
                } => {
                    if local.as_str().to_lowercase() != "video" {
                        movie = WatchlistDto::init(WatchlistSource::Plex);
                        continue;
                    }
                }
                xmlparser::Token::Attribute {
                    prefix: _,
                    local,
                    value,
                    span: _,
                } => match local.as_str() {
                    "year" => {
                        movie.year = value.as_str().parse().expect("failed to parse year value")
                    }
                    "title" => movie.title = value.to_string(),
                    "ratingKey" => movie.plex_movie_id = Some(value.to_string()),
                    _ => continue,
                },
                xmlparser::Token::ElementEnd { end: _, span: _ } => {
                    if movie == WatchlistDto::init(WatchlistSource::Plex) {
                        continue;
                    }

                    watchlist.push(movie);
                    movie = WatchlistDto::init(WatchlistSource::Plex);
                }
                _ => continue,
            }
        }

        Ok(watchlist)
    }

    pub async fn search(
        &self,
        movies_to_search: Vec<WatchlistDto>,
    ) -> Result<Vec<WatchlistDto>, reqwest::Error> {
        let mut result = vec![];

        for search_movie in movies_to_search {
            let url = self.create_discover_url(
                        format!("library/search?query={}&limit=30&searchTypes=movies&searchProviders=discover&includeMetadata=1", search_movie.title).as_str()
            );

            println!(
                "Searching for {} ({})",
                search_movie.title, search_movie.year
            );

            let search = self
                .client
                .get(&url)
                .header("Accept", "application/json, text/plain, */*")
                .send()
                .await?
                .json::<PlexSearchDto>()
                .await?;

            let search_result = search
                .media_container
                .search_results
                .iter()
                .find(|x| x.id == "external")
                .unwrap()
                .clone()
                .search_result;

            let movie = search_result.iter().find(|x| {
                x.metadata.title == search_movie.title && x.metadata.year == search_movie.year
            });

            match movie {
                Some(value) => {
                    println!(
                        "Found {} ({}) in Plex search, plex_movie_id='{}'",
                        search_movie.title, search_movie.year, value.metadata.rating_key
                    );
                    result.push(WatchlistDto {
                        title: search_movie.title,
                        year: search_movie.year,
                        source: WatchlistSource::Plex,
                        plex_movie_id: Some(value.metadata.rating_key.clone()),
                    })
                }
                None => {
                    println!(
                        "Did not find {} ({}) in Plex search",
                        search_movie.title, search_movie.year
                    );
                    continue;
                }
            }
        }

        Ok(result)
    }

    pub async fn add_to_watchlist(
        &self,
        movies_to_add: Vec<WatchlistDto>,
    ) -> Result<(), reqwest::Error> {
        for movie_add in movies_to_add {
            if movie_add.plex_movie_id.is_none() {
                continue;
            }

            let url = self.create_discover_url(
                format!(
                    "actions/addToWatchlist?ratingKey={}",
                    movie_add.plex_movie_id.unwrap()
                )
                .as_str(),
            );

            println!(
                "Adding {} ({}) to watchlist",
                movie_add.title, movie_add.year
            );

            let add = self.client.put(&url).send().await?;

            if add.status() == 200 {
                println!(
                    "Sucessfully added {} ({}) to the watchlist",
                    movie_add.title, movie_add.year
                );

                continue;
            }

            println!(
                "Something went wrong with {} ({}) error code: {}",
                movie_add.title,
                movie_add.year,
                add.status()
            );
        }

        Ok(())
    }
}
