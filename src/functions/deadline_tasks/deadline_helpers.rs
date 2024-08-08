use crate::functions::json_file_structs::*;
use anyhow::Context;
use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

// linux: $HOME/.local/share/chartodo/deadline_tasks.json
// windows: C:\Users\some_user\AppData\Local\chartodo\deadline_tasks.json
// mac: /Users/some_user/Library/Application Support/chartodo/deadline_tasks.json

const CHARTODO_PATH: &str = "linux: $HOME/.local/share/chartodo/
    windows: C:/Users/your_user/AppData/Local/chartodo/
    mac: /Users/your_user/Library/Application Support/chartodo/";

fn path_to_deadline_tasks() -> PathBuf {
    // get the data dir XDG spec and return it with path to regular_tasks.json
    let mut deadline_tasks_path = dirs::data_dir()
        .context(
            "linux: couldn't get $HOME/.local/share/
                windows: couldn't get C:/Users/your_user/AppData/Local/
                mac: couldn't get /Users/your_user/Library/Application Support/

                those directories should exist for your OS. please double check that they do.",
        )
        .expect("something went wrong with fetching the user's data dirs");
    deadline_tasks_path.push("chartodo/deadline_tasks.json");

    deadline_tasks_path
}

pub fn deadline_tasks_create_dir_and_file_if_needed() {
    // get data dir and check if chartodo folder exists. if not, create it
    let mut deadline_tasks_path = dirs::data_dir()
        .context(
            "linux: couldn't get $HOME/.local/share/
                windows: couldn't get C:/Users/your_user/AppData/Local/
                mac: couldn't get /Users/your_user/Library/Application Support/",
        )
        .expect("something went wrong with fetching the user's data dirs");
    deadline_tasks_path.push("chartodo");

    // check if chartodo folder exists
    if !deadline_tasks_path.exists() {
        // note: this isn't create_dir_all cuz if god forbid the file paths leading up to it
        // somehow don't exist, i'd rather it just fail than to force create them
        std::fs::create_dir(deadline_tasks_path.clone())
            .context(
                "linux: couldn't create dir $HOME/.local/share/chartodo/
                windows: couldn't create dir C:/Users/your_user/AppData/Local/chartodo/
                mac: couldn't create dir /Users/your_user/Library/Application Support/chartodo/",
            )
            .expect("something went wrong with creating chartodo folder");
    }
    deadline_tasks_path.push("deadline_tasks.json");

    // create the file if it doesn't exist
    if !Path::new(&deadline_tasks_path).exists() {
        let deadline_tasks_json = File::create(deadline_tasks_path)
            .with_context(|| {
                format!(
                    "couldn't create deadline_tasks json file in the following dirs:
                {}",
                    CHARTODO_PATH
                )
            })
            .expect("couldn't create new deadline_tasks.json file");

        let fresh_deadline_tasks = r#"
        {
            "todo": [
                {
                    "task": "breathe-once-before-2099",
                    "date": "2099-01-01",
                    "time": "00:00",
                    "repeat_number": null,
                    "repeat_unit": null,
                    "repeat_done": null,
                    "repeat_original_date": null,
                    "repeat_original_time": null
                }
            ],
            "done": []
        }
        "#;

        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks).
            context(
                    "somehow the fucking data to put in the new deadline_tasks file wasn't correct. you should never be able to see this"
                ).
            expect("changing str to tasks struct failed");

        let mut write_to_file = BufWriter::new(deadline_tasks_json);
        serde_json::to_writer_pretty(&mut write_to_file, &fresh_deadline_tasks)
            .with_context(|| {
                format!(
                    "failed to write fresh deadline tasks to new deadline_tasks json file in:
            {}",
                    CHARTODO_PATH
                )
            })
            .expect("failed to write fresh deadline tasks to deadline_tasks json file");
    }
}

pub fn open_deadline_tasks_and_return_tasks_struct() -> Tasks {
    // open file and parse
    let deadline_tasks_file = File::open(path_to_deadline_tasks())
        .with_context(|| {
            format!(
                "couldn't open deadline_tasks.json in the following directories:
                {}",
                CHARTODO_PATH
            )
        })
        .expect("couldn't open deadline_tasks.json file");
    let deadline_tasks: Tasks = serde_json::from_reader(deadline_tasks_file)
        .with_context(|| {
            format!(
                "failed to parse struct from deadline_tasks.json in the following dirs:
                {}",
                CHARTODO_PATH
            )
        })
        .expect("failed to parse deadline_tasks.json as struct");

    deadline_tasks
}

pub fn write_changes_to_new_deadline_tasks(mut deadline_tasks: Tasks) {
    // sort before writing. this used to be sort_by_key w/ cloning
    deadline_tasks.todo.sort_by(|x, y| {
        match x.date.as_ref().unwrap().cmp(y.date.as_ref().unwrap()) {
            std::cmp::Ordering::Equal => x.time.as_ref().unwrap().cmp(y.time.as_ref().unwrap()),
            lesser_or_greater => lesser_or_greater,
        }
    });
    deadline_tasks.done.sort_by(|x, y| {
        match x.date.as_ref().unwrap().cmp(y.date.as_ref().unwrap()) {
            std::cmp::Ordering::Equal => x.time.as_ref().unwrap().cmp(y.time.as_ref().unwrap()),
            lesser_or_greater => lesser_or_greater,
        }
    });

    // write the changes to the new file
    let deadline_tasks_file = File::create(path_to_deadline_tasks())
        .with_context(|| {
            format!(
                "couldn't create new deadline_tasks.json file in the following directories:
{}",
                CHARTODO_PATH
            )
        })
        .expect("couldn't create new deadline_tasks.json");
    let mut write_to_file = BufWriter::new(deadline_tasks_file);
    serde_json::to_writer_pretty(&mut write_to_file, &deadline_tasks)
        .with_context(|| {
            format!(
                "failed to write changes to deadline_tasks.json in the following dirs:
    {}",
                CHARTODO_PATH
            )
        })
        .expect("failed to write changes to deadline_tasks.json");
}
