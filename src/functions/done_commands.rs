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
