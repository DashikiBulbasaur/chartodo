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
