use std::io::Write;

pub enum TodoOrDone {
    Todo,
    Done,
}

#[derive(PartialEq)]
pub enum TaskType {
    Regular,
    Deadline,
    Repeating,
}

pub enum PositionCommands {
    Done,
    NotDone,
    RmTodo,
    RmDone,
    Reset,
    Start,
}

fn return_list_and_task_type(todo_or_done: TodoOrDone, task_type: &TaskType) -> (String, String) {
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

fn return_all_command_equivalent(positional: PositionCommands) -> (String, String) {
    let position_command = match positional {
        PositionCommands::Done => String::from("done"),
        PositionCommands::NotDone => String::from("notdone"),
        PositionCommands::RmTodo => String::from("rmtodo"),
        PositionCommands::RmDone => String::from("rmdone"),
        PositionCommands::Reset => String::from("reset"),
        PositionCommands::Start => String::from("start"),
    };

    let all_command: String = match positional {
        PositionCommands::Done => String::from("doneall"),
        PositionCommands::NotDone => String::from("notdoneall"),
        PositionCommands::RmTodo => String::from("cleartodo"),
        PositionCommands::RmDone => String::from("cleardone"),
        PositionCommands::Reset => String::from("resetall"),
        PositionCommands::Start => String::from("startall"),
    };

    (position_command, all_command)
}

pub fn validate_empty_task_list(
    empty_list: bool,
    todo_or_done: TodoOrDone,
    task_type: TaskType,
) -> bool {
    match &todo_or_done {
        TodoOrDone::Todo => match empty_list {
            true => {
                print_empty_task_list_error(todo_or_done, task_type);
                true
            }
            false => false,
        },
        TodoOrDone::Done => match empty_list {
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
    let (list, type_of_task) = return_list_and_task_type(todo_or_done, &task_type);

    writeln!(
        writer,
        "ERROR: The {type_of_task} {list} list is currently empty."
    )
    .expect("writeln failed");
}

pub fn validate_valid_args(
    no_valid_args: bool,
    todo_or_done: TodoOrDone,
    task_type: TaskType,
) -> bool {
    match &todo_or_done {
        TodoOrDone::Todo => match no_valid_args {
            true => {
                print_valid_args_error(todo_or_done, task_type);
                true
            }
            false => false,
        },
        TodoOrDone::Done => match no_valid_args {
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
    let (list, type_of_task) = return_list_and_task_type(todo_or_done, &task_type);

    writeln!(
        writer,
        "ERROR: None of the positions you provided were viable \
        -- they were all either negative, zero, exceeded the {type_of_task} {list} \
        list's length, or were invalid range positioning."
    )
    .expect("writeln failed");
}

pub fn validate_should_do_all_equivalent(
    arg_len: usize,
    list_len: usize,
    todo_or_done: TodoOrDone,
    task_type: TaskType,
    positional: PositionCommands,
) -> bool {
    if arg_len >= list_len && list_len > 5 {
        print_should_do_all_equivalent_warning(todo_or_done, task_type, positional);
        true
    } else {
        false
    }
}

fn print_should_do_all_equivalent_warning(
    todo_or_done: TodoOrDone,
    task_type: TaskType,
    positional: PositionCommands,
) {
    let writer = &mut std::io::stdout();
    let (list, type_of_task) = return_list_and_task_type(todo_or_done, &task_type);
    let (position_command, all_command) = return_all_command_equivalent(positional);

    match task_type {
        TaskType::Regular => {
            writeln!(
                writer,
                "WARNING: you've specified marking the entire {type_of_task} {list} list as \
                {position_command}. You should do chartodo {all_command}."
            )
            .expect("writeln failed");
        }
        _ => {
            writeln!(
                writer,
                "WARNING: you've specified marking the entire {type_of_task} {list} list as \
                {position_command}. You should do chartodo {type_of_task}-{all_command}."
            )
            .expect("writeln failed");
        }
    }
}
