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
    // get the data dir XDG spec and return it with path to deadline_tasks.json
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
                    "the fresh data to put in the new deadline_tasks file wasn't correct. you should never be able to see this"
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

    // this is to check if somehow the file exists but there is nothing in it
    // if there is nothing in it, write some data
    let deadline_tasks: Tasks = match serde_json::from_reader(deadline_tasks_file) {
        Ok(tasks) => tasks,
        Err(_) => {
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

            let open_deadline_tasks_file = File::open(path_to_deadline_tasks())
                .with_context(|| {
                    format!(
                        "couldn't open deadline_tasks.json in the following directories:
                        {}",
                        CHARTODO_PATH
                    )
                })
                .expect("couldn't open deadline_tasks.json file");
            let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks).
                context(
                        "the fresh data to put in an empty deadline_tasks file wasn't correct. you should never be able to see this"
                    ).
                expect("changing str to tasks struct failed");

            let mut write_to_file = BufWriter::new(open_deadline_tasks_file);
            serde_json::to_writer_pretty(&mut write_to_file, &fresh_deadline_tasks)
                .with_context(|| {
                    format!(
                        "failed to write fresh deadline tasks to new deadline_tasks json file in:
                {}",
                        CHARTODO_PATH
                    )
                })
                .expect("failed to write fresh deadline tasks to deadline_tasks json file");

            fresh_deadline_tasks
        }
    };

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

// cargo test deadline_helpers_unit_tests -- --test-threads=1
#[cfg(test)]
mod deadline_helpers_unit_tests {
    use super::*;

    fn deadline_tasks_copy_path() -> PathBuf {
        // get the path for deadline_tasks_copy.json, which will be used to hold the original contents
        // of deadline_tasks.json while it's getting modified
        let mut deadline_tasks_copy_path = dirs::data_dir()
            .context(
                "linux: couldn't get $HOME/.local/share/
                    windows: couldn't get C:/Users/your_user/AppData/Local/
                    mac: couldn't get /Users/your_user/Library/Application Support/

                    those directories should exist for your OS. please double check that they do.",
            )
            .expect("something went wrong with fetching the user's data dirs");
        deadline_tasks_copy_path.push("chartodo/deadline_tasks_copy.json");

        deadline_tasks_copy_path
    }

    #[test]
    fn deadline_tasks_copy_path_is_correct() {
        // funny that i'm testing a helper fn inside a mod that's supposed to test fns outside of it
        let linux_path = "/.local/share/chartodo/deadline_tasks_copy.json";
        // note: windows is supposed to have \
        let windows_path = "/AppData/Local/chartodo/deadline_tasks_copy.json";
        let mac_path = "/Library/Application Support/chartodo/deadline_tasks_copy.json";
        let mut got_deadline_tasks_copy_path: bool = false;
        let deadline_tasks_copy_path = deadline_tasks_copy_path();
        let deadline_tasks_copy_path = deadline_tasks_copy_path.to_str().unwrap();

        if deadline_tasks_copy_path.contains(linux_path) {
            got_deadline_tasks_copy_path = true;
        } else if deadline_tasks_copy_path.contains(windows_path) {
            got_deadline_tasks_copy_path = true;
        } else if deadline_tasks_copy_path.contains(mac_path) {
            got_deadline_tasks_copy_path = true;
        }

        assert!(got_deadline_tasks_copy_path);
    }

    #[test]
    fn path_to_deadline_tasks_is_correct() {
        let linux_path = "/.local/share/chartodo/deadline_tasks.json";
        // note: windows is supposed to have \
        let windows_path = "/AppData/Local/chartodo/deadline_tasks.json";
        let mac_path = "/Library/Application Support/chartodo/deadline_tasks.json";
        let mut got_deadline_tasks_path: bool = false;
        let deadline_path = path_to_deadline_tasks();
        let deadline_path = deadline_path.to_str().unwrap();

        if deadline_path.contains(linux_path) {
            got_deadline_tasks_path = true;
        } else if deadline_path.contains(windows_path) {
            got_deadline_tasks_path = true;
        } else if deadline_path.contains(mac_path) {
            got_deadline_tasks_path = true;
        }

        assert!(got_deadline_tasks_path);
    }

    #[test]
    fn aaaa_deadline_tasks_clone_file() {
        // name is aaaa so it's done first
        // since we will be modifying the original file to run a test, the original data must be
        // preserved first
        std::fs::File::create(deadline_tasks_copy_path())
            .context("failed to create deadline_tasks_copy.json")
            .expect("failed to create a copy during unit test");

        std::fs::copy(path_to_deadline_tasks(), deadline_tasks_copy_path())
            .context("failed to copy deadline_tasks.json to deadline_tasks_copy.json")
            .expect("failed to copy original file to copy file during unit test");
    }

    #[test]
    fn opening_deadline_tasks_is_correct() {
        // write new struct to file rust
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

        let open_deadline_tasks_file = File::create(path_to_deadline_tasks())
            .with_context(|| {
                format!(
                    "couldn't open deadline_tasks.json in the following directories:
                    {}",
                    CHARTODO_PATH
                )
            })
            .expect("couldn't open deadline_tasks.json file");
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks).
            context(
                    "during testing: the fresh data to put in the new deadline_tasks file wasn't correct. you should never be able to see this"
                ).
            expect("changing str to tasks struct failed");

        let mut write_to_file = BufWriter::new(open_deadline_tasks_file);
        serde_json::to_writer_pretty(&mut write_to_file, &fresh_deadline_tasks)
            .with_context(|| {
                format!(
                    "failed to write fresh deadline tasks to new deadline_tasks json file in:
            {}",
                    CHARTODO_PATH
                )
            })
            .expect("failed to write fresh deadline tasks to deadline_tasks json file");

        // after writing struct to file, read it and check that it spits out the same struct
        let test_struct = open_deadline_tasks_and_return_tasks_struct();

        assert_eq!(test_struct, fresh_deadline_tasks);
    }

    #[test]
    fn zzzz_rename_copy_to_original() {
        // name is zzzz so it's done last
        // now that tests are done, rename the modified original and rename copy to original

        std::fs::remove_file(path_to_deadline_tasks())
            .context("failed delete modified deadline_tasks.json after running tests")
            .expect("failed to delete deadline_tasks.json after deadline_helpers unit tests");

        std::fs::rename(deadline_tasks_copy_path(), path_to_deadline_tasks())
            .context("failed to rename deadline_tasks_copy to deadline_tasks")
            .expect("failed to rename deadline_tasks_copy to deadline_tasks after tests were done");
    }
}
