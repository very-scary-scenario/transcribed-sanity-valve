use crate::read_albums::Album;
use pulldown_cmark::{Parser, html};
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::process::exit;
use tera::{Context, Tera};

#[derive(Serialize)]
struct IndexContext {
    readme: String,
    albums: Vec<Album>,
}

fn get_readme() -> String {
    let readme_source = fs::read_to_string("README.md").expect("failed to read readme");
    let parser = Parser::new(&readme_source);
    let mut readme_html = String::new();
    html::push_html(&mut readme_html, parser);

    readme_html
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

    fs::write(
        target_dir.join("index.html"),
        tera.render(
            "index.html",
            &Context::from_serialize(IndexContext {
                albums: albums,
                readme: get_readme(),
            }).expect("failed to build context"),
        )
        .expect("failed to render index"),
    )
    .expect("failed to write index.html");
}
