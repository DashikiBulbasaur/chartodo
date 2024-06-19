use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{fs::File, io::Write, process::Command};

// note: to run these tests, do cargo test --test outputs -- --test-threads=1
// note that it seems to be 90% working with cargo test --test outputs, though it may just be a
// coincidence. not running it on one thread prevents the file reset at the end

// note: to run only integration tests, do cargo test --test '*'. For this program's case, probably
// good to also add test-threads=1

fn create_test_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = dirs::data_dir().expect("could not get path $HOME/.local/share/");
    path.push("chartodo");

    // note: this is just me being careful
    if !path.exists() {
        let _ = std::fs::create_dir(path.clone());
    }
    path.push("general_list.txt");

    let mut test_file = File::create(path)?;
    test_file.write_all(
        b"CHARTODO\nthis\nis\nthe\ntodo\nlist\n-----\nDONE\nthis\nis\nthe\ndone\nlist",
    )?;

    Ok(())
}

fn create_empty_todo_test_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = dirs::data_dir().expect("could not get path $HOME/.local/share/");
    path.push("chartodo");

    // note: this is just me being careful
    if !path.exists() {
        let _ = std::fs::create_dir(path.clone());
    }
    path.push("general_list.txt");

    let mut test_file = File::create(path)?;
    test_file.write_all(b"CHARTODO\n-----\nDONE\nthis\nis\nthe\ndone\nlist")?;

    Ok(())
}

fn create_empty_done_test_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = dirs::data_dir().expect("could not get path $HOME/.local/share/");
    path.push("chartodo");

    // note: this is just me being careful
    if !path.exists() {
        let _ = std::fs::create_dir(path.clone());
    }
    path.push("general_list.txt");

    let mut test_file = File::create(path)?;
    test_file.write_all(b"CHARTODO\nthis\nis\nthe\ntodo\nlist\n-----\nDONE")?;

    Ok(())
}

fn create_both_lists_empty_test_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = dirs::data_dir().expect("could not get path $HOME/.local/share/");
    path.push("chartodo");

    // note: this is just me being careful
    if !path.exists() {
        let _ = std::fs::create_dir(path.clone());
    }
    path.push("general_list.txt");

    let mut test_file = File::create(path)?;
    test_file.write_all(b"CHARTODO\n-----\nDONE")?;

    Ok(())
}

#[test]
fn list_prints_correctly() -> Result<(), Box<dyn std::error::Error>> {
    // note: I really don't like doing it this way, but the program only ever accesses one file
    // (unless I decide to expand), and so the only way to do integration testing is to access that
    // one file again and again
    let _ = create_test_file();

    let mut cmd = Command::cargo_bin("chartodo")?;
    cmd.arg("list");
    cmd.assert().success().stdout(predicate::str::contains(
        "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
    ));

    Ok(())
}

mod add_todo_item_tests {
    use super::*;

