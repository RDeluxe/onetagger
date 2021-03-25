use std::error::Error;
use std::collections::HashMap;
use std::fs::read_dir;
use serde::{Deserialize, Serialize};
use crate::tag::{Tag, Field, AudioFileFormat};

pub struct QuickTag {}

impl QuickTag {

    //Load all supported files from folder
    pub fn load_files(path: &str) -> Result<Vec<QuickTagFile>, Box<dyn Error>> {
        let mut out = vec![];
        for entry in read_dir(path)? {
            //Check if valid
            if entry.is_err() {
                continue;
            }
            let entry = entry.unwrap();
            //Skip dirs
            if entry.path().is_dir() {
                continue;
            }
            //Load tags
            let path = entry.path();
            let path = path.to_str().unwrap();
            match QuickTagFile::from_path(&path) {
                Some(t) => out.push(t),
                None => error!("Error loading file: {}", path)
            }
        }

        Ok(out)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuickTagFile {
    path: String,
    format: AudioFileFormat,
    title: String,
    artists: Vec<String>,
    genres: Vec<String>,
    release_date: Option<String>,
    bpm: Option<i64>,
    rating: u8,
    tags: HashMap<String, Vec<String>>
}

impl QuickTagFile {
    //Load tags from path
    pub fn from_path(path: &str) -> Option<QuickTagFile> {
        let tag_wrap = Tag::load_file(path).ok()?;
        QuickTagFile::from_tag(path, &tag_wrap)
    }

    pub fn from_tag(path: &str, tag_wrap: &Tag) -> Option<QuickTagFile> {
        let tag = tag_wrap.tag()?;
        Some(QuickTagFile {
            path: path.to_string(),
            format: tag_wrap.format.clone(),
            title: tag.get_field(Field::Title)?.first()?.to_string(),
            artists: tag.get_field(Field::Artist)?,
            genres: tag.get_field(Field::Genre).unwrap_or(vec![]),
            rating: tag.get_rating().unwrap_or(0),
            release_date: tag.get_date(),
            bpm: match tag.get_field(Field::BPM) {
                Some(t) => t.first().unwrap_or(&"can't parse".to_string()).parse().ok(),
                None => None
            },
            tags: tag.all_tags()
        })
    }
}
