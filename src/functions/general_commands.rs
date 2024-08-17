use super::general_helpers::*;
use crate::functions::{
    deadline_tasks::deadline_helpers::*, regular_tasks::regular_helpers::*,
    repeating_tasks::repeating_helpers::*,
};
use comfy_table::*;
use modifiers::UTF8_ROUND_CORNERS;
use presets::UTF8_FULL;
use std::io::Write;

pub fn list() {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    deadline_tasks_create_dir_and_file_if_needed();
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();
    let mut table = Table::new();

    // open file and parse
    let regular_tasks = open_regular_tasks_and_return_tasks_struct();
    let deadline_tasks = open_deadline_tasks_and_return_tasks_struct();
    let repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // get strings to print
    let (regular_todo, regular_done) = regular_tasks_list(regular_tasks);
    let (deadline_todo, deadline_done) = deadline_tasks_list(deadline_tasks);
    let (repeating_todo, repeating_done) = repeating_tasks_list(repeating_tasks);

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("CHARTODO").add_attribute(Attribute::Bold),
            Cell::new("DEADLINES").add_attribute(Attribute::Bold),
            Cell::new("REPEATING").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            format!("{}", regular_todo),
            format!("{}", deadline_todo),
            format!("{}", repeating_todo),
        ])
        .add_row(vec![
            format!("{}", regular_done),
            format!("{}", deadline_done),
            format!("{}", repeating_done),
        ]);

    writeln!(writer, "{table}").expect("writeln failed");
}

pub fn clear_all_lists() -> bool {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    deadline_tasks_create_dir_and_file_if_needed();
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if all lists are empty
    if regular_tasks.todo.is_empty()
        && regular_tasks.done.is_empty()
        && deadline_tasks.todo.is_empty()
        && deadline_tasks.done.is_empty()
        && repeating_tasks.todo.is_empty()
        && repeating_tasks.done.is_empty()
    {
        writeln!(writer, "ERROR: All of the lists are currently empty.").expect("writeln failed");

        // error = true
        return true;
    }

    // clear all lists
    regular_tasks.todo.clear();
    regular_tasks.done.clear();
    deadline_tasks.todo.clear();
    deadline_tasks.done.clear();
    repeating_tasks.todo.clear();
    repeating_tasks.done.clear();

    // error = false
    false
}

pub fn clear_regular_tasks() -> bool {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if all lists are empty
    if regular_tasks.todo.is_empty() && regular_tasks.done.is_empty() {
        writeln!(writer, "ERROR: The regular task lists are currently empty.")
            .expect("writeln failed");

        // error = true
        return true;
    }

    // clear all lists
    regular_tasks.todo.clear();
    regular_tasks.done.clear();

    // error = false
    false
}

pub fn clear_deadline_tasks() -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if all lists are empty
    if deadline_tasks.todo.is_empty() && deadline_tasks.done.is_empty() {
        writeln!(
            writer,
            "ERROR: The deadline task lists are currently empty."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // clear all lists
    deadline_tasks.todo.clear();
    deadline_tasks.done.clear();

    // error = false
    false
}

pub fn clear_repeating_tasks() -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if all lists are empty
    if repeating_tasks.todo.is_empty() && repeating_tasks.done.is_empty() {
        writeln!(
            writer,
            "ERROR: The repeating task lists are currently empty."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // clear all lists
    repeating_tasks.todo.clear();
    repeating_tasks.done.clear();

    // error = false
    false
}
