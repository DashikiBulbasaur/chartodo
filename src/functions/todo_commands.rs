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

    // note: used to test with is_empty and stdout back when i had no argument chaining. now
    // with option<vector<string>>, checking whether it's empty defaults to stderr

    if (add_todos.len() + (todo_buf.len() - 1)) > 15 {
        return writeln!(writer, "The todo list is too full. Please try removing items or clearing it altogether. For more information, try chartodo help").expect("writeln failed");
    }

    // -----

    // the following lines do the following in order:
    // 1. push new item(s) to todo
    // 2. create a new file
    // 3. push todo_buf and done_buf to file
    add_todos.iter().for_each(|item| {
        // note that ----- may never register
        if item != "-----" && item.len() < 31 && !item.is_empty() {
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

    if todo_buf.len() == 1 {
        return writeln!(
            writer,
            "The todo list is currently empty, so there are no todo items that can be marked as done. Try adding items to the todo list. To see how, type chartodo help"
        )
        .expect("writeln failed");
    }

    // in a better world, I'd love for this to be u8 so I can guarantee the small allocation in
    // memory
    let mut positions_sorted: Vec<usize> = vec![];
    // filter each argument for correctness, push it to a list. reverse sort and filter that list
    // for duplicates
    todos_to_done.iter().for_each(|item| {
        if item.parse::<usize>().is_ok()
            && !item.is_empty()
            && item.parse::<usize>().unwrap() != 0
            && item.parse::<usize>().unwrap() < todo_buf.len()
        {
            positions_sorted.push(item.parse().unwrap());
        }
    });
    drop(todos_to_done);

    // lowkey don't like how i make another vec. would like for it to just be 1 vec, but right now
    // this works. TODO: maybe fix later
    positions_sorted.sort();
    positions_sorted.reverse();
    positions_sorted.dedup();

    if positions_sorted.len() >= todo_buf.len() - 1 {
        return writeln!(writer, "The number of your arguments meet or exceed the todo list's current filled length. At this point, you might as well just do chartodo doneall. For more information, try chartodo help").expect("writeln failed");
    }

    if positions_sorted.len() + (done_buf.len() - 1) > 15 {
        return writeln!(writer, "You're trying to change too many todos to done, as doing so would exceed the done list's max length. Try marking fewer todos as done, or remove some done items/clear the done list. For more information, try chartodo help").expect("writeln failed");
    }

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

    if todo_buf.len() == 1 {
        writeln!(
            writer,
            "The todo list is currently empty, so there are no todo items that can be removed. Try adding items to the todo list. To see how, type chartodo help"
        )
        .expect("writeln failed");
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
    drop(todos_to_remove);

    positions_sorted.sort();
    positions_sorted.reverse();
    positions_sorted.dedup();

    if positions_sorted.len() >= todo_buf.len() - 1 {
        return writeln!(writer, "The number of your arguments meet or exceed the todo list's current filled length. At this point, you might as well just do chartodo cleartodo. For more information, try chartodo help").expect("writeln failed");
    }

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

    if (todo_buf.len() - 1) + (done_buf.len() - 1) > 15 {
        done_buf.clear();
        done_buf.push("DONE".to_string());
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

pub fn edit_todo_item(position_and_item: Vec<String>) {
    let path = path_to_chartodo_file();

    // NB: read file and create vecs
    let (mut todo_buf, done_buf) = read_file_and_create_vecs(path.clone());

    let writer = &mut std::io::stdout();

    if position_and_item.len() != 2 {
        return writeln!(
            writer,
            "You must specify both the item's position and what to edit it to, and no more/less. Good example: chartodo edit 3 abc. Please try again, or try chartodo help"
        )
        .expect("writeln failed");
    }

    if todo_buf.len() == 1 {
        return writeln!(
            writer,
            "The todo list is currently empty, so there are no todo items that can be edited. Try adding items to the todo list. To see how, type chartodo help"
        )
        .expect("writeln failed");
    }

    if position_and_item.first().unwrap().is_empty() {
        // again, idk how this would ever activate
        return writeln!(writer, "You must provide the todo item's position that will be edited. Please try again, or try chartodo help").expect("writeln failed");
    }

    if position_and_item.get(1).unwrap().is_empty() {
        // again, idk how this would ever activate
        return writeln!(writer, "You must specify what the todo item will be edited to. Please try again, or try chartodo help").expect("writeln failed");
    }

    if position_and_item.first().unwrap().parse::<u8>().is_err() {
        return writeln!(writer, "You must specify the item's position that will be edited. Please specify a position between 1 and {}, or try chartodo help", todo_buf.len() - 1).expect("writeln failed");
    }

    if position_and_item.first().unwrap().parse::<u8>().unwrap() == 0 {
        return writeln!(writer, "The item's position can't be zero. Please specify a position between 1 and {}, or try chartodo help", todo_buf.len() - 1).expect("writeln failed");
    }

    if position_and_item.first().unwrap().parse::<usize>().unwrap() > todo_buf.len() - 1 {
        return writeln!(writer, "The position you specified is bigger than the todo list. Please specify a position between 1 and {}, or try chartodo help", todo_buf.len() - 1).expect("writeln failed");
    }

    if position_and_item.get(1).unwrap().len() > 30 {
        return writeln!(writer, "Editing a todo item to be longer than 30 characters is not allowed. Please try again, or try chartodo help").expect("writeln failed");
    }

    // get the todo item, remove it from todo, and push it to done
    let position: usize = position_and_item.first().unwrap().parse().unwrap();
    let edit_todo = todo_buf.get(position).unwrap().to_string();
    todo_buf[position].clone_from(position_and_item.get(1).unwrap());

    // NB: after changes, write to file
    let (todo_buf, done_buf) = create_new_file_and_write(path, todo_buf, done_buf);

    // NB: add positions to the lists
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

    writeln!(
        writer,
        "Todo item '{}' was changed to '{}'.\n",
        edit_todo,
        position_and_item.get(1).unwrap()
    )
    .expect("writeln failed");

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}
