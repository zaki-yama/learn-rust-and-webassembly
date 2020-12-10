use reqwest::blocking::ClientBuilder;
use url::Url;
use web_crawler_book::LinkExtractor;

fn main() -> eyre::Result<()> {
    env_logger::init();

    let url = std::env::args()
        .nth(1)
        .unwrap_or(String::from("https://www.rust-lang.org"));
    let url = Url::parse(&url)?;
    let client = ClientBuilder::new().build()?;
    let extractor = LinkExtractor::from_client(client);

    let links = extractor.get_links(url)?;
    for link in links.iter() {
        println!("{}", link);
    }

    Ok(())
}
