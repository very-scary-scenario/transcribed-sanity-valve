use crate::lyrics::Line;
use chrono::NaiveDate;
use regex::Regex;
use std::cmp::Ordering;
use std::fs;

trait ReadFromDir {
    fn from_direntry(entry: fs::DirEntry) -> Option<Self>
    where
        Self: Sized;
}

fn read_dir<T: ReadFromDir + Ord>(path: &str) -> Vec<T> {
    let mut things = Vec::new();

    for path in fs::read_dir(path).expect("could not list directory") {
        let entry = path.expect("could not read album directory");
        match T::from_direntry(entry) {
            Some(t) => things.push(t),
            None => (),
        }
    }
    things.sort_unstable();
    things
}

#[derive(PartialEq, Eq)]
pub struct Track {
    pub number: u8,
    pub title: String,
    pub lyrics: Vec<Line>,
}

impl PartialOrd for Track {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.number.cmp(&other.number));
    }
}

impl Ord for Track {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.number.cmp(&other.number);
    }
}

impl ReadFromDir for Track {
    fn from_direntry(entry: fs::DirEntry) -> Option<Track> {
        let file_name = entry
            .file_name()
            .into_string()
            .expect("could not convert track filename");
        let re_match = match Regex::new(r"^(\d+)\. (.*)\.txt$")
            .unwrap()
            .captures(&file_name)
        {
            Some(m) => m,
            None => {
                println!("Ignoring {}", file_name);
                return None;
            }
        };

        Some(Track {
            number: re_match[1].parse().expect("could not find track number"),
            title: String::from(&re_match[2]),
            lyrics: Line::parse_lyrics(
                &fs::read_to_string(entry.path()).expect("could not read lyrics"),
            ),
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Album {
    pub date: NaiveDate,
    pub title: String,
    pub tracks: Vec<Track>,
}

impl ReadFromDir for Album {
    fn from_direntry(entry: fs::DirEntry) -> Option<Album> {
        let folder_name = entry
            .file_name()
            .into_string()
            .expect("could not convert folder name");
        let re_match = Regex::new(r"^(\d{4}-\d{2}-\d{2}) - (.*)$")
            .unwrap()
            .captures(&folder_name)
            .expect("could not parse folder name");

        let date = chrono::NaiveDate::parse_from_str(&re_match[1], "%F").expect("could not parse album release date");

        Some(Album {
            date: date,
            title: String::from(&re_match[2]),
            tracks: read_dir(
                &entry
                    .path()
                    .into_os_string()
                    .into_string()
                    .expect("could not convert folder path"),
            ),
        })
    }
}

pub fn read_albums() -> Vec<Album> {
    read_dir("albums")
}
