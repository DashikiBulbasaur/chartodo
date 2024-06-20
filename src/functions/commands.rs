use super::helpers::*;
use std::{io::Write, path::PathBuf};

// NB: the general flow for each functionality are
// 1. read the file and create vecs for the two lists
// 2. if needed, modify a list/both lists, then write to the same file
// 3. add positions to the vec lists
// 4. print the lists

// TODO: reduce the length of some of the errors

// linux: $HOME/.local/share/chartodo/general_list.txt
// windows: C:\Users\some_user\AppData\Local\chartodo\general_list.txt
// mac: /Users/some_user/Library/Application Support/chartodo/general_list.txt
fn path_to_chartodo_file() -> PathBuf {
    let mut path = dirs::data_dir().unwrap();
    path.push("chartodo/general_list.txt");

    path
}

pub fn list() {
    let path = path_to_chartodo_file();

    // NB: read from file and separate it into vecs
    let (todo_buf, done_buf) = read_file_and_create_vecs(path);
    // NB: add positions to todo_buf and done_buf before printing
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);
    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}

pub fn add_todo_item(add_item: String) {
    let path = path_to_chartodo_file();

    // NB: read from file and separate into vecs
    let (mut todo_buf, done_buf) = read_file_and_create_vecs(path.clone());

    let writer = &mut std::io::stdout();

    // the following conditionals check for invalid inputs.
    if add_item.is_empty() {
        return writeln!(
            writer,
            "Items to be added to the todo list cannot be empty. Please try again, or try chartodo help"
        )
        .expect("writeln failed");
    }

    // note: this will never activate i think. either clap or cargo/rust panics when something like
    // -- is added.
    if add_item.trim() == "-----" {
        return writeln!(
            writer,
            "----- is an invalid item. It is the only invalid item. Please try again, or try chartodo help"
        )
        .expect("writeln failed");
    }

    if add_item.trim().len() > 50 {
        return writeln!(
            writer,
            "The maximum length of an item is 50 characters. Please try again, or try chartodo help"
        )
        .expect("writeln failed");
    }

    // -----

    // the following lines do the following in order:
    // 1. push new item to todo
    // 2. create a new file
    // 3. push todo_buf and done_buf to file
    todo_buf.push(add_item.clone());
    let (todo_buf, done_buf) = create_new_file_and_write(path, todo_buf, done_buf);

    // ----

    // NB: todo_buf has a max len of 15
    // NB: adding items that would exceed the max len just erases the last item and replaces it
    // with the new one. idk how it does this, but i'm fine with it

    // NB: add positions to todo_buf and done_buf before printing
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

    writeln!(writer, "'{}' was added to todo\n", add_item).expect("writeln failed");

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}

pub fn change_todo_item_to_done(position: String) {
    let path = path_to_chartodo_file();

    // NB: read file and create vecs
    let (mut todo_buf, mut done_buf) = read_file_and_create_vecs(path.clone());

    let writer = &mut std::io::stdout();

    if position.is_empty() {
        return writeln!(
            writer,
            "You must specify the todo item's position. Good example: chartodo done 3. Please try again, or try chartodo help"
        )
        .expect("writeln failed");
    }

    // note: I'm keeping this as u8 just so it's slightly faster
    if position.parse::<u8>().is_err() {
        return writeln!(
            writer,
            "You must specify the todo item's position, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. Good example: chartodo done 3. Please try again, or try chartodo help"
        )
        .expect("writeln failed");

        // NB: the user can't seem to do a negative number arg like -1, or else clap/cargo
        // panics and complains. I also can't seem to test for it.
    }

    if todo_buf.len() == 1 {
        writeln!(
            writer,
            "The todo list is currently empty, so there are no todo items that can be marked as done. Try adding items to the todo list. To see how, type chartodo help"
        )
        .expect("writeln failed");

        return print_the_lists(todo_buf, done_buf);
    }

    if position.parse::<u8>().unwrap() == 0 {
        return writeln!(
            writer,
            "The position specified cannot be 0. Try a position that is between 1 and {}. Please try again, or try chartodo help", todo_buf.len() - 1
        )
        .expect("writeln failed");
    }

    if position.parse::<u8>().unwrap() > (todo_buf.len() - 1).try_into().unwrap() {
        return writeln!(
            writer,
            "The todo list is smaller than your specified position; therefore, the item you want to mark as done doesn't exist. The position has to be {} or lower. Please try again, or try 'chartodo help'.", todo_buf.len() - 1
        )
        .expect("writeln failed");
    }

    // get the todo item, remove it from todo, and push it to done
    let position = position.parse::<usize>().unwrap();
    let todo_to_done = todo_buf.get(position).unwrap().to_string();
    todo_buf.remove(position);
    done_buf.push(todo_to_done.clone());

    // NB: after changes, write to file
    let (todo_buf, done_buf) = create_new_file_and_write(path, todo_buf, done_buf);

    // NB: add positions to todo and done b4 printing
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

    writeln!(writer, "'{}' was marked as done\n", todo_to_done).expect("writeln failed");

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}