    #[test]
    fn adds_item_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("add").arg("item");
        cmd.assert().success().stdout(predicate::str::contains(
            "'item' was added to todo\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: item\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("a").arg("item");
        cmd.assert().success().stdout(predicate::str::contains(
            "'item' was added to todo\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: item\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        Ok(())
    }

    #[test]
    fn empty_add_item() -> Result<(), Box<dyn std::error::Error>> {
        // note: I don't know how this would ever activate. On main, it panics if there's no item to be
        // added. I guess this would activate if a person pasted a no-character/empty string to the
        // console?

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("add").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "Items to be added to the todo list cannot be empty. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("a").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "Items to be added to the todo list cannot be empty. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn item_to_be_added_is_too_long() -> Result<(), Box<dyn std::error::Error>> {
        // note: the character limit for the list is 150
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("add").arg("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The maximum length of an item is 150 characters. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("a").arg("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The maximum length of an item is 150 characters. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }
}

mod todo_item_is_done_tests {
    use super::*;

    #[test]
    fn todo_item_to_be_marked_as_done_is_not_specified() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position. A good example would be: 'chartodo done 3'. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position. A good example would be: 'chartodo done 3'. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn position_is_not_a_number_or_not_u8_for_the_todo_item_to_be_marked_as_done(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("a");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo done 3'. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("a");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo done 3'. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn correctly_check_todo_list_is_empty_when_marking_todo_item_as_done(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_empty_todo_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is currently empty, so there are no todo items that can be marked as done. Try adding items to the todo list. To see how, type 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is currently empty, so there are no todo items that can be marked as done. Try adding items to the todo list. To see how, type 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn position_for_the_todo_item_to_be_marked_as_done_is_zero(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("0");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("0");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn position_for_todo_item_to_be_marked_as_done_is_too_big(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("10");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is smaller than your specified position; therefore, the item you want to mark as done doesn't exist. The position has to be 5 or lower. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("10");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is smaller than your specified position; therefore, the item you want to mark as done doesn't exist. The position has to be 5 or lower. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn todo_item_moved_to_done_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "'list' was marked as done\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list\n6: list",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "'list' was marked as done\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list\n6: list",
        ));

        Ok(())
    }
}

mod remove_todo_item_tests {
    use super::*;

    #[test]
    fn position_for_todo_item_to_be_removed_is_not_specified(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position that will be removed. A good example would be: 'chartodo rmtodo 3'. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position that will be removed. A good example would be: 'chartodo rmtodo 3'. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn position_is_not_a_number_or_not_u8_for_todo_item_to_be_removed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo").arg("a");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position that will be removed, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo rmtodo 3'. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt").arg("a");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position that will be removed, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo rmtodo 3'. Please try again, or try 'chartodo help'.",
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
            "The todo list is currently empty, so there are no todo items that can be removed. Try adding items to the todo list. To see how, type 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is currently empty, so there are no todo items that can be removed. Try adding items to the todo list. To see how, type 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn position_for_the_todo_item_to_be_removed_is_zero() -> Result<(), Box<dyn std::error::Error>>
    {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo").arg("0");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt").arg("0");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn position_for_todo_item_to_be_removed_is_too_big() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo").arg("10");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is smaller than your specified position; therefore, the item you want to remove doesn't exist. The position has to be 5 or lower. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt").arg("10");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is smaller than your specified position; therefore, the item you want to remove doesn't exist. The position has to be 5 or lower. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn todo_item_removed_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "'list' was removed from todo\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "'list' was removed from todo\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
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

mod clear_done_list_tests {
    use super::*;

    #[test]
    fn done_list_is_already_empty_cleartodo() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_empty_done_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("cleardone");
        cmd.assert()
            .try_success()?
            .stdout(predicate::str::contains("The done list is already empty."));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("cd");
        cmd.assert()
            .try_success()?
            .stdout(predicate::str::contains("The done list is already empty."));

        Ok(())
    }

    #[test]
    fn done_list_was_cleared_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("cleardone");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list was cleared.\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n-----\nDONE",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("cd");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list was cleared.\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n-----\nDONE",
        ));

        Ok(())
    }
}

mod clear_both_list_tests {
    use super::*;

    #[test]
    fn both_list_are_already_empty_cleartodo() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_both_lists_empty_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("clearall");
        cmd.assert()
            .try_success()?
            .stdout(predicate::str::contains("The todo and done lists are already empty."));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ca");
        cmd.assert()
            .try_success()?
            .stdout(predicate::str::contains("The todo and done lists are already empty."));

        Ok(())
    }

    #[test]
    fn both_lists_were_cleared_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("clearall");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo and done lists were cleared.\n\nCHARTODO\n-----\nDONE",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ca");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo and done lists were cleared.\n\nCHARTODO\n-----\nDONE",
        ));

        Ok(())
    }
}

mod remove_done_item_tests {
    use super::*;

