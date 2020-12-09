use select::document::Document;
use select::predicate::Name;
use url::ParseError as UrlParseError;
use url::Url;

fn main() -> eyre::Result<()> {
    let response = reqwest::blocking::get("https://www.rust-lang.com")?;
    let base_url = response.url().clone();
    let body = response.text()?;
    let doc = Document::from(body.as_str());

    for href in doc.find(Name("a")).filter_map(|a| a.attr("href")) {
        let url = match Url::parse(href) {
            Ok(url) => {
                println!("{:?}", url);
            }
            Err(UrlParseError::RelativeUrlWithoutBase) => {
                // `href` を絶対URLに変換する
                let url = base_url.join(href)?;
                println!("{:?}", url);
            }
            Err(e) => {}
        };
    }
    Ok(())
}
