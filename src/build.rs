use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Album {
    title: String,
    year: u16,
}

impl Album {
    fn from_direntry(entry: fs::DirEntry) -> Album {
        let folder_name = entry.file_name().into_string().expect("could not convert folder name");
        let re_match = Regex::new(r"^(\d+) - (.*)$").unwrap().captures(&folder_name).expect("could not parse folder name");

        Album {
            title: String::from(&re_match[2]),
            year: re_match[1].parse().expect("could not find album name"),
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
    dbg!(get_albums());
}