    #[test]
    fn position_for_done_item_to_be_removed_is_not_specified(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmdone").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the done item's position that will be removed. A good example would be: 'chartodo rmdone 3'. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the done item's position that will be removed. A good example would be: 'chartodo rmdone 3'. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn position_is_not_a_number_or_not_u8_for_done_item_to_be_removed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmdone").arg("a");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the done item's position that will be removed, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo rmdone 3'. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd").arg("a");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the done item's position that will be removed, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo rmdone 3'. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn correctly_check_done_list_is_empty_when_removing_done_item(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_empty_done_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmdone").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is already empty, so there are no done items that can be removed."));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is already empty, so there are no done items that can be removed.",
        ));

        Ok(())
    }

    #[test]
    fn position_for_the_done_item_to_be_removed_is_zero() -> Result<(), Box<dyn std::error::Error>>
    {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmdone").arg("0");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd").arg("0");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn position_for_done_item_to_be_removed_is_too_big() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmdone").arg("10");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is smaller than your specified position; therefore, the item you want to remove doesn't exist. The position has to be 5 or lower. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd").arg("10");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is smaller than your specified position; therefore, the item you want to remove doesn't exist. The position has to be 5 or lower. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn done_item_removed_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmdone").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "'list' was removed from done\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n-----\nDONE\n1: this\n2: is\n3: the\n4: done",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "'list' was removed from done\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n-----\nDONE\n1: this\n2: is\n3: the\n4: done",
        ));

        Ok(())
    }
}

mod reverse_done_item_back_to_todo_tests {
    use super::*;

    #[test]
    fn position_for_done_item_to_be_reversed_is_not_specified(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdone").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the done item's position that will be reversed. A good example would be: 'chartodo notdone 3', and if there was a done item at position 3, it would be reversed back to a todo item. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the done item's position that will be reversed. A good example would be: 'chartodo notdone 3', and if there was a done item at position 3, it would be reversed back to a todo item. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn position_is_not_a_number_or_not_u8_for_done_item_to_be_reversed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdone").arg("a");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the done item's position that will be reversed, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo notdone 3', and if there was a done item at position 3, it would be reversed back to a todo item. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd").arg("a");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the done item's position that will be reversed, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo notdone 3', and if there was a done item at position 3, it would be reversed back to a todo item. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn correctly_check_done_list_is_empty_when_reversing_done_item(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_empty_done_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdone").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is already empty, so there are no done items that can be reversed."));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is already empty, so there are no done items that can be reversed.",
        ));

        Ok(())
    }

    #[test]
    fn position_for_the_done_item_to_be_reversed_is_zero() -> Result<(), Box<dyn std::error::Error>>
    {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdone").arg("0");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd").arg("0");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn position_for_done_item_to_be_reversed_is_too_big() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdone").arg("10");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is smaller than your specified position; therefore, the item you want to reverse doesn't exist. The position has to be 5 or lower. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd").arg("10");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is smaller than your specified position; therefore, the item you want to reverse doesn't exist. The position has to be 5 or lower. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn done_item_reversed_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdone").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "'list' was reversed from done back to todo.\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: list\n-----\nDONE\n1: this\n2: is\n3: the\n4: done",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "'list' was reversed from done back to todo.\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: list\n-----\nDONE\n1: this\n2: is\n3: the\n4: done",
        ));

        Ok(())
    }
}

mod edit_todo_item_tests {
    use super::*;