pub fn remove_todo_item(position: String) {
    let path = path_to_chartodo_file();

    // NB: read file and create vecs
    let (mut todo_buf, done_buf) = read_file_and_create_vecs(path.clone());

    let writer = &mut std::io::stdout();

    if position.is_empty() {
        return writeln!(
            writer,
            "You must specify the todo item's position that will be removed. A good example would be: 'chartodo rmtodo 3'. Please try again, or try 'chartodo help'."
        )
        .expect("writeln failed");
    }

    if position.parse::<u8>().is_err() {
        return writeln!(
            writer,
            "You must specify the todo item's position that will be removed, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo rmtodo 3'. Please try again, or try 'chartodo help'."
        )
        .expect("writeln failed");

        // NB: the user can't seem to do a negative number arg like -1, or else clap/cargo
        // panics and complains. I also can't seem to test for it.
    }

    if todo_buf.len() == 1 {
        writeln!(
            writer,
            "The todo list is currently empty, so there are no todo items that can be removed. Try adding items to the todo list. To see how, type 'chartodo help'."
        )
        .expect("writeln failed");

        return print_the_lists(todo_buf, done_buf);
    }

    if position.parse::<u8>().unwrap() == 0 {
        return writeln!(
            writer,
            "The position specified cannot be 0. Try a position that is between 1 and {}. Please try again, or try 'chartodo help'.", todo_buf.len() - 1
        )
        .expect("writeln failed");
    }

    if position.parse::<u8>().unwrap() > (todo_buf.len() - 1).try_into().unwrap() {
        return writeln!(
            writer,
            "The todo list is smaller than your specified position; therefore, the item you want to remove doesn't exist. The position has to be {} or lower. Please try again, or try 'chartodo help'.", todo_buf.len() - 1
        )
        .expect("writeln failed");
    }

    // get the todo item, remove it from todo, and push it to done
    let position = position.parse::<usize>().unwrap();
    let remove_todo = todo_buf.get(position).unwrap().to_string();
    todo_buf.remove(position);

    // NB: after changes, write to file
    let (todo_buf, done_buf) = create_new_file_and_write(path, todo_buf, done_buf);

    // NB: add positions to the lists
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

    writeln!(writer, "'{}' was removed from todo\n", remove_todo).expect("writeln failed");

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}

pub fn clear_todo_list() {
    let path = path_to_chartodo_file();

    // NB: read file and create vecs
    let (mut todo_buf, done_buf) = read_file_and_create_vecs(path.clone());

    let writer = &mut std::io::stdout();

    if todo_buf.len() == 1 {
        return writeln!(writer, "The todo list is already empty.").expect("writeln failed");
    }

    todo_buf.clear();
    todo_buf.push("CHARTODO".to_string());

    // NB: after changes, write to file
    let (todo_buf, done_buf) = create_new_file_and_write(path, todo_buf, done_buf);

    // NB: add positions to the lists
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

    writeln!(writer, "The todo list was cleared.\n").expect("writeln failed");

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}

pub fn change_all_todos_to_done() {
    let path = path_to_chartodo_file();

    // NB: read file and create vecs
    let (mut todo_buf, mut done_buf) = read_file_and_create_vecs(path.clone());

    let writer = &mut std::io::stdout();

    if todo_buf.len() == 1 {
        return writeln!(
            writer,
            "The todo list is empty, and so has no items that can be changed to done."
        )
        .expect("writeln failed");
    }

    todo_buf
        .iter()
        .skip(1)
        .for_each(|item| done_buf.push(item.to_string()));
    todo_buf.clear();
    todo_buf.push("CHARTODO".to_string());

    // NB: after changes, write to file
    let (todo_buf, done_buf) = create_new_file_and_write(path, todo_buf, done_buf);

    // NB: add positions to the lists
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

    writeln!(writer, "All todos were changed to done.\n").expect("writeln failed");

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}

