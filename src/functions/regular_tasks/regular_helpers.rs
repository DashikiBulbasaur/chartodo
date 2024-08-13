use crate::functions::json_file_structs::*;
use anyhow::Context;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter},
    path::{Path, PathBuf},
};

// linux: $HOME/.local/share/chartodo/regular_tasks.json
// windows: C:\Users\some_user\AppData\Local\chartodo\regular_tasks.json
// mac: /Users/some_user/Library/Application Support/chartodo/regular_tasks.json

const CHARTODO_PATH: &str = "linux: $HOME/.local/share/chartodo/
    windows: C:/Users/your_user/AppData/Local/chartodo/
    mac: /Users/your_user/Library/Application Support/chartodo/";

fn path_to_regular_tasks() -> PathBuf {
    // get the data dir XDG spec and return it with path to regular_tasks.json
    let mut regular_tasks_path = dirs::data_dir()
        .context(
            "linux: couldn't get $HOME/.local/share/
                windows: couldn't get C:/Users/your_user/AppData/Local/
                mac: couldn't get /Users/your_user/Library/Application Support/

                those directories should exist for your OS. please double check that they do.",
        )
        .expect("something went wrong with fetching the user's data dirs");
    regular_tasks_path.push("chartodo/regular_tasks.json");

    regular_tasks_path
}

// the old file that the fn below is talking about is general_list.txt, an old format I used to use
// code related to it will be deleted once there are enough iterations and I can be 99% sure no one's using it

pub fn regular_tasks_create_dir_and_file_if_needed() {
    // get data dir and check if chartodo folder exists. if not, create it
    let mut regular_tasks_path = dirs::data_dir()
        .context(
            "linux: couldn't get $HOME/.local/share/
                windows: couldn't get C:/Users/your_user/AppData/Local/
                mac: couldn't get /Users/your_user/Library/Application Support/",
        )
        .expect("something went wrong with fetching the user's data dirs");
    regular_tasks_path.push("chartodo");

    // check if chartodo folder exists
    if !regular_tasks_path.exists() {
        // note: this isn't create_dir_all cuz if god forbid the file paths leading up to it
        // somehow don't exist, i'd rather it just fail than to force create them
        std::fs::create_dir(regular_tasks_path.clone())
            .context(
                "linux: couldn't create dir $HOME/.local/share/chartodo/
                windows: couldn't create dir C:/Users/your_user/AppData/Local/chartodo/
                mac: couldn't create dir /Users/your_user/Library/Application Support/chartodo/",
            )
            .expect("something went wrong with creating chartodo folder");
    }
    regular_tasks_path.push("regular_tasks.json");

    // check if the old file exists, and push its contents to new json
    let mut old_path = dirs::data_dir()
        .context(
            "linux: couldn't get $HOME/.local/share/
                windows: couldn't get C:/Users/your_user/AppData/Local/
                mac: couldn't get /Users/your_user/Library/Application Support/",
        )
        .expect("something went wrong with fetching the user's data dirs");
    old_path.push("chartodo");
    old_path.push("general_list.txt");

    if !Path::new(&regular_tasks_path).exists() && Path::new(&old_path).exists() {
        transfer_old_file_contents_to_new_json(&old_path, &regular_tasks_path);
    }

    // just to double check in case both the old file and the json exists
    if Path::new(&regular_tasks_path).exists() && Path::new(&old_path).exists() {
        std::fs::remove_file(&old_path)
            .with_context(|| {
                format!(
                    "old general_list.txt file exists and couldn't remove in the following dirs:
                {}",
                    CHARTODO_PATH
                )
            })
            .expect("couldn't remove old file");
    }

    // not sure if the other if conditions above are redundant. this one is to create the file if it doesn't exist
    if !Path::new(&regular_tasks_path).exists() {
        let regular_tasks_json = File::create(regular_tasks_path)
            .with_context(|| {
                format!(
                    "couldn't create regular_tasks json file in the following dirs:
                {}",
                    CHARTODO_PATH
                )
            })
            .expect("couldn't create new regular_tasks.json file");

        let fresh_regular_tasks = r#"
        {
            "todo": [
                {
                    "task": "this is the todo list",
                    "date": null,
                    "time": null,
                    "repeat_number": null,
                    "repeat_unit": null,
                    "repeat_done": null,
                    "repeat_original_date": null,
                    "repeat_original_time": null,
                }
            ],
            "done": [
                {
                    "task": "this is the done list",
                    "date": null,
                    "time": null,
                    "repeat_number": null,
                    "repeat_unit": null,
                    "repeat_done": null,
                    "repeat_original_date": null,
                    "repeat_original_time": null,
                }
            ]
        }
        "#;

        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                    "somehow the fucking data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
                ).
            expect("changing str to tasks struct failed");

        let mut write_to_file = BufWriter::new(regular_tasks_json);
        serde_json::to_writer_pretty(&mut write_to_file, &fresh_regular_tasks)
            .with_context(|| {
                format!(
                    "failed to write fresh regular tasks to new regular_tasks json file in:
            {}",
                    CHARTODO_PATH
                )
            })
            .expect("failed to write fresh regular tasks to regular_tasks json file");
    }
}

