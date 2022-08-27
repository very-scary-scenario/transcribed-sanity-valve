use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;

lazy_static! {
    static ref ALIASES: HashMap<&'static str, &'static str> = HashMap::from([
        ("B", "Both"),
        ("G", "Gishnchips"),
        ("P", "Mos Prob"),
        ("J", "Jewdacris"),
        ("?", ""),
    ]);
}

#[derive(Serialize, PartialEq, Eq)]
pub struct Phrase {
    pub content: String,
    pub attribution: String,
}

#[derive(Serialize, PartialEq, Eq)]
pub struct Line {
    pub phrases: Vec<Phrase>,
}

impl Line {
    pub fn parse_lyrics(input: &str) -> Vec<Line> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^((\s+)?([^:]+): )?(.*)$").unwrap();
        }

        let mut lines = Vec::new();

        let mut attribution: &str = "";
        let mut line_phrases: Vec<Phrase> = Vec::new();

        for input_line in input.lines() {
            let mut line_is_over = true;
            let cap = RE.captures(input_line).expect("could not parse line");
            dbg!(&cap);
            let content = cap.get(4).expect("could not get line content").as_str();

            match cap.get(3) {
                Some(a) => {
                    // this line has attribution to a vocalist
                    attribution = a.as_str();

                    match ALIASES.get(attribution) {
                        Some(a) => attribution = a,
                        None => (),
                    }

                    match cap.get(2) {
                        Some(_) => {
                            // there is leading whitespace in this line
                            line_is_over = false
                        }
                        None => (),
                    }
                }
                None => (),
            }

            if line_is_over && line_phrases.len() > 0 {
                lines.push(Line {
                    phrases: line_phrases,
                });
                line_phrases = Vec::new();
            }

            line_phrases.push(Phrase {
                content: String::from(content),
                attribution: String::from(attribution),
            });
        }
        lines.push(Line {
            phrases: line_phrases,
        });
        lines
    }
}
