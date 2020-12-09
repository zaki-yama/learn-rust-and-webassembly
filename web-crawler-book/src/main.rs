use select::document::Document;
use select::predicate::Name;

fn main() -> eyre::Result<()> {
    let body = reqwest::blocking::get("https://www.rust-lang.com")?.text()?;

    let doc = Document::from(body.as_str());

    for a in doc.find(Name("a")) {
        println!("{:?}", a);
    }
    Ok(())
}
