mod letterboxd;
mod models;
mod plex;

use dotenv::dotenv;
use letterboxd::Letterboxd;
use plex::Plex;
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let letterboxd = Letterboxd::init(
        std::env::var("LETTERBOXD_USERNAME").expect("LETTERBOXD_USERNAME must be set."),
        std::env::var("LETTERBOXD_PASSWORD").expect("LETTERBOXD_PASSWORD must be set."),
    )
    .await;
    let letterboxd_watchlist = letterboxd.fetch_watchlist().await.unwrap();

    let plex = Plex::init(std::env::var("PLEX_TOKEN").expect("PLEX_TOKEN must be set."));

    // let plex_watchlist = plex.fetch_watchlist().await.unwrap();
    let searched = plex.search(letterboxd_watchlist).await.unwrap();

    let _ = plex.add_to_watchlist(searched).await;

    Ok(())
}
