mod functions;

use clap::Parser;
use functions::functionalities::{add_todo_item, change_todo_item_to_done, list};
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

    if &args.command == "list" || &args.command == "l" {
        list();
    } else if &args.command == "add" || &args.command == "a" {
        add_todo_item(
            args.item_identifier
                .expect("***Please specify the item you want to add to the todo list. Either you specified an empty string item, or you typed --. Both of which are not allowed. A correct example would be: 'chartodo add item'. For more information, try --help***"),
        );
    } else if &args.command == "done" || &args.command == "d" {
        change_todo_item_to_done(
            args.item_identifier
                .expect("***Please specify the item's position that you want to change as 'done'. Either you specified an empty string item, or you typed --. Both of which are not allowed. A correct example would be: 'chartodo done 3', and if a todo item existed at the third position, it would be changed to done. For more information, try --help***"));
    } else {
        command_error();
    }
}

fn command_error() {
    let writer = &mut std::io::stdout();
    writeln!(writer, "invalid command. please try again, or try --help").expect("writeln failed");
}