pub fn clear_done_list() {
    let path = path_to_chartodo_file();

    // NB: read file and create vecs
    let (todo_buf, mut done_buf) = read_file_and_create_vecs(path.clone());

    let writer = &mut std::io::stdout();

    if done_buf.len() == 1 {
        return writeln!(writer, "The done list is already empty.").expect("writeln failed");
    }

    done_buf.clear();
    done_buf.push("DONE".to_string());

    // NB: after changes, write to file
    let (todo_buf, done_buf) = create_new_file_and_write(path, todo_buf, done_buf);

    // NB: add positions to the lists
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

    writeln!(writer, "The done list was cleared.\n").expect("writeln failed");

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}

pub fn clear_both_lists() {
    let path = path_to_chartodo_file();

    // NB: read file and create vecs
    let (mut todo_buf, mut done_buf) = read_file_and_create_vecs(path.clone());

    let writer = &mut std::io::stdout();

    if todo_buf.len() == 1 && done_buf.len() == 1 {
        return writeln!(writer, "The todo and done lists are already empty.")
            .expect("writeln failed");
    }

    todo_buf.clear();
    todo_buf.push("CHARTODO".to_string());
    done_buf.clear();
    done_buf.push("DONE".to_string());

    // NB: after changes, write to file
    let (todo_buf, done_buf) = create_new_file_and_write(path, todo_buf, done_buf);

    // NB: add positions to the lists
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

    writeln!(writer, "The todo and done lists were cleared.\n").expect("writeln failed");

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}

pub fn remove_done_item(position: String) {
    let path = path_to_chartodo_file();

    // NB: read file and create vecs
    let (todo_buf, mut done_buf) = read_file_and_create_vecs(path.clone());

    let writer = &mut std::io::stdout();

    if position.is_empty() {
        return writeln!(
            writer,
            "You must specify the done item's position that will be removed. A good example would be: 'chartodo rmdone 3'. Please try again, or try 'chartodo help'."
        )
        .expect("writeln failed");
    }

    if position.parse::<u8>().is_err() {
        return writeln!(
            writer,
            "You must specify the done item's position that will be removed, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo rmdone 3'. Please try again, or try 'chartodo help'."
        )
        .expect("writeln failed");

        // NB: the user can't seem to do a negative number arg like -1, or else clap/cargo
        // panics and complains. I also can't seem to test for it.
    }

    if done_buf.len() == 1 {
        writeln!(
            writer,
            "The done list is already empty, so there are no done items that can be removed."
        )
        .expect("writeln failed");

        return print_the_lists(todo_buf, done_buf);
    }

    if position.parse::<u8>().unwrap() == 0 {
        return writeln!(
            writer,
            "The position specified cannot be 0. Try a position that is between 1 and {}. Please try again, or try 'chartodo help'.", done_buf.len() - 1
        )
        .expect("writeln failed");
    }

    if position.parse::<u8>().unwrap() > (done_buf.len() - 1).try_into().unwrap() {
        return writeln!(
            writer,
            "The done list is smaller than your specified position; therefore, the item you want to remove doesn't exist. The position has to be {} or lower. Please try again, or try 'chartodo help'.", done_buf.len() - 1
        )
        .expect("writeln failed");
    }

    // get the todo item, remove it from todo, and push it to done
    let position = position.parse::<usize>().unwrap();
    let remove_done = done_buf.get(position).unwrap().to_string();
    done_buf.remove(position);

    // NB: after changes, write to file
    let (todo_buf, done_buf) = create_new_file_and_write(path, todo_buf, done_buf);

    // NB: add positions to the lists
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

    writeln!(writer, "'{}' was removed from done\n", remove_done).expect("writeln failed");

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}

