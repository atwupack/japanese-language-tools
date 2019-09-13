use std::error::Error;
use std::process;

use serde::Deserialize;

use isolang::Language;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum TatoebaLanguage {
    KnownLanguage(Language),
    UnknownLanguage(String),
}

#[derive(Debug, Deserialize)]
struct Sentence {
    tatoeba_id: u32,
    language: TatoebaLanguage,
    text: String,
}

fn parse_tatoeba() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path("data/sentences.csv")?;
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let sentence: Sentence = result?;
        println!("{:?}", sentence);
//        if sentence.language == Language::Jpn {
//            println!("{:?}", sentence);
//        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = parse_tatoeba() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
