use super::helpers::*;
use std::{io::Write, path::PathBuf};

// NB: the general flow for each functionality are
// 1. read the file and create vecs for the two lists
// 2. if needed, modify a list/both lists, then write to the same file
// 3. add positions to the vec lists
// 4. print the lists

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
            "Items to be added to the todo list cannot be empty. Please try again, or try 'chartodo help'."
        )
        .expect("writeln failed");
    }

    // note: this will never activate i think. either clap or cargo/rust panics when something like
    // -- is added.
    if add_item.trim() == "-----" {
        return writeln!(
            writer,
            "----- is an invalid item. It is the only invalid item. Please try again, or try 'chartodo help'."
        )
        .expect("writeln failed");
    }

    if add_item.trim().len() > 150 {
        return writeln!(
            writer,
            "The maximum length of an item is 150 characters. Please try again, or try 'chartodo help'."
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
            "You must specify the todo item's position. A good example would be: 'chartodo done 3'. Please try again, or try 'chartodo help'."
        )
        .expect("writeln failed");
    }

    // note: I'm keeping this as u8 just so it's slightly faster
    if position.parse::<u8>().is_err() {
        return writeln!(
            writer,
            "You must specify the todo item's position, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo done 3'. Please try again, or try 'chartodo help'."
        )
        .expect("writeln failed");

        // NB: the user can't seem to do a negative number arg like -1, or else clap/cargo
        // panics and complains. I also can't seem to test for it.
    }

    if todo_buf.len() == 1 {
        writeln!(
            writer,
            "The todo list is currently empty, so there are no todo items that can be marked as done. Try adding items to the todo list. To see how, type 'chartodo help'."
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
