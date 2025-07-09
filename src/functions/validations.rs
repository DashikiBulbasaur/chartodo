use crate::functions::json_file_structs::*;
use std::io::Write;

pub enum TodoOrDone {
    Todo,
    Done,
}

pub enum TaskType {
    Regular,
    Deadline,
    Repeating,
}

fn return_list_and_task_type(todo_or_done: TodoOrDone, task_type: TaskType) -> (String, String) {
    let list: String = match todo_or_done {
        TodoOrDone::Todo => String::from("todo"),
        TodoOrDone::Done => String::from("done"),
    };

    let task: String = match task_type {
        TaskType::Regular => String::from("regular"),
        TaskType::Deadline => String::from("deadline"),
        TaskType::Repeating => String::from("repeating"),
    };

    (list, task)
}

pub fn validate_empty_task_list(
    task_list: &[Task],
    todo_or_done: TodoOrDone,
    task_type: TaskType,
) -> bool {
    match &todo_or_done {
        TodoOrDone::Todo => match task_list.is_empty() {
            true => {
                print_empty_task_list_error(todo_or_done, task_type);
                true
            }
            false => false,
        },
        TodoOrDone::Done => match task_list.is_empty() {
            true => {
                print_empty_task_list_error(todo_or_done, task_type);
                true
            }
            false => false,
        },
    }
}

fn print_empty_task_list_error(todo_or_done: TodoOrDone, task_type: TaskType) {
    let writer = &mut std::io::stdout();
    let (list, type_of_task) = return_list_and_task_type(todo_or_done, task_type);

    writeln!(
        writer,
        "ERROR: The {type_of_task} {list} list is currently empty."
    )
    .expect("writeln failed");
}

pub fn validate_valid_args(args: &[String], todo_or_done: TodoOrDone, task_type: TaskType) -> bool {
    match &todo_or_done {
        TodoOrDone::Todo => match args.is_empty() {
            true => {
                print_valid_args_error(todo_or_done, task_type);
                true
            }
            false => false,
        },
        TodoOrDone::Done => match args.is_empty() {
            true => {
                print_valid_args_error(todo_or_done, task_type);
                true
            }
            false => false,
        },
    }
}

fn print_valid_args_error(todo_or_done: TodoOrDone, task_type: TaskType) {
    let writer = &mut std::io::stdout();
    let (list, type_of_task) = return_list_and_task_type(todo_or_done, task_type);

    writeln!(
        writer,
        "ERROR: None of the positions you provided were viable \
        -- they were all either negative, zero, exceeded the {type_of_task} {list} \
        list's length, or were invalid range positioning."
    )
    .expect("writeln failed");
}