fn transfer_old_file_contents_to_new_json(old_path: &PathBuf, new_json: &PathBuf) {
    let file = File::open(old_path).with_context(|| format!("couldn't open old file in the following dirs:
{}", CHARTODO_PATH
)).expect("old file not found even though it was already checked that it exists. something went very wrong");

    let reader = BufReader::new(file);

    // separate the lists into vecs so i can do operations on them
    let mut file_buf: Vec<String> = vec![];
    let mut todo_buf: Vec<String> = vec![];
    let mut done_buf: Vec<String> = vec![];

    // the reason for doing this separately is that Rust complains if I use line to check for
    // conditions
    for line in reader.lines() {
        file_buf.push(line.expect("couldn't unwrap line and push to file_buf"));
    }

    // if this is 1, that means the todo list is done and the loop can push to done_buf
    let mut todo_done_demarcation = 0;
    // NB: max len for todo_buf is 15 and 30 for done_buf
    for line in file_buf {
        if line == "-----" {
            todo_done_demarcation = 1;
        } else {
            match todo_done_demarcation {
                0 => {
                    // only applies if the user manually modifies general_list.txt
                    // lines with more than 50 chars are ommitted
                    if todo_buf.len() < 15 && line.len() < 30 {
                        todo_buf.push(line.to_string());
                    }
                }
                _ => {
                    // only applies if the user manually modifies general_list.txt
                    // lines with more than 50 chars are ommitted
                    if done_buf.len() < 15 && line.len() < 30 {
                        done_buf.push(line.to_string());
                    }
                }
            }
        }
    }

    // create vec<task> for both todo and done
    let mut vec_of_todos: Vec<Task> = vec![];
    todo_buf.iter().for_each(|item| {
        let task = Task {
            task: item.to_string(),
            date: None,
            time: None,
            repeat_number: None,
            repeat_unit: None,
            repeat_done: None,
            repeat_original_date: None,
            repeat_original_time: None,
        };

        vec_of_todos.push(task);
    });

    let mut vec_of_dones: Vec<Task> = vec![];
    done_buf.iter().for_each(|item| {
        let task = Task {
            task: item.to_string(),
            date: None,
            time: None,
            repeat_number: None,
            repeat_unit: None,
            repeat_done: None,
            repeat_original_date: None,
            repeat_original_time: None,
        };

        vec_of_dones.push(task);
    });

    let regular_tasks: Tasks = Tasks {
        todo: vec_of_todos,
        done: vec_of_dones,
    };

    // write the old contents to new json file
    let regular_tasks_json = File::create(new_json)
        .with_context(|| {
            format!(
                "couldn't create regular_tasks json file in the following dirs:
            {}",
                CHARTODO_PATH
            )
        })
        .expect("couldn't create new regular_tasks.json file");
    let mut write_to_file = BufWriter::new(regular_tasks_json);
    serde_json::to_writer_pretty(&mut write_to_file, &regular_tasks)
        .with_context(|| {
            format!(
                "couldn't write old contens to regular_tasks json file in the following dirs:
            {}",
                CHARTODO_PATH
            )
        })
        .expect("failed to write old contents to new json file");

    // now that the old contents have been transferred, remove old file
    std::fs::remove_file(old_path)
        .with_context(|| {
            format!(
                "couldn't remove old general_list.txt in the following dirs:
    {}",
                CHARTODO_PATH
            )
        })
        .expect("anyhow's with_context failed?");
}

