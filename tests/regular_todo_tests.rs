use anyhow::Context;
use assert_cmd::prelude::*;
use chartodo::functions::{json_file_structs::*, regular_tasks::regular_helpers::*};
use predicates::prelude::*;
use std::{path::PathBuf, process::Command};

// cargo test --test regular_todo_tests -- --test-threads=1

// these are taken from regular_helpers
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

mod aaa_do_this_first {
    use super::*;

    // these have been tested in other fns, these are just included here as a sanity check
    #[test]
    fn regular_tasks_path_is_correct() {
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

    #[test]
    fn regular_tasks_copy_path_is_correct() {
        let linux_path = "/.local/share/chartodo/regular_tasks_copy.json";
        // note: windows is supposed to have \
        let windows_path = "/AppData/Local/chartodo/regular_tasks_copy.json";
        let mac_path = "/Library/Application Support/chartodo/regular_tasks_copy.json";
        let mut got_regular_tasks_copy_path: bool = false;
        let regular_tasks_copy_path = regular_tasks_copy_path();
        let regular_tasks_copy_path = regular_tasks_copy_path.to_str().unwrap();

        if regular_tasks_copy_path.contains(linux_path) {
            got_regular_tasks_copy_path = true;
        } else if regular_tasks_copy_path.contains(windows_path) {
            got_regular_tasks_copy_path = true;
        } else if regular_tasks_copy_path.contains(mac_path) {
            got_regular_tasks_copy_path = true;
        }

        assert!(got_regular_tasks_copy_path);
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
}

mod regular_todo_add {
    use super::*;

    #[test]
    fn regular_todo_adding_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // add one
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("add").arg("regular-todo-item");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: regular-todo-item"));

        Ok(())
    }

    #[test]
    fn regular_todo_adding_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // add one
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("a").arg("regular-todo-item");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: regular-todo-item"));

        Ok(())
    }

    #[test]
    fn regular_todo_adding_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // add one
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("add").arg("regular-todo-item").arg("hello");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: regular-todo-item"));
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("2: hello"));

        Ok(())
    }

    #[test]
    fn regular_todo_adding_multiple_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // add one
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("a").arg("regular-todo-item").arg("hello");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: regular-todo-item"));
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("2: hello"));

        Ok(())
    }
}