pub fn item_not_done(position: String) {
    let path = path_to_chartodo_file();

    // NB: read file and create vecs
    let (mut todo_buf, mut done_buf) = read_file_and_create_vecs(path.clone());

    let writer = &mut std::io::stdout();

    if position.is_empty() {
        return writeln!(
            writer,
            "You must specify the done item's position that will be reversed. A good example would be: 'chartodo notdone 3', and if there was a done item at position 3, it would be reversed back to a todo item. Please try again, or try 'chartodo help'."
        )
        .expect("writeln failed");
    }

    if position.parse::<u8>().is_err() {
        return writeln!(
            writer,
            "You must specify the done item's position that will be reversed, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo notdone 3', and if there was a done item at position 3, it would be reversed back to a todo item. Please try again, or try 'chartodo help'."
        )
        .expect("writeln failed");

        // NB: the user can't seem to do a negative number arg like -1, or else clap/cargo
        // panics and complains. I also can't seem to test for it.
    }

    if done_buf.len() == 1 {
        writeln!(
            writer,
            "The done list is already empty, so there are no done items that can be reversed."
        )
        .expect("writeln failed");

        return print_the_lists(todo_buf, done_buf);
    }

    if position.parse::<u8>().unwrap() == 0 {
        return writeln!(
            writer,
            "The position specified cannot be 0. Try a position that is between 1 and {}. Please try again, or try 'chartodo help'.", done_buf.len() - 1
        )
        .expect("writeln failed");
    }

    if position.parse::<u8>().unwrap() > (done_buf.len() - 1).try_into().unwrap() {
        return writeln!(
            writer,
            "The done list is smaller than your specified position; therefore, the item you want to reverse doesn't exist. The position has to be {} or lower. Please try again, or try 'chartodo help'.", done_buf.len() - 1
        )
        .expect("writeln failed");
    }

    // get the todo item, remove it from todo, and push it to done
    let position = position.parse::<usize>().unwrap();
    let reverse_done = done_buf.get(position).unwrap().to_string();
    done_buf.remove(position);
    todo_buf.push(reverse_done.clone());

    // NB: after changes, write to file
    let (todo_buf, done_buf) = create_new_file_and_write(path, todo_buf, done_buf);

    // NB: add positions to the lists
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

    writeln!(
        writer,
        "'{}' was reversed from done back to todo.\n",
        reverse_done
    )
    .expect("writeln failed");

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}

pub fn edit_todo_item(position: String, new_todo_item: String) {
    let path = path_to_chartodo_file();

    // NB: read file and create vecs
    let (mut todo_buf, done_buf) = read_file_and_create_vecs(path.clone());

    let writer = &mut std::io::stdout();

    if position.is_empty() {
        return writeln!(
            writer,
            "You must specify the todo item's position that will be edited. A good example would be: 'chartodo edit 3 abc', and if a todo item existed at position 3, it would be changed to 'abc'. Please try again, or try 'chartodo help'."
        )
        .expect("writeln failed");
    }

    if position.parse::<u8>().is_err() {
        return writeln!(
            writer,
            "You must specify the todo item's position that will be edited, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo edit 3 abc', and if a todo item existed at position 3, it would be changed to 'abc'. Please try again, or try 'chartodo help'."
        )
        .expect("writeln failed");

        // NB: the user can't seem to do a negative number arg like -1, or else clap/cargo
        // panics and complains. I also can't seem to test for it.
    }

    if todo_buf.len() == 1 {
        writeln!(
            writer,
            "The todo list is currently empty, so there are no todo items that can be edited. Try adding items to the todo list. To see how, type 'chartodo help'."
        )
        .expect("writeln failed");

        return print_the_lists(todo_buf, done_buf);
    }

    if position.parse::<u8>().unwrap() == 0 {
        return writeln!(
            writer,
            "The position specified cannot be 0. Try a position that is between 1 and {}. Please try again, or try 'chartodo help'.", todo_buf.len() - 1
        )
        .expect("writeln failed");
    }

    if position.parse::<u8>().unwrap() > (todo_buf.len() - 1).try_into().unwrap() {
        return writeln!(
            writer,
            "The todo list is smaller than your specified position; therefore, the item you want to edit doesn't exist. The position has to be {} or lower. Please try again, or try 'chartodo help'.", todo_buf.len() - 1
        )
        .expect("writeln failed");
    }

    if new_todo_item.is_empty() {
        return writeln!(writer, "You must specify what you want todo item #{position} to be changed to. A good example would be 'chartodo edit {position} new_todo'. Please try again, or try 'chartodo help'.").expect("writeln failed");
    }

    if new_todo_item.len() > 150 {
        return writeln!(writer, "Editing a todo item to be longer than 150 characters is not allowed. Please try again, or try 'chartodo help'.").expect("writeln failed");
    }

    // get the todo item, remove it from todo, and push it to done
    let position = position.parse::<usize>().unwrap();
    let edit_todo = todo_buf.get(position).unwrap().to_string();
    todo_buf[position].clone_from(&new_todo_item);

    // NB: after changes, write to file
    let (todo_buf, done_buf) = create_new_file_and_write(path, todo_buf, done_buf);

    // NB: add positions to the lists
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

    writeln!(
        writer,
        "Todo item '{}' was changed to '{}'.\n",
        edit_todo, new_todo_item
    )
    .expect("writeln failed");

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}
