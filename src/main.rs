mod functions;

use clap::Parser;
use functions::commands::*;
use std::io::Write;

#[derive(Parser)]
struct Cli {
    /// The action taken
    command: String,
    /// If applicable, the name/position of the TODO/DONE item
    item_identifier: Option<String>,
    /// If changing a TODO item, this is where you specify what to change it to. If adding a todo
    /// item to a specific position, this is where you specify the position.
    edit_or_position: Option<String>,
}

fn main() {
    let args = Cli::parse();

    // TODO: better error handling (anyhow crate) for commands that expect some extra arg
    match args.command.as_str() {
        "help" | "h" => help(),
        "list" | "l" => list(),
        "add" | "a" => add_todo_item(
            args.item_identifier
                .expect("***Please specify the item you want to add to the todo list. Either you specified an empty string item, or you typed --. Both are not allowed. A correct example would be: 'chartodo add item'. For more information, try --help***")),
        "done" | "d" => change_todo_item_to_done(
            args.item_identifier
                .expect("***Please specify the item's position that you want to change as 'done'. Either you specified an empty string item, or you typed --. Both are not allowed. A correct example would be: 'chartodo done 3', and if a todo item existed at the third position, it would be changed to done. For more information, try --help***")),
        "rmtodo" | "rmt" => remove_todo_item(args.item_identifier.expect("***Please specify the position for the item that you want to remove. Either you specified an empty string item, or you typed --. Both are not allowed. A correct example would be: 'chartodo rmtodo 3', and if a todo item existed at the third position, it would be removed. For more information, try --help***")),
        "cleartodo" | "ct" => clear_todo_list(),
        "doneall" | "da" => change_all_todos_to_done(),
        "cleardone" | "cd" => clear_done_list(),
        "clearall" | "ca" => clear_both_lists(),
        "rmdone" | "rmd" => remove_done_item(args.item_identifier.expect("***Please specify the position for the item that you want to remove. Either you specified an empty string item, or you typed --. Both are not allowed. A correct example would be: 'chartodo rmdone 3', and if a done item existed at the third position, it would be removed. For more information, try --help***")),
        "notdone" | "nd" => item_not_done(args.item_identifier.expect("***Please specify the position for the done item that you want to reverse. Either you specified an empty string item, or you typed --. Both are not allowed. A correct example would be: 'chartodo notdone 3', and if a done item existed at the third position, it would be reversed. For more information, try --help***")),
        _ => command_error(),
    }
}

fn command_error() {
    let writer = &mut std::io::stdout();
    writeln!(
        writer,
        "invalid command. please try again, or try 'chartodo help'."
    )
    .expect("writeln failed");
}

fn help() {
    let writer = &mut std::io::stdout();
    writeln!(
        writer,
        "
    CHARTODO is a simple command-line-interface (CLI) todo list application

    Commands:
    help, h
        show help
    list, l
        show the todo list
        example: chartodo list
    add, a
        add an item to the todo list. To add a multi-word item, replace space with something like -
        example: chartodo add item
        example: chartodo add new-item
    done, d
        change a todo item to done, using a numbered position to specify which one
        example: 'chartodo done 3' would change the third todo item to done
    rmtodo, rmt
        remove a todo item from the list, using a numbered position to specify which one
        example: 'chartodo rmt 4' would remove the fourth todo item
    cleartodo, ct
        clear the todo list
        example: chartodo cleartodo
    doneall, da
        change all todo items to done
        example: chartodo da
    cleardone, cd
        clear the done list
        example: chartodo cd
    clearall, ca
        clear both todo and done lists
        example: chartodo clearall
    rmdone, rmd
        removes a done item at the specified position
        example: chartodo rmd 4
    notdone, nd
        reverses a done item back to a todo item
        example: chartodo nd 3
    "
    )
    .expect("writeln failed");
}
