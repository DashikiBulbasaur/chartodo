mod functions;

use anyhow::{Context, Ok, Result};
use clap::Parser;
use functions::{general_commands::*, regular_tasks::{regular_todo::*, regular_done::*}};
use std::io::Write;

#[derive(Parser)]
struct Cli {
    /// The action taken
    command: String,
    /// This has several functions:
    /// 1. for commands that take positions, they would go here
    /// 2. for a command like edit, both position and edit-item would be here
    item_identifier: Option<Vec<String>>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // TODO: better error handling (anyhow crate) for commands that expect some extra arg
    match args.command.as_str() {
        "help" | "h" => {
            help();
            Ok(())
        }
        "list" | "l" => {
            list();
            Ok(())
        }
        "add" | "a" => {
            regular_tasks_add_todo(
                args
                    .item_identifier
                    .with_context(|| format!("Did not provide the todo item(s) to be added. Good example: chartodo {} new-item, or chartodo {} item next-item one-more-item. If you have questions, try chartodo help or chartodo --help", args.command, args.command))?
            );
            list();
            Ok(())
        }
        "done" | "d" => {
            regular_tasks_change_todo_to_done(
                args
                    .item_identifier
                    .with_context(|| format!("Did not provide the todo item(s) to be changed to done. Good example: chartodo {} 3, or chartodo {} 3 4 5. If you have questions, try chartodo help or chartodo --help", args.command, args.command))?
            );
            list();
            Ok(())
        }
        "rmtodo" | "rmt" => {
            regular_tasks_remove_todo(
                args
                    .item_identifier
                    .with_context(|| format!("Did not provide the todo item(s) to be removed. Good example: chartodo {} 3, or chartodo {} 3 4 5. If you have more questions, try chartodo help or chartodo --help", args.command, args.command))?
            );
            list();
            Ok(())
        }
        "cleartodo" | "ct" => {
            regular_tasks_clear_todo();
            list();
            Ok(())
        }
        "doneall" | "da" => {
            regular_tasks_change_all_todo_to_done();
            list();
            Ok(())
        }
        "cleardone" | "cd" => {
            regular_tasks_clear_done();
            list();
            Ok(())
        }
        "clearregular" | "cr" => {
            clear_regular_tasks();
            list();
            Ok(())
        }
        "rmdone" | "rmd" => {
            regular_tasks_remove_done(
                args
                    .item_identifier
                    .with_context(|| format!("Did not provide the done item to be removed. Good example: chartodo {} 3, or chartodo {} 3 4 5. If you have more questions, try chartodo help or chartodo --help", args.command, args.command))?
            );
            list();
            Ok(())
        }
        "notdone" | "nd" => {
            regular_tasks_not_done(
                args
                    .item_identifier
                    .with_context(|| format!("Did not provide the done item to be reversed back to todo. Good example: chartodo {} 3, or chartodo {} 3 4 5. If you have more questions, try chartodo help or chartodo --help", args.command, args.command))?
            );
            list();
            Ok(())
        }
        "edit" | "e" => {
            regular_tasks_edit_todo(
                args
                    .item_identifier
                    .with_context(|| format!("Did not provide the todo item to be edited. Good example: chartodo {} 3 abc. If you have more questions, try chartodo help or chartodo --help", args.command))?,
            );
            list();
            Ok(())
        }
        "notdoneall" | "nda" => {
            regular_tasks_reverse_all_dones();
            list();
            Ok(())
        }
        "clearall" | "ca" => {
            clear_all_lists();
            list();
            Ok(())
        }
        "" => {
            // note: seems like it's hard for the user to reach this
            no_arg_command();
            Ok(())
        }
        _ => {
            command_error();
            Ok(())
        }
    }
}

fn no_arg_command() {
    let writer = &mut std::io::stdout();
    writeln!(writer, "You must provide a command. Try chartodo help.").expect("writeln failed");
}

fn command_error() {
    let writer = &mut std::io::stdout();
    writeln!(
        writer,
        "Invalid command. please try again, or try chartodo help"
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
        example: chartodo help
    list, l
        show the todo list
        example: chartodo list
    add, a
        add an item to the todo list. To add a multi-word item, replace space with something like -
        example: chartodo add item
        example: chartodo add new-item
        example: chartodo add 1st-item 2nd-item 3rd-item
    done, d
        change a todo item to done, using a numbered position to specify which one
        example: chartodo done 3
        example: chartodo d 5 1 3 2
    rmtodo, rmt
        remove a todo item from the list, using a numbered position to specify which one
        example: chartodo rmt 4
        example: chartodo rmt 4 3 2
    cleartodo, ct
        clear the todo list
        example: chartodo cleartodo
    doneall, da
        change all todo items to done
        example: chartodo da
    cleardone, cd
        clear the done list
        example: chartodo cd
    clearregular, cr
        clear the regular todo and done lists
        example: chartodo clearregular
    rmdone, rmd
        removes a done item at the specified position
        example: chartodo rmd 4
        exmaple: chartodo rmdone 1 2 3
    notdone, nd
        reverses a done item back to a todo item
        example: chartodo nd 3
        example: chartodo notdone 3 2 1 5
    edit, e
        changes a todo item, with its position specified, to what you want
        example: chartodo edit 3 change-item-to-this
    notdoneall, nda
        reverses all done items back to todo
        example: chartodo nda
    "
    )
    .expect("writeln failed");
}