    #[test]
    fn position_for_todo_item_to_be_edited_is_not_specified(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position that will be edited. A good example would be: 'chartodo edit 3 abc', and if a todo item existed at position 3, it would be changed to 'abc'. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position that will be edited. A good example would be: 'chartodo edit 3 abc', and if a todo item existed at position 3, it would be changed to 'abc'. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn position_is_not_a_number_or_not_u8_for_todo_item_to_be_edited(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("a").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position that will be edited, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo edit 3 abc', and if a todo item existed at position 3, it would be changed to 'abc'. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("a").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position that will be edited, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo edit 3 abc', and if a todo item existed at position 3, it would be changed to 'abc'. Please try again, or try 'chartodo help'.",
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
            "The todo list is currently empty, so there are no todo items that can be edited. Try adding items to the todo list. To see how, type 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("5").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is currently empty, so there are no todo items that can be edited. Try adding items to the todo list. To see how, type 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn position_for_the_todo_item_to_be_edited_is_zero() -> Result<(), Box<dyn std::error::Error>>
    {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("0").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("0").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn position_for_todo_item_to_be_edited_is_too_big() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("10").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is smaller than your specified position; therefore, the item you want to edit doesn't exist. The position has to be 5 or lower. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("10").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is smaller than your specified position; therefore, the item you want to edit doesn't exist. The position has to be 5 or lower. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn todo_item_to_edited_to_is_not_specified() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("1").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify what you want todo item #1 to be changed to. A good example would be 'chartodo edit 1 new_todo'. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("1").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify what you want todo item #1 to be changed to. A good example would be 'chartodo edit 1 new_todo'. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn todo_item_to_be_edited_to_is_too_long() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("1").arg("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "Editing a todo item to be longer than 150 characters is not allowed. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("1").arg("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "Editing a todo item to be longer than 150 characters is not allowed. Please try again, or try 'chartodo help'.",
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

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("5").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "Todo item 'list' was changed to 'abc'.\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: abc\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        Ok(())
    }
}

#[test]
fn help_is_shown_correctly() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("chartodo")?;
    cmd.arg("help");
    cmd.assert().success().stdout(predicate::str::contains(
        "
    CHARTODO is a simple command-line-interface (CLI) todo list application

    Commands:
    help, h
        show help
    list, l
        show the todo list
        example: chartodo list
    add, a
        add an item to the todo list. To add a multi-word item, replace space with something like -
        example: chartodo add item
        example: chartodo add new-item
    done, d
        change a todo item to done, using a numbered position to specify which one
        example: 'chartodo done 3' would change the third todo item to done
    rmtodo, rmt
        remove a todo item from the list, using a numbered position to specify which one
        example: 'chartodo rmt 4' would remove the fourth todo item
    cleartodo, ct
        clear the todo list
        example: chartodo cleartodo
    doneall, da
        change all todo items to done
        example: chartodo da
    cleardone, cd
        clear the done list
        example: chartodo cd
    clearall, ca
        clear both todo and done lists
        example: chartodo clearall
    rmdone, rmd
        removes a done item at the specified position
        example: chartodo rmd 4
    notdone, nd
        reverses a done item back to a todo item
        example: chartodo nd 3
    edit, e
        changes a todo item, with its position specified, to what you want
        example: chartodo edit 3 change-item-to-this
    ",
    ));

    let mut cmd = Command::cargo_bin("chartodo")?;
    cmd.arg("h");
    cmd.assert().success().stdout(predicate::str::contains(
        "
    CHARTODO is a simple command-line-interface (CLI) todo list application

    Commands:
    help, h
        show help
    list, l
        show the todo list
        example: chartodo list
    add, a
        add an item to the todo list. To add a multi-word item, replace space with something like -
        example: chartodo add item
        example: chartodo add new-item
    done, d
        change a todo item to done, using a numbered position to specify which one
        example: 'chartodo done 3' would change the third todo item to done
    rmtodo, rmt
        remove a todo item from the list, using a numbered position to specify which one
        example: 'chartodo rmt 4' would remove the fourth todo item
    cleartodo, ct
        clear the todo list
        example: chartodo cleartodo
    doneall, da
        change all todo items to done
        example: chartodo da
    cleardone, cd
        clear the done list
        example: chartodo cd
    clearall, ca
        clear both todo and done lists
        example: chartodo clearall
    rmdone, rmd
        removes a done item at the specified position
        example: chartodo rmd 4
    notdone, nd
        reverses a done item back to a todo item
        example: chartodo nd 3
    edit, e
        changes a todo item, with its position specified, to what you want
        example: chartodo edit 3 change-item-to-this
    ",
    ));

    Ok(())
}

#[test]
fn invalid_input() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("chartodo")?;
    cmd.arg("blahblah");
    cmd.assert().success().stdout(predicate::str::contains(
        "invalid command. please try again, or try 'chartodo help'.",
    ));

    Ok(())
}

#[test]
fn zzz_resets_the_file() {
    // note: this is just to reset the file after all the changes for my own convenience.
    // the fn also starts with zzz cuz rust runs the tests in alphabetical order, and I
    // want this to be the last one everytime
    let _ = create_test_file();
}