/*
mod add_todo_item_tests {
    use super::*;

    #[test]
    fn adds_item_correctly() -> Result<(), Box<dyn std::error::Error>> {
        // test adding one item
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("add").arg("item");
        cmd.assert().success().stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: item\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("a").arg("item");
        cmd.assert().success().stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: item\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        // test adding multiple items
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("add").arg("item1").arg("item2");
        cmd.assert().success().stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: item1\n7: item2\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("a").arg("item1").arg("item2");
        cmd.assert().success().stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: item1\n7: item2\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        // test adding multiple invalid items
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("add")
            .arg("item1")
            .arg("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
            .arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: item1\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("a")
            .arg("item1")
            .arg("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
            .arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: item1\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        Ok(())
    }

    #[test]
    fn todo_item_to_add_isnt_specified() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("add");
        cmd.assert().try_failure()?.stderr(predicate::str::contains("Did not provide the todo item(s) to be added. Good example: chartodo add new-item, or chartodo add item next-item one-more-item. If you have questions, try chartodo help or chartodo --help"));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("a");
        // note: lowkey starting to realize i don't need predicate for a lot of these
        cmd.assert().try_failure()?.stderr(predicate::str::contains("Did not provide the todo item(s) to be added. Good example: chartodo a new-item, or chartodo a item next-item one-more-item. If you have questions, try chartodo help or chartodo --help"));

        Ok(())
    }

    #[test]
    fn adding_too_many_items_to_todo_list() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_almost_full_todo_list_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("add").arg("item").arg("item2");
        cmd.assert().success().stdout(predicate::str::contains(
            "The todo list is too full. Please try removing items or clearing it altogether. For more information, try chartodo help",
        ));

        let _ = create_almost_full_todo_list_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("a").arg("item").arg("item2");
        cmd.assert().success().stdout(predicate::str::contains(
            "The todo list is too full. Please try removing items or clearing it altogether. For more information, try chartodo help",
        ));

        Ok(())
    }
}

mod todo_item_to_done_tests {
    use super::*;

    #[test]
    fn todo_item_to_done_position_isnt_specified() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done");
        cmd.assert()
            .try_failure()?
            .stderr(predicate::str::contains("Did not provide the todo item(s) to be changed to done. Good example: chartodo done 3, or chartodo done 3 4 5. If you have questions, try chartodo help or chartodo --help"));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d");
        cmd.assert()
            .try_failure()?
            .stderr(predicate::str::contains("Did not provide the todo item(s) to be changed to done. Good example: chartodo d 3, or chartodo d 3 4 5. If you have questions, try chartodo help or chartodo --help"));

        Ok(())
    }

    #[test]
    fn correctly_check_todo_list_is_empty_when_marking_todo_item_as_done(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_empty_todo_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is currently empty, so there are no todo items that can be marked as done. Try adding items to the todo list. To see how, type chartodo help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is currently empty, so there are no todo items that can be marked as done. Try adding items to the todo list. To see how, type chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn trying_to_change_too_many_todo_items_to_done() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("15")
            .arg("15");
        cmd.assert()
            .try_success()?
            .stdout(predicate::str::contains("The number of your arguments meet or exceed the todo list's current filled length. At this point, you might as well just do chartodo doneall. For more information, try chartodo help"));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("15")
            .arg("15");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The number of your arguments meet or exceed the todo list's current filled length. At this point, you might as well just do chartodo doneall. For more information, try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn too_many_todos_to_done_as_it_would_exceed_done_list_length(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_almost_full_done_list_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("5").arg("4");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n-----\nDONE\n1: list\n2: todo",
        ));

        let _ = create_almost_full_done_list_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("1").arg("4");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: is\n2: the\n3: list\n-----\nDONE\n1: todo\n2: this",
        ));

        Ok(())
    }

    #[test]
    fn todo_item_moved_to_done_correctly() -> Result<(), Box<dyn std::error::Error>> {
        // test only one item
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list\n6: list",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list\n6: list",
        ));

        Ok(())
    }

    #[test]
    fn todo_items_moved_to_done_correctly() -> Result<(), Box<dyn std::error::Error>> {
        // test multiple items plus some invalids
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done")
            .arg("1")
            .arg("5")
            .arg("3")
            .arg("a")
            .arg("")
            .arg("0")
            .arg("15");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: is\n2: todo\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list\n6: list\n7: the\n8: this",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d")
            .arg("1")
            .arg("5")
            .arg("3")
            .arg("a")
            .arg("")
            .arg("0")
            .arg("15");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: is\n2: todo\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list\n6: list\n7: the\n8: this",
        ));

        Ok(())
    }
}

mod remove_todo_item_tests {
    use super::*;

    #[test]
    fn position_for_todo_item_to_be_removed_is_missing() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo");
        cmd.assert().try_failure()?.stderr(predicate::str::contains(
            "Did not provide the todo item(s) to be removed. Good example: chartodo rmtodo 3, or chartodo rmtodo 3 4 5. If you have more questions, try chartodo help or chartodo --help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt");
        cmd.assert().try_failure()?.stderr(predicate::str::contains(
            "Did not provide the todo item(s) to be removed. Good example: chartodo rmt 3, or chartodo rmt 3 4 5. If you have more questions, try chartodo help or chartodo --help",
        ));

        Ok(())
    }

    #[test]
    fn correctly_check_todo_list_is_empty_when_marking_todo_item_as_done(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_empty_todo_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is currently empty, so there are no todo items that can be removed. Try adding items to the todo list. To see how, type chartodo help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is currently empty, so there are no todo items that can be removed. Try adding items to the todo list. To see how, type chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn too_many_todo_items_to_remove() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo")
            .arg("1")
            .arg("1")
            .arg("3")
            .arg("2")
            .arg("4")
            .arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains("The number of your arguments meet or exceed the todo list's current filled length. At this point, you might as well just do chartodo cleartodo. For more information, try chartodo help"));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt")
            .arg("1")
            .arg("1")
            .arg("3")
            .arg("2")
            .arg("4")
            .arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains("The number of your arguments meet or exceed the todo list's current filled length. At this point, you might as well just do chartodo cleartodo. For more information, try chartodo help"));

        Ok(())
    }

    #[test]
    fn todo_item_removed_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        Ok(())
    }

    #[test]
    fn todo_items_removed_correctly() -> Result<(), Box<dyn std::error::Error>> {
        // and some invalids
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo")
            .arg("5")
            .arg("1")
            .arg("b")
            .arg("3")
            .arg("")
            .arg("0")
            .arg("15");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: is\n2: todo\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt")
            .arg("5")
            .arg("1")
            .arg("b")
            .arg("3")
            .arg("")
            .arg("0")
            .arg("15");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: is\n2: todo\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        Ok(())
    }
}

mod clear_todo_list_tests {
    use super::*;

    #[test]
    fn todo_list_is_already_empty_cleartodo() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_empty_todo_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("cleartodo");
        cmd.assert()
            .try_success()?
            .stdout(predicate::str::contains("The todo list is already empty."));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ct");
        cmd.assert()
            .try_success()?
            .stdout(predicate::str::contains("The todo list is already empty."));

        Ok(())
    }

    #[test]
    fn todo_list_was_cleared_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("cleartodo");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list was cleared.\n\nCHARTODO\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ct");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list was cleared.\n\nCHARTODO\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        Ok(())
    }
}

mod change_all_todos_to_done_tests {
    use super::*;

    #[test]
    fn todo_list_is_already_empty_doneall() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_empty_todo_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("doneall");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is empty, and so has no items that can be changed to done.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("da");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is empty, and so has no items that can be changed to done.",
        ));

        Ok(())
    }

    #[test]
    fn done_list_is_gonna_be_full_clear_it_first() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_almost_full_done_list_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("doneall");
        cmd.assert().try_success()?.stdout(predicate::str::contains("All todos were changed to done.\n\nCHARTODO\n-----\nDONE\n1: this\n2: is\n3: the\n4: todo\n5: list"));

        let _ = create_almost_full_done_list_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("da");
        cmd.assert().try_success()?.stdout(predicate::str::contains("All todos were changed to done.\n\nCHARTODO\n-----\nDONE\n1: this\n2: is\n3: the\n4: todo\n5: list"));

        Ok(())
    }

    #[test]
    fn all_todos_were_changed_to_done() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("doneall");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "All todos were changed to done.\n\nCHARTODO\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list\n6: this\n7: is\n8: the\n9: todo\n10: list",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("da");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "All todos were changed to done.\n\nCHARTODO\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list\n6: this\n7: is\n8: the\n9: todo\n10: list",
        ));

        Ok(())
    }
}

mod edit_todo_item_tests {
    use super::*;

    #[test]
    fn position_for_the_todo_item_to_be_edited_is_missing() -> Result<(), Box<dyn std::error::Error>>
    {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit");
        cmd.assert().try_failure()?.stderr(predicate::str::contains(
            "Error: Did not provide the todo item to be edited. Good example: chartodo edit 3 abc. If you have more questions, try chartodo help or chartodo --help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e");
        cmd.assert().try_failure()?.stderr(predicate::str::contains(
            "Error: Did not provide the todo item to be edited. Good example: chartodo e 3 abc. If you have more questions, try chartodo help or chartodo --help",
        ));

        Ok(())
    }

    #[test]
    fn correctly_check_todo_list_is_empty_when_editing_todo_item(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_empty_todo_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("5").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is currently empty, so there are no todo items that can be edited. Try adding items to the todo list. To see how, type chartodo help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("5").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is currently empty, so there are no todo items that can be edited. Try adding items to the todo list. To see how, type chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn too_many_or_missing_arguments_to_edit_todo_item() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("3");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify both the item's position and what to edit it to, and no more/less. Good example: chartodo edit 3 abc. Please try again, or try chartodo help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("3");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify both the item's position and what to edit it to, and no more/less. Good example: chartodo edit 3 abc. Please try again, or try chartodo help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("4").arg("5").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify both the item's position and what to edit it to, and no more/less. Good example: chartodo edit 3 abc. Please try again, or try chartodo help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("4").arg("5").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify both the item's position and what to edit it to, and no more/less. Good example: chartodo edit 3 abc. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn position_for_todo_item_to_be_edited_is_empty() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must provide the todo item's position that will be edited. Please try again, or try chartodo help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must provide the todo item's position that will be edited. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn todo_item_to_be_edited_to_is_empty() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("3").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify what the todo item will be edited to. Please try again, or try chartodo help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("abc").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify what the todo item will be edited to. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn position_is_not_a_number_or_not_u8_for_todo_item_to_be_edited(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("a").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the item's position that will be edited. Please specify a position between 1 and 5, or try chartodo help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("b").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the item's position that will be edited. Please specify a position between 1 and 5, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn position_for_the_todo_item_to_be_edited_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("0").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The item's position can't be zero. Please specify a position between 1 and 5, or try chartodo help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("0").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The item's position can't be zero. Please specify a position between 1 and 5, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn position_for_todo_item_to_be_edited_is_too_big() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("10").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position you specified is bigger than the todo list. Please specify a position between 1 and 5, or try chartodo help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("10").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position you specified is bigger than the todo list. Please specify a position between 1 and 5, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn todo_item_to_be_edited_to_is_too_long() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit")
            .arg("1")
            .arg("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "Editing a todo item to be longer than 30 characters is not allowed. Please try again, or try chartodo help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("1").arg("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "Editing a todo item to be longer than 30 characters is not allowed. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn todo_item_edited_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("5").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "Todo item 'list' was changed to 'abc'.\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: abc\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("5").arg("xyz");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "Todo item 'abc' was changed to 'xyz'.\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: xyz\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        Ok(())
    }
}
*/

mod zzz_do_this_last {
    use super::*;

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

/*
#[test]
fn zzz_do_this_last() {
    // note: this is just to reset the file after all the changes for my own convenience.
    // the fn also starts with zzz cuz rust runs the tests in alphabetical order, and I
    // want this to be the last one everytime
    let _ = create_test_file();
}
*/
