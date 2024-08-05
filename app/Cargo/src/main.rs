use reqwest::blocking::Client;
use scraper::{Html, Selector};
use serde_json::json;
use std::fs::File;
use std::io::Result;

fn write_links_to_json(links: Vec<String>) -> Result<()> {
    let file = File::create("links.json")?;
    let json_links = json!(links);
    serde_json::to_writer(file, &json_links)?;
    Ok(())
}

fn main() -> Result<()> {
    let url = "https://thepiratebay10.xyz/torrent/58942514/The_Rocky_Horror_Picture_Show_(1975)_[1080p]_[BluRay]";
    let client = Client::new();
    let response = client.get(url)
        .send()
        .expect("Erreur lors de la requête")
        .text()
        .expect("Erreur lors de la lecture de la réponse");

    let document = Html::parse_document(&response);
    let selector = Selector::parse("a").unwrap();

    let mut links = Vec::new();
    for element in document.select(&selector) {
        let link = element.value().attr("href").unwrap_or("");
        if link.starts_with("magnet:") {
            links.push(link.to_string());
        }
    }

    write_links_to_json(links)?;
    Ok(())
}