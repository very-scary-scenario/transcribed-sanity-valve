use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Track {
    number: u8,
    title: String,
    lyrics: String,
}

impl Track {
    fn from_direntry(entry: fs::DirEntry) -> Option<Track> {
        let file_name = entry.file_name().into_string().expect("could not convert track filename");
        let re_match = match Regex::new(r"^(\d+)\. (.*)\.txt$").unwrap().captures(&file_name) {
            Some(m) => m,
            None => {
                println!("Ignoring {}", file_name);
                return None
            }
        };

        Some(Track {
            number: re_match[1].parse().expect("could not find track number"),
            title: String::from(&re_match[2]),
            lyrics: fs::read_to_string(entry.path()).expect("could not read lyrics"),
        })
    }
}

fn get_tracks(entry: fs::DirEntry) -> Vec<Track> {
    let mut tracks = Vec::new();

    for path in fs::read_dir(entry.path()).expect("could not list album directory") {
        let entry = path.expect("could not read track directory");
        match Track::from_direntry(entry) {
            Some(t) => tracks.push(t),
            None => (),
        }
    }

    tracks
}

#[derive(Debug)]
struct Album {
    year: u16,
    title: String,
    tracks: Vec<Track>,
}

impl Album {
    fn from_direntry(entry: fs::DirEntry) -> Album {
        let folder_name = entry.file_name().into_string().expect("could not convert folder name");
        let re_match = Regex::new(r"^(\d+) - (.*)$").unwrap().captures(&folder_name).expect("could not parse folder name");

        Album {
            year: re_match[1].parse().expect("could not find album year"),
            title: String::from(&re_match[2]),
            tracks: get_tracks(entry),
        }
    }
}

fn get_albums() -> Vec<Album> {
    let mut albums = Vec::new();

    for path in fs::read_dir("albums").expect("could not list albums directory") {
        let entry = path.expect("could not read album directory");
        albums.push(Album::from_direntry(entry))
    }

    albums
}

pub fn build_site() {
    let albums = get_albums();
    dbg!(albums);
}
