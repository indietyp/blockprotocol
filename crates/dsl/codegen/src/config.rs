use std::collections::{HashMap, HashSet};

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
enum Is {
    Trivia,
    BinaryOp,
    UnaryOp,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Kind {
    regex: Vec<String>,
    literal: String,

    is: HashSet<Is>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Syntax {}

#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
    kind: HashMap<String, Kind>,
    syntax: HashMap<String, Syntax>,
}