pub fn write_changes_to_new_regular_tasks(regular_tasks: Tasks) {
    // write the changes to the new file
    let regular_tasks_file = File::create(path_to_regular_tasks())
        .with_context(|| {
            format!(
                "couldn't create new regular_tasks.json file in the following directories:
{}",
                CHARTODO_PATH
            )
        })
        .expect("couldn't create new regular_tasks.json");
    let mut write_to_file = BufWriter::new(regular_tasks_file);
    serde_json::to_writer_pretty(&mut write_to_file, &regular_tasks)
        .with_context(|| {
            format!(
                "failed to write changes to regular_tasks.json in the following dirs:
    {}",
                CHARTODO_PATH
            )
        })
        .expect("failed to write changes to regular_tasks.json");
}

pub fn open_regular_tasks_and_return_tasks_struct() -> Tasks {
    // open file and parse
    let regular_tasks_file = File::open(path_to_regular_tasks())
        .with_context(|| {
            format!(
                "couldn't open regular_tasks.json in the following directories:
                {}",
                CHARTODO_PATH
            )
        })
        .expect("couldn't open regular_tasks.json file");
    let regular_tasks: Tasks = serde_json::from_reader(regular_tasks_file)
        .with_context(|| {
            format!(
                "failed to parse struct from regular_tasks.json in the following dirs:
                {}",
                CHARTODO_PATH
            )
        })
        .expect("failed to parse regular_tasks.json as struct");

    regular_tasks
}

#[cfg(test)]
mod regular_helpers_unit_tests {
    use super::*;

    #[test]
    fn aaaa_regular_tasks_clone_file() {
        // name is aaaa so it's done first
        // since we will be modifying the original file to run a test, the original data must be
        // preserved first
        let mut regular_tasks_copy_path = dirs::data_dir()
            .context(
                "linux: couldn't get $HOME/.local/share/
                    windows: couldn't get C:/Users/your_user/AppData/Local/
                    mac: couldn't get /Users/your_user/Library/Application Support/

                    those directories should exist for your OS. please double check that they do.",
            )
            .expect("something went wrong with fetching the user's data dirs");
        regular_tasks_copy_path.push("chartodo/regular_tasks_copy.json");

        std::fs::copy(path_to_regular_tasks(), regular_tasks_copy_path)
            .context("failed to copy regular_tasks.json to regular_tasks_copy.json")
            .expect("anyhow context failed");
    }

    #[test]
    fn zzzz_rename_copy_to_original() {
        // name is zzzz so it's done last
        // now that tests are done, rename the modified original and rename copy to original

        std::fs::remove_file(path_to_regular_tasks())
            .context("failed delete modified regular_tasks.json after running tests")
            .expect("anyhow context failed");

        let mut regular_tasks_copy = dirs::data_dir()
                    .context(
                        "linux: couldn't get $HOME/.local/share/
                            windows: couldn't get C:/Users/your_user/AppData/Local/
                            mac: couldn't get /Users/your_user/Library/Application Support/

                            those directories should exist for your OS. please double check that they do.",
                    )
                    .expect("something went wrong with fetching the user's data dirs");
        regular_tasks_copy.push("chartodo/regular_tasks_copy.json");
        std::fs::rename(regular_tasks_copy, path_to_regular_tasks())
            .context("failed to rename regular_tasks_copy to regular_tasks")
            .expect("anyhow context failed");
    }

    #[test]
    fn path_to_regular_tasks_is_correct() {
        let linux_path = "/.local/share/chartodo/regular_tasks.json";
        // note: windows is supposed to have \
        let windows_path = "/AppData/Local/chartodo/regular_tasks.json";
        let mac_path = "/Library/Application Support/chartodo/regular_tasks.json";
        let mut got_regular_tasks_path: bool = false;
        let regular_path = path_to_regular_tasks();
        let regular_path = regular_path.to_str().unwrap();

        if regular_path.contains(linux_path) {
            got_regular_tasks_path = true;
        } else if regular_path.contains(windows_path) {
            got_regular_tasks_path = true;
        } else if regular_path.contains(mac_path) {
            got_regular_tasks_path = true;
        }

        assert!(got_regular_tasks_path);
    }
}
