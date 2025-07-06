use crate::functions::json_file_structs::*;
use anyhow::Context;
use std::{
    fs::File,
    io::BufWriter,
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

    // not sure if the other if conditions above are redundant. this one is to create the file if it doesn't exist
    if !Path::new(&regular_tasks_path).exists() {
        let regular_tasks_json = File::create(regular_tasks_path)
            .with_context(|| {
                format!(
                    "couldn't create regular_tasks json file in the following dirs:
                    {CHARTODO_PATH}"
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
                    "repeat_original_time": null
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
                    "repeat_original_time": null
                }
            ]
        }
        "#;

        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "the fresh data to put in the new regular_tasks file wasn't\
                correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        let mut write_to_file = BufWriter::new(regular_tasks_json);
        serde_json::to_writer_pretty(&mut write_to_file, &fresh_regular_tasks)
            .with_context(|| {
                format!(
                    "failed to write fresh regular tasks to new regular_tasks json file in:
                    {CHARTODO_PATH}"
                )
            })
            .expect("failed to write fresh regular tasks to regular_tasks json file");
    }
}

pub fn write_changes_to_new_regular_tasks(regular_tasks: Tasks) {
    // write the changes to the new file
    let regular_tasks_file = File::create(path_to_regular_tasks())
        .with_context(|| {
            format!(
                "couldn't create new regular_tasks.json file in the following\
                directories:
                {CHARTODO_PATH}"
            )
        })
        .expect("couldn't create new regular_tasks.json");
    let mut write_to_file = BufWriter::new(regular_tasks_file);
    serde_json::to_writer_pretty(&mut write_to_file, &regular_tasks)
        .with_context(|| {
            format!(
                "failed to write changes to regular_tasks.json in the following dirs:
                {CHARTODO_PATH}"
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
                {CHARTODO_PATH}"
            )
        })
        .expect("couldn't open regular_tasks.json file");

    // this is to check if somehow the file exists but there is nothing in it
    // if there is nothing in it, write some data
    let regular_tasks: Tasks = match serde_json::from_reader(regular_tasks_file) {
        Ok(tasks) => tasks,
        Err(_) => {
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
                        "repeat_original_time": null
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
                        "repeat_original_time": null
                    }
                ]
            }
            "#;

            let open_regular_tasks_file = File::open(path_to_regular_tasks())
                .with_context(|| {
                    format!(
                        "couldn't open regular_tasks.json in the following directories:
                        {CHARTODO_PATH}"
                    )
                })
                .expect("couldn't open regular_tasks.json file");
            let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
                .context(
                    "the fresh data to put in an empty regular_tasks file
                    wasn't correct. you should never be able to see this",
                )
                .expect("changing str to tasks struct failed");

            let mut write_to_file = BufWriter::new(open_regular_tasks_file);
            serde_json::to_writer_pretty(&mut write_to_file, &fresh_regular_tasks)
                .with_context(|| {
                    format!(
                        "failed to write fresh regular tasks to new regular_tasks json file in:
                        {CHARTODO_PATH}"
                    )
                })
                .expect("failed to write fresh regular tasks to regular_tasks json file");

            fresh_regular_tasks
        }
    };

    regular_tasks
}

// cargo test regular_helpers_unit_tests -- --test-threads=1
#[cfg(test)]
mod regular_helpers_unit_tests {
    use super::*;

    fn regular_tasks_copy_path() -> PathBuf {
        // get the path for regular_tasks_copy.json, which will be used to hold the original contents
        // of regular_tasks.json while it's getting modified
        let mut regular_tasks_copy_path = dirs::data_dir()
            .context(
                "linux: couldn't get $HOME/.local/share/
                    windows: couldn't get C:/Users/your_user/AppData/Local/
                    mac: couldn't get /Users/your_user/Library/Application Support/

                    those directories should exist for your OS. please double check that they do.",
            )
            .expect("something went wrong with fetching the user's data dirs");
        regular_tasks_copy_path.push("chartodo/regular_tasks_copy.json");

        regular_tasks_copy_path
    }

    #[test]
    fn regular_tasks_copy_path_is_correct() {
        // funny that i'm testing a helper fn inside a mod that's supposed to test fns outside of it
        let linux_path = "/.local/share/chartodo/regular_tasks_copy.json";
        // note: windows is supposed to have \
        let windows_path = "/AppData/Local/chartodo/regular_tasks_copy.json";
        let mac_path = "/Library/Application Support/chartodo/regular_tasks_copy.json";
        let mut got_regular_tasks_copy_path: bool = false;
        let regular_tasks_copy_path = regular_tasks_copy_path();
        let regular_tasks_copy_path = regular_tasks_copy_path.to_str().unwrap();

        if regular_tasks_copy_path.contains(linux_path)
            || regular_tasks_copy_path.contains(windows_path)
            || regular_tasks_copy_path.contains(mac_path)
        {
            got_regular_tasks_copy_path = true;
        }

        assert!(got_regular_tasks_copy_path);
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

        if regular_path.contains(linux_path)
            | regular_path.contains(windows_path)
            | regular_path.contains(mac_path)
        {
            got_regular_tasks_path = true;
        }

        assert!(got_regular_tasks_path);
    }

    #[test]
    fn aaaa_regular_tasks_clone_file() {
        // name is aaaa so it's done first
        // since we will be modifying the original file to run a test, the original data must be
        // preserved first
        std::fs::File::create(regular_tasks_copy_path())
            .context("failed to create regular_tasks_copy.json")
            .expect("failed to create a copy during unit test");

        std::fs::copy(path_to_regular_tasks(), regular_tasks_copy_path())
            .context("failed to copy regular_tasks.json to regular_tasks_copy.json")
            .expect("failed to copy original file to copy file during unit test");
    }

    #[test]
    fn opening_regular_tasks_is_correct() {
        // write new struct to file rust
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
                    "repeat_original_time": null
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
                    "repeat_original_time": null
                }
            ]
        }
        "#;

        let open_regular_tasks_file = File::create(path_to_regular_tasks())
            .with_context(|| {
                format!(
                    "couldn't open regular_tasks.json in the following directories:
                    {CHARTODO_PATH}"
                )
            })
            .expect("couldn't open regular_tasks.json file");
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new\
                regular_tasks file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        let mut write_to_file = BufWriter::new(open_regular_tasks_file);
        serde_json::to_writer_pretty(&mut write_to_file, &fresh_regular_tasks)
            .with_context(|| {
                format!(
                    "failed to write fresh regular tasks to new regular_tasks json file in:
                    {CHARTODO_PATH}"
                )
            })
            .expect("failed to write fresh regular tasks to regular_tasks json file");

        // after writing struct to file, read it and check that it spits out the same struct
        let test_struct = open_regular_tasks_and_return_tasks_struct();

        assert_eq!(test_struct, fresh_regular_tasks);
    }

    #[test]
    fn zzzz_rename_copy_to_original() {
        // name is zzzz so it's done last
        // now that tests are done, remove the modified original and rename copy to original

        std::fs::remove_file(path_to_regular_tasks())
            .context("failed delete modified regular_tasks.json after running tests")
            .expect("failed to delete regular_tasks.json after regular_helpers unit tests");

        std::fs::rename(regular_tasks_copy_path(), path_to_regular_tasks())
            .context("failed to rename regular_tasks_copy to regular_tasks")
            .expect("failed to rename regular_tasks_copy to regular_tasks after tests were done");
    }
}
