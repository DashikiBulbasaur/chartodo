mod helpers;

use assert_cmd::prelude::*;
use helpers::*;
use predicates::prelude::*;
use std::process::Command;

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
    fn todo_item_to_add_is_empty() -> Result<(), Box<dyn std::error::Error>> {
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
        // note: gonna decrease this to 30 eventually
        // TODO: decrease this to 30
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

    #[test]
    fn todo_item_to_add_isnt_specified() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("add");
        cmd.assert().try_failure()?.stderr(predicate::str::contains("Error: Did not provide the todo item to be added. Good example: chartodo add new-item. If you have more questions, try chartodo help or chartodo --help"));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("a");
        // note: lowkey starting to realize i don't need predicate for a lot of these
        cmd.assert().try_failure()?.stderr(predicate::str::contains("Error: Did not provide the todo item to be added. Good example: chartodo a new-item. If you have more questions, try chartodo help or chartodo --help"));

        Ok(())
    }
}

mod todo_item_to_done_tests {
    use super::*;

    #[test]
    fn todo_item_to_done_position_isnt_specified() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done");
        cmd.assert().try_failure()?.stderr(predicate::str::contains(
            "Error: Did not provide the todo item to be changed to done. Good example: chartodo done 3. If you have more questions, try chartodo help or chartodo --help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d");
        cmd.assert().try_failure()?.stderr(predicate::str::contains(
            "Error: Did not provide the todo item to be changed to done. Good example: chartodo d 3. If you have more questions, try chartodo help or chartodo --help",
        ));

        Ok(())
    }

    #[test]
    fn todo_item_to_done_position_is_empty() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("d").arg("256");
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
    fn position_for_todo_item_to_be_removed_is_missing() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo");
        cmd.assert().try_failure()?.stderr(predicate::str::contains(
            "Error: Did not provide the todo item to be removed. Good example: chartodo rmtodo 3. If you have more questions, try chartodo help or chartodo --help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt");
        cmd.assert().try_failure()?.stderr(predicate::str::contains(
            "Error: Did not provide the todo item to be removed. Good example: chartodo rmt 3. If you have more questions, try chartodo help or chartodo --help",
        ));

        Ok(())
    }

    #[test]
    fn position_for_todo_item_to_be_removed_is_empty() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("rmt").arg("256");
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
    fn todo_item_to_be_edited_to_is_missing() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("3");
        cmd.assert().try_failure()?.stderr(predicate::str::contains(
            "Error: Did not specify what you want the todo item to be edited to. Good example: chartodo edit 3 abc. If you have more questions, try chartodo help or chartodo --help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("4");
        cmd.assert().try_failure()?.stderr(predicate::str::contains(
            "Error: Did not specify what you want the todo item to be edited to. Good example: chartodo e 4 abc. If you have more questions, try chartodo help or chartodo --help",
        ));

        Ok(())
    }

    #[test]
    fn position_for_todo_item_to_be_edited_is_empty() -> Result<(), Box<dyn std::error::Error>> {
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
    fn position_for_the_todo_item_to_be_edited_is_zero() -> Result<(), Box<dyn std::error::Error>> {
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
        // TODO: decreae this to 30
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
fn zzz_resets_the_file() {
    // note: this is just to reset the file after all the changes for my own convenience.
    // the fn also starts with zzz cuz rust runs the tests in alphabetical order, and I
    // want this to be the last one everytime
    let _ = create_test_file();
}
