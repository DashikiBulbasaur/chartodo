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

pub fn add_todo_item(add_todos: Vec<String>) {
    let path = path_to_chartodo_file();

    // NB: read from file and separate into vecs
    let (mut todo_buf, done_buf) = read_file_and_create_vecs(path.clone());

    let writer = &mut std::io::stdout();

    // the following conditionals check for invalid inputs.
    if add_todos.is_empty() {
        return writeln!(
            writer,
            "You must add one or more items to the todo list. Good examples: chartodo add item, or chartodo add item one-more-item. Please try again, or try chartodo help"
        )
        .expect("writeln failed");
    }

    if (add_todos.len() + (todo_buf.len() - 1)) > 14 {
        return writeln!(writer, "The todo list is too full. Please try removing items or clearing it altogether. For more information, try chartodo help").expect("writeln failed");
    }

    // -----

    // the following lines do the following in order:
    // 1. push new item(s) to todo
    // 2. create a new file
    // 3. push todo_buf and done_buf to file
    add_todos.iter().for_each(|item| {
        if item != "-----" && item.len() < 31 {
            todo_buf.push(item.to_string());
        }
    });
    let (todo_buf, done_buf) = create_new_file_and_write(path, todo_buf, done_buf);

    // ----

    // NB: todo_buf has a max len of 15
    // NB: adding items that would exceed the max len just erases the last item and replaces it
    // with the new one. idk how it does this, but i'm fine with it

    // NB: add positions to todo_buf and done_buf before printing
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}

pub fn change_todo_item_to_done(todos_to_done: Vec<String>) {
    let path = path_to_chartodo_file();

    // NB: read file and create vecs
    let (mut todo_buf, mut done_buf) = read_file_and_create_vecs(path.clone());

    let writer = &mut std::io::stdout();

    if todos_to_done.is_empty() {
        return writeln!(
            writer,
            "You must specify the todo item's position(s). Good example: chartodo done 3, or chartodo done 3 4 5. Please try again, or try chartodo help"
        )
        .expect("writeln failed");
    }

    if todo_buf.len() == 1 {
        return writeln!(
            writer,
            "The todo list is currently empty, so there are no todo items that can be marked as done. Try adding items to the todo list. To see how, type chartodo help"
        )
        .expect("writeln failed");
    }

    if todos_to_done.len() > 14 {
        return writeln!(writer, "The todo list's maximum length is 15. You provided 15 or more todo items. At this point, you might as well just do chartodo doneall. For more information, try chartodo help").expect("writeln failed");
    }

    // in a better world, I'd love for this to be u8 so I can guarantee the small allocation in
    // memory
    let mut positions_sorted: Vec<usize> = vec![];
    // filter each argument for correctness, push it to a list. reverse sort and filter that list
    // for duplicates
    todos_to_done.iter().for_each(|item| {
        if item.parse::<usize>().is_ok()
            && !item.is_empty()
            && item.parse::<u8>().unwrap() != 0
            && item.parse::<usize>().unwrap() < todo_buf.len()
        {
            positions_sorted.push(item.parse().unwrap());
        }
    });

    // lowkey don't like how i make another vec. would like for it to just be 1 vec, but right now
    // this works. TODO: fix later
    positions_sorted.reverse();
    positions_sorted.dedup();

    // for each position in the list, remove from todo and push to done
    positions_sorted.iter().for_each(|position| {
        done_buf.push(todo_buf.get(*position).unwrap().to_string());
        todo_buf.remove(*position);
    });

    // NB: after changes, write to file
    let (todo_buf, done_buf) = create_new_file_and_write(path, todo_buf, done_buf);

    // NB: add positions to todo and done b4 printing
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}

pub fn remove_todo_item(todos_to_remove: Vec<String>) {
    let path = path_to_chartodo_file();

    // NB: read file and create vecs
    let (mut todo_buf, done_buf) = read_file_and_create_vecs(path.clone());

    let writer = &mut std::io::stdout();

    if todos_to_remove.is_empty() {
        return writeln!(
            writer,
            "You must specify the todo item's position(s) that will be removed. A good example would be: chartodo rmtodo 3, or chartodo rmt 3 4 5. Please try again, or try 'chartodo help'."
        )
        .expect("writeln failed");
    }

    if todo_buf.len() == 1 {
        writeln!(
            writer,
            "The todo list is currently empty, so there are no todo items that can be removed. Try adding items to the todo list. To see how, type 'chartodo help'."
        )
        .expect("writeln failed");

        return print_the_lists(todo_buf, done_buf);
    }

    let mut positions_sorted: Vec<usize> = vec![];
    todos_to_remove.iter().for_each(|item| {
        if item.parse::<usize>().is_ok()
            && !item.is_empty()
            && item.parse::<u8>().unwrap() != 0
            && item.parse::<usize>().unwrap() < todo_buf.len()
        {
            positions_sorted.push(item.parse().unwrap());
        }
    });
    positions_sorted.reverse();
    positions_sorted.dedup();

    positions_sorted.iter().for_each(|position| {
        todo_buf.remove(*position);
    });

    // NB: after changes, write to file
    let (todo_buf, done_buf) = create_new_file_and_write(path, todo_buf, done_buf);

    // NB: add positions to the lists
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

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

    if (todo_buf.len() - 1) + (done_buf.len() - 1) > 31 {
        done_buf.clear();
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

pub fn edit_todo_item(position: Vec<String>, new_todo_item: String) {
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

    if position.len() > 1 {
        return writeln!(
            writer,
            "You can only specify one todo item to edit. Please try again, or try chartodo help"
        )
        .expect("writeln failed");
    }

    if todo_buf.len() == 1 {
        return writeln!(
            writer,
            "The todo list is currently empty, so there are no todo items that can be edited. Try adding items to the todo list. To see how, type 'chartodo help'."
        )
        .expect("writeln failed");
    }

    if new_todo_item.is_empty() {
        return writeln!(writer, "You must specify what you want todo item to be changed to. A good example would be: chartodo edit 1 new_todo. Please try again, or try chartodo help.").expect("writeln failed");
    }

    if new_todo_item.len() > 30 {
        return writeln!(writer, "Editing a todo item to be longer than 30 characters is not allowed. Please try again, or try chartodo help").expect("writeln failed");
    }

    // get the todo item, remove it from todo, and push it to done
    let position: usize = position.first().unwrap().parse().unwrap();
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
