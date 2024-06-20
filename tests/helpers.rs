use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{fs::File, io::Write, process::Command};

// cargo test --test helpers -- --test-threads=1

pub fn create_test_file() -> Result<(), Box<dyn std::error::Error>> {
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

#[allow(dead_code)]
pub fn create_empty_todo_test_file() -> Result<(), Box<dyn std::error::Error>> {
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

#[allow(dead_code)]
pub fn create_empty_done_test_file() -> Result<(), Box<dyn std::error::Error>> {
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

pub fn create_both_lists_empty_test_file() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut cmd = Command::cargo_bin("chartodo")?;
    cmd.arg("l");
    cmd.assert().success().stdout(predicate::str::contains(
        "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
    ));

    Ok(())
}

mod clear_both_list_tests {
    use super::*;

    #[test]
    fn both_list_are_already_empty_cleartodo() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_both_lists_empty_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("clearall");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo and done lists are already empty.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ca");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo and done lists are already empty.",
        ));

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
        "Invalid command. please try again, or try chartodo help",
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
