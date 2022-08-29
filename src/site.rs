use crate::albums::Album;
use maud::{html, PreEscaped, Markup, DOCTYPE};
use pulldown_cmark::{html, Parser};
use std::fs;
use std::path::Path;
use std::process::exit;

const README_SOURCE: &'static str = include_str!("../README.md");
const STYLES: &'static str = include_str!("style.css");

fn slugify(input: &str) -> String {
    let mut output = String::new();

    for letter in input.to_ascii_lowercase().chars() {
        if letter == ' ' {
            output.push('-');
        } else if letter.is_ascii_lowercase() {
            output.push(letter);
        }
    }

    output
}

fn get_readme() -> String {
    let parser = Parser::new(&README_SOURCE);
    let mut readme_html = String::new();
    html::push_html(&mut readme_html, parser);

    readme_html
}

fn lyrics(albums: &Vec<Album>) -> Markup {
    return html!(
        @for album in albums {
            h2 #(slugify(&album.title)) { (album.title) }
            @for track in &album.tracks {
                section #(slugify(&track.title)) {
                    h3 { (format!("{}. {}", track.number, track.title)) }
                    @for line in &track.lyrics {
                        @for phrase in &line.phrases {
                            span .lyric title=(phrase.attribution) { (phrase.content) }
                            " "
                        }
                        br;
                    }
                }
            }
        }
    )
}

fn contents(albums: &Vec<Album>) -> Markup {
    html!(
        ul {
            @for album in albums {
                li {
                    a href=(format!("#{}", slugify(&album.title))) { (album.title) }
                    ol {
                        @for track in &album.tracks {
                            li {
                                a href=(format!("#{}", slugify(&track.title))) { (track.title) }
                            }
                        }
                    }
                }
            }
        }
    )
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

    let markup = html!(
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Transcribed Sanity Valve" }
                style { (PreEscaped(STYLES)) }
            }
            body {
                header {
                    (PreEscaped(get_readme()))
                    h2 { "Albums" }
                    section #"contents" { (contents(&albums)) }
                }
                article { (lyrics(&albums)) }
                footer {
                    p {
                        "a " a href="https://vscary.co/" { "Very Scary Scenario" } " production"
                    }
                    p {
                        a href="https://github.com/very-scary-scenario/transcribed-sanity-valve/" { "contributions welcome" }
                    }
                }
            }
        }
    );

    fs::write(target_dir.join("index.html"), markup.into_string())
        .expect("failed to write index.html");
}
