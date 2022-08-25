use std::fs;
use std::path::Path;
use std::process::exit;
use crate::read_albums::Album;
use serde::Serialize;
use tera::{Context, Tera};

#[derive(Serialize)]
struct IndexContext {
    albums: Vec<Album>,
}

pub fn build_site(albums: Vec<Album>) {
    let target_dir: &Path = Path::new("_html");

    match fs::remove_dir_all(target_dir) {
        Ok(()) => (),
        Err(e) => {
            println!("Not deleting existing build dir: {}", e)
        }
    }

    match fs::create_dir(target_dir) {
        Ok(()) => (),
        Err(e) => {
            println!("Could not create build directory: {}", e);
            exit(1);
        }
    }

    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            exit(1);
        }
    };

    fs::write(target_dir.join("index.html"),
        tera.render(
            "index.html",
            &Context::from_serialize(IndexContext { albums: albums })
                .expect("failed to build context"),
        )
        .expect("failed to render index")
    ).expect("failed to write index.html");
}
