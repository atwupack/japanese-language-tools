use std::path::Path;
use std::fs::File;
use serde::{Deserialize};
use std::error::Error;
use postgres::{Connection, TlsMode};

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
    pub grade: Option<i32>,
    pub jlpt: Option<i32>,
}


pub fn parse<I>(input: I) -> Result<Kanjidic2, Box<dyn Error>>
    where
        I: AsRef<Path>,
{
    let mut f = File::open(input)?;
    let kd2: Kanjidic2 = serde_xml_rs::from_reader(f)?;
    Ok(kd2)
}

pub fn import_kanji<I>(input: I) -> Result<(), Box<dyn Error>>
    where
        I: AsRef<Path>,
{
    let conn = Connection::connect("postgres://jlt:jlt@localhost:5432/postgres", TlsMode::None)?;
    let stmt = conn.prepare("INSERT INTO kanji (literal, jlpt, grade) VALUES ($1, $2, $3)")?;

    let kanji = parse(input)?;

    for k in kanji.characters {
        stmt.execute(&[&k.literal, &k.misc.jlpt, &k.misc.grade])?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let kd = parse("data/kanjidic2.xml");
        println!("{:?}", kd);
    }

    #[test]
    fn test_import() {
        let result = import_kanji("data/kanjidic2.xml");
        if let Err(e) = result {
            println!("{}", e);
        }
    }
}