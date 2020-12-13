use crate::value::Value;
// use linked_hash_map::LinkedHashMap;
use linear_map::LinearMap;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type Map<K, V> = BTreeMap<K, V>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterData {
    pub description: String,
}

pub type Characters = Map<String, CharacterData>;
pub type State = Map<String, Value>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub passage: String,
    pub line: usize,
    pub state: State,
    pub characters: Characters,
}

pub type Dialogue = Map<String, String>;

pub type Branches<T> = LinearMap<String, Vec<T>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choices {
    pub choices: Map<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Goto {
    pub goto: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCmd {
    pub set: State,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PassageLine {
    Branches(Branches<PassageLine>),
    Choices(Choices),
    Goto(Goto),
    Text(String),
    SetCmd(SetCmd),
    Dialogue(Dialogue),
    Continue,
    InvalidChoice,
}

pub type Passage = Vec<PassageLine>;

pub type Story = Map<String, Passage>;
