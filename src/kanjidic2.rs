use std::path::Path;
use std::fs::File;
use serde::{Deserialize};
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Kanjidic2 {
    #[serde(rename = "character", default)]
    pub characters: Vec<Character>,
}

#[derive(Debug, Deserialize)]
pub struct Character {
    pub literal: String,
    pub misc: Misc,
}

#[derive(Debug, Deserialize)]
pub struct Misc {
    pub grade: Option<u8>,
    pub jlpt: Option<u8>,
}


pub fn parse<I>(input: I) -> Result<Kanjidic2, Box<dyn Error>>
    where
        I: AsRef<Path>,
{
    let mut f = File::open(input)?;
    let kd2: Kanjidic2 = serde_xml_rs::from_reader(f)?;
    Ok(kd2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let kd = parse("data/kanjidic2.xml");
        println!("{:?}", kd);
    }
}