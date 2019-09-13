use std::error::Error;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Sentence {
    tatoeba_id: u32,
    language: String,
    text: String,
}

pub fn parse<I, C>(input: I, mut callback: C) -> Result<(), Box<dyn Error>>
    where
        I: AsRef<Path>,
        C: FnMut(Sentence) -> Result<(), Box<dyn Error>>,
{
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(input)?;

    for result in rdr.deserialize() {
        let sentence: Sentence = result?;
        callback(sentence)?;
    }

    Ok(())
}

pub fn filter_language<I, O>(input: I, output: O, language: &str) -> Result<(), Box<dyn Error>>
    where
        I: AsRef<Path>,
        O: AsRef<Path>,
{
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(output)?;

    parse(input, |sentence| {
        if sentence.language == language {
            wtr.serialize(sentence)?;
        }
        Ok(())
    })?;

    wtr.flush()?;
    Ok(())
}
