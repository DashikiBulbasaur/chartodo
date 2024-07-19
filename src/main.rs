mod functions;

use anyhow::{Context, Ok, Result};
use clap::Parser;
use functions::{
    deadline_tasks::{
        deadline_done::{
            deadline_tasks_clear_done, deadline_tasks_not_done, deadline_tasks_notdoneall,
            deadline_tasks_rmdone,
        },
        deadline_todo::*,
    },
    general_commands::*,
    regular_tasks::{regular_done::*, regular_todo::*},
};
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
        "clearboth" | "cb" => {
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
        "deadline-add" | "dl-a" => {
            deadline_tasks_add(
                args.item_identifier
                    .context("didn't provide a deadline add argument")?,
            );
            list();
            Ok(())
        }
        "deadline-addonlydate" | "dl-aod" => {
            deadline_tasks_add_no_time(
                args.item_identifier
                    .context("didn't provide a deadline-addonlydate argument")?,
            );
            list();
            Ok(())
        }
        "deadline-addonlytime" | "dl-aot" => {
            deadline_tasks_add_no_date(
                args.item_identifier
                    .context("didn't provide a deadline-addonlytime argument")?,
            );
            list();
            Ok(())
        }
        "deadline-done" | "dl-d" => {
            deadline_tasks_done(
                args.item_identifier
                    .context("didn't provide a deadline-done argument")?,
            );
            list();
            Ok(())
        }
        "deadline-rmtodo" | "dl-rmt" => {
            deadline_tasks_rmtodo(
                args.item_identifier
                    .context("didn't provide a deadline-rmtodo argument")?,
            );
            list();
            Ok(())
        }
        "deadline-cleartodo" | "dl-ct" => {
            deadline_tasks_clear_todo();
            list();
            Ok(())
        }
        "deadline-doneall" | "dl-da" => {
            deadline_tasks_done_all();
            list();
            Ok(())
        }
        "deadline-editall" | "dl-ea" => {
            deadline_tasks_edit_all(
                args.item_identifier
                    .context("didn't provide arguments for deadline-editall")?,
            );
            list();
            Ok(())
        }
        "deadline-edittask" | "dl-eta" => {
            deadline_tasks_edit_task(
                args.item_identifier
                    .context("didn't provide arguments for deadline-edittask")?,
            );
            list();
            Ok(())
        }
        "deadline-editdate" | "dl-ed" => {
            deadline_tasks_edit_date(
                args.item_identifier
                    .context("didn't provide arguments for deadline-editdate")?,
            );
            list();
            Ok(())
        }
        "deadline-edittime" | "dl-eti" => {
            deadline_tasks_edit_time(
                args.item_identifier
                    .context("didn't provide arguments for deadline-edittime")?,
            );
            list();
            Ok(())
        }
        "deadline-clearboth" | "dl-cb" => {
            clear_deadline_tasks();
            list();
            Ok(())
        }
        "deadline-rmdone" | "dl-rmd" => {
            deadline_tasks_rmdone(
                args.item_identifier
                    .context("didn't provide arguments for deadline-rmdone")?,
            );
            list();
            Ok(())
        }
        "deadline-notdone" | "dl-nd" => {
            deadline_tasks_not_done(
                args.item_identifier
                    .context("didn't provide arguments for deadline-notdone")?,
            );
            list();
            Ok(())
        }
        "deadline-cleardone" | "dl-cd" => {
            deadline_tasks_clear_done();
            list();
            Ok(())
        }
        "deadline-notdoneall" | "dl-nda" => {
            deadline_tasks_notdoneall();
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

    help | h
        show help
        example: chartodo help
    list | l
        show the todo list
        example: chartodo list
    clearall | ca
        clear everything (TODO, DEADLINE, REPEATING)
        example: chartodo ca

    TODO:
        add | a
            add an item to the todo list. To add a multi-word item, replace space with something like -
            example: chartodo add item
            example: chartodo add new-item
            example: chartodo add 1st-item 2nd-item 3rd-item
        done | d
            change a todo item to done, using a numbered position to specify which one
            example: chartodo done 3
            example: chartodo d 5 1 3 2
        rmtodo | rmt
            remove a todo item from the list, using a numbered position to specify which one
            example: chartodo rmt 4
            example: chartodo rmt 4 3 2
        cleartodo | ct
            clear the todo list
            example: chartodo cleartodo
        doneall | da
            change all todo items to done
            example: chartodo da
        cleardone | cd
            clear the done list
            example: chartodo cd
        clearboth | cb
            clear both todo and done lists
            example: chartodo clearall
        rmdone | rmd
            removes a done item at the specified position
            example: chartodo rmd 4
            exmaple: chartodo rmdone 1 2 3
        notdone | nd
            reverses a done item back to a todo item
            example: chartodo nd 3
            example: chartodo notdone 3 2 1 5
        edit | e
            changes a todo item, with its position specified, to what you want
            example: chartodo edit 3 change-item-to-this
        notdoneall | nda
            reverses all done items back to todo
            example: chartodo nda

    DEADLINE:
        deadline-add | dl-a
            adds a task with a day and time limit. date format: yy-mm-dd. time format: 24-hour
            example: chartodo dl-a go-on-a-run 2099-01-01 08:00
            example: chartodo dl-a go-shopping 2030-12-01 13:00 go-bowling 2030-12-01 15:30
            note that there is no space in the time format
        deadline-addonlydate | dl-aod
            adds a deadline task. only the date is specified and time defaults to 00:00
            example: chartodo dl-aod midnight 2099-12-12
            example: chartodo dl-aod homework1-due 2100-01-01 homework2 2200-01-01
        deadline-addonlytime | dl-aot
            adds a deadline task. only the time is specified and date defaults to current date
            example: chartodo dl-aot homework-due-today 23:59
            example: chartodo dl-aot essay-due-today 23:59
        deadline-done | dl-d
            mark one/several deadline task(s) as done
            example: chartodo dl-d 1
            example: chartodo dl-d 1 2 3 4 5
        deadline-rmtodo | dl-rmt
            remove one or several todo item(s)
            example: chartodo dl-rmt 1
            example: chartodo dl-rmt 1 2 3 4 5
        deadline-cleartodo | deadline-ct
            clear the deadline todo list
            example: chartodo dl-ct
        deadline-doneall | dl-da
            mark the entire deadline todo list as done
            example: chartodo dl-da
        deadline-editall | dl-ea
            edit all the parameters of a deadline todo task
            example: chartodo dl-ea 1 new-item 2150-01-01 00:00
        deadline-edittask | dl-eta
            edit the task parameter of a deadline todo task
            example: chartodo dl-eta 1 new-item
        deadline-editdate | dl-ed
            edit the date parameter of a deadline todo task
            example: chartodo dl-eta 1 2150-01-1
        deadline-edittime | dl-eti
            edit the time parameter of a deadline todo task
            example: chartodo dl-eta 1 23:59
        deadline-clearboth | dl-cb
            clears both of the deadline todo and done lists
            example: chartodo dl-cb
        deadline-rmdone | dl-rmd
            removes a deadline done item
            example: chartodo dl-rmd 1
            example: chartodo dl-rmd 1 2 3 4 5
        deadline-notdone | dl-nd
            reverses a deadline done item back to todo
            example: chartodo dl-nd 1
            example: chartodo dl-nd 1 2 3 4 5
        deadline-cleardone | dl-cd
            clears the deadline done list
            example: chartodo dl-cd
        deadline-notdoneall | dl-nda
            reverses all deadline done items back to todo
            example: chartodo dl-nda
    "
    )
    .expect("writeln failed");
}
