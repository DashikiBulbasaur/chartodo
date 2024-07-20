use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tasks {
    pub todo: Vec<Task>,
    pub done: Vec<Task>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub task: String,
    #[serde(default = "return_default_date")]
    pub date: Option<String>,
    #[serde(default = "return_default_time")]
    pub time: Option<String>,
    #[serde(default = "return_default_repeat_number")]
    pub repeat_number: Option<u32>,
    #[serde(default = "return_default_repeat_unit")]
    pub repeat_unit: Option<String>,
    #[serde(default = "return_default_repeat_done")]
    pub repeat_done: Option<bool>,
    #[serde(default = "return_default_repeat_original_date")]
    pub repeat_original_date: Option<String>,
    #[serde(default = "return_default_repeat_original_time")]
    pub repeat_original_time: Option<String>,
}

// the following fns return a default value if the fields aren't present in the file
//
// note that I'm not sure how infallible this is. ideally i'd like it to add defaults based on what function is calling it,
// but idk how to do that rn.
//
// i'm also not sure if doing these fns repeatedly is needed. like perhaps it'd be better to just call serde(default), and
// that just defaults to None if it detects an Option. idk.

fn return_default_date() -> Option<String> {
    None
}

fn return_default_time() -> Option<String> {
    None
}

fn return_default_repeat_number() -> Option<u32> {
    // u32 is 4294967296 - 1
    None
}

fn return_default_repeat_unit() -> Option<String> {
    None
}

fn return_default_repeat_done() -> Option<bool> {
    None
}

fn return_default_repeat_original_date() -> Option<String> {
    None
}

fn return_default_repeat_original_time() -> Option<String> {
    None
}
