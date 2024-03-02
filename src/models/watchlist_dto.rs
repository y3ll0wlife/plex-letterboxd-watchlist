#[derive(Debug, PartialEq)]
pub enum WatchlistSource {
    Letterboxd,
    Plex,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct WatchlistDto {
    pub title: String,
    pub year: usize,
    pub source: WatchlistSource,
    pub plex_movie_id: Option<String>,
}

impl WatchlistDto {
    pub fn init(source: WatchlistSource) -> WatchlistDto {
        WatchlistDto {
            title: String::new(),
            year: 0,
            source,
            plex_movie_id: None,
        }
    }
}
