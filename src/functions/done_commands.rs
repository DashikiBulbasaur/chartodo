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

pub fn remove_done_item(dones_to_remove: Vec<String>) {
    let path = path_to_chartodo_file();

    // NB: read file and create vecs
    let (todo_buf, mut done_buf) = read_file_and_create_vecs(path.clone());

    let writer = &mut std::io::stdout();

    if done_buf.len() == 1 {
        writeln!(
            writer,
            "The done list is already empty, so there are no done items that can be removed."
        )
        .expect("writeln failed");
    }

    let mut positions_sorted: Vec<usize> = vec![];
    dones_to_remove.iter().for_each(|item| {
        if item.parse::<usize>().is_ok()
            && !item.is_empty()
            && item.parse::<u8>().unwrap() != 0
            && item.parse::<usize>().unwrap() < todo_buf.len()
        {
            positions_sorted.push(item.parse().unwrap());
        }
    });
    drop(dones_to_remove);

    positions_sorted.sort();
    positions_sorted.reverse();
    positions_sorted.dedup();

    if positions_sorted.len() >= done_buf.len() - 1 {
        return writeln!(writer, "The number of arguments you provided meet or exceed the done list's current filled length. You might as well do chartodo cleardone. For more information, try chartodo help").expect("writeln failed");
    }

    positions_sorted.iter().for_each(|position| {
        done_buf.remove(*position);
    });

    // NB: after changes, write to file
    let (todo_buf, done_buf) = create_new_file_and_write(path, todo_buf, done_buf);

    // NB: add positions to the lists
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}

pub fn item_not_done(dones_to_todos: Vec<String>) {
    let path = path_to_chartodo_file();

    // NB: read file and create vecs
    let (mut todo_buf, mut done_buf) = read_file_and_create_vecs(path.clone());

    let writer = &mut std::io::stdout();

    if done_buf.len() == 1 {
        return writeln!(
            writer,
            "The done list is already empty, so there are no done items that can be reversed."
        )
        .expect("writeln failed");
    }

    let mut positions_sorted: Vec<usize> = vec![];
    dones_to_todos.iter().for_each(|item| {
        if item.parse::<usize>().is_ok()
            && !item.is_empty()
            && item.parse::<u8>().unwrap() != 0
            && item.parse::<usize>().unwrap() < todo_buf.len()
        {
            positions_sorted.push(item.parse().unwrap());
        }
    });
    drop(dones_to_todos);

    positions_sorted.sort();
    positions_sorted.reverse();
    positions_sorted.dedup();

    if positions_sorted.len() >= done_buf.len() - 1 {
        return writeln!(writer, "The number of arguments you provided meet or exceed the done list's current filled length. You might as well just do chartodo notdoneall. For more information, try chartodo help").expect("writeln failed");
    }

    if (positions_sorted.len()) + (todo_buf.len() - 1) > 15 {
        return writeln!(writer, "The todo list is currently full. Try removing items or clearing it. For more information, try chartodo help").expect("writeln failed");
    }

    positions_sorted.iter().for_each(|position| {
        todo_buf.push(done_buf.get(*position).unwrap().to_string());
        done_buf.remove(*position);
    });

    // NB: after changes, write to file
    let (todo_buf, done_buf) = create_new_file_and_write(path, todo_buf, done_buf);

    // NB: add positions to the lists
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

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

pub fn reverse_all_done_items_to_todo() {
    let path = path_to_chartodo_file();

    // NB: read file and create vecs
    let (mut todo_buf, mut done_buf) = read_file_and_create_vecs(path.clone());

    let writer = &mut std::io::stdout();

    if done_buf.len() == 1 {
        return writeln!(
            writer,
            "The todo list is empty, and so has no items that can be changed to done."
        )
        .expect("writeln failed");
    }

    if (todo_buf.len() - 1) + (done_buf.len() - 1) > 15 {
        return writeln!(writer, "You're trying to reverse too many dones back to todos, and doing so would exceed the todo list's maximum length. Please remove some or clear all of the todo items. To see how, try chartodo help").expect("writeln failed");
    }

    done_buf
        .iter()
        .skip(1)
        .for_each(|item| todo_buf.push(item.to_string()));
    done_buf.clear();
    done_buf.push("DONE".to_string());

    // NB: after changes, write to file
    let (todo_buf, done_buf) = create_new_file_and_write(path, todo_buf, done_buf);

    // NB: add positions to the lists
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

    writeln!(writer, "All dones were reversed back to todos\n").expect("writeln failed");

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}
