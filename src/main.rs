mod functions;

use anyhow::{Context, Ok, Result};
use clap::Parser;
use functions::{
    deadline_tasks::{deadline_done::*, deadline_todo::*},
    general_commands::*,
    regular_tasks::{regular_done::*, regular_todo::*},
    repeating_tasks::{repeating_done::*, repeating_todo::*},
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

    // since printing the list is separate from normal commands (due to how repeating tasks are handled), and since functions
    // will print to the terminal if an user error occurs, to avoid printing both the list and error if an error occurs,
    // we'll flag via bool for an error from a fn (if necessary) and won't print the list if it was tripped
    match args.command.as_str() {
        "help" | "h" if args.item_identifier.is_none() => {
            help();
            Ok(())
        }
        "list" | "l" if args.item_identifier.is_none() => {
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
            let error_status = regular_tasks_change_todo_to_done(
                args
                    .item_identifier
                    .with_context(|| format!("Did not provide the todo item(s) to be changed to done. Good example: chartodo {} 3, or chartodo {} 3 4 5. If you have questions, try chartodo help or chartodo --help", args.command, args.command))?
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "rmtodo" | "rmt" => {
            let error_status = regular_tasks_remove_todo(
                args
                    .item_identifier
                    .with_context(|| format!("Did not provide the todo item(s) to be removed. Good example: chartodo {} 3, or chartodo {} 3 4 5. If you have more questions, try chartodo help or chartodo --help", args.command, args.command))?
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "cleartodo" | "ct" if args.item_identifier.is_none() => {
            let error_status = regular_tasks_clear_todo();
            if !error_status {
                list();
            }

            Ok(())
        }
        "doneall" | "da" if args.item_identifier.is_none() => {
            let error_status = regular_tasks_change_all_todo_to_done();
            if !error_status {
                list();
            }

            Ok(())
        }
        "cleardone" | "cd" if args.item_identifier.is_none() => {
            let error_status = regular_tasks_clear_done();
            if !error_status {
                list()
            }

            Ok(())
        }
        "clearboth" | "cb" if args.item_identifier.is_none() => {
            let error_status = clear_regular_tasks();
            if !error_status {
                list();
            }

            Ok(())
        }
        "rmdone" | "rmd" => {
            let error_status = regular_tasks_remove_done(
                args
                    .item_identifier
                    .with_context(|| format!("Did not provide the done item to be removed. Good example: chartodo {} 3, or chartodo {} 3 4 5. If you have more questions, try chartodo help or chartodo --help", args.command, args.command))?
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "notdone" | "nd" => {
            let error_status = regular_tasks_not_done(
                args
                    .item_identifier
                    .with_context(|| format!("Did not provide the done item to be reversed back to todo. Good example: chartodo {} 3, or chartodo {} 3 4 5. If you have more questions, try chartodo help or chartodo --help", args.command, args.command))?
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "edit" | "e" => {
            let error_status = regular_tasks_edit_todo(
                args
                    .item_identifier
                    .with_context(|| format!("Did not provide the todo item to be edited. Good example: chartodo {} 3 abc. If you have more questions, try chartodo help or chartodo --help", args.command))?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "notdoneall" | "nda" if args.item_identifier.is_none() => {
            let error_status = regular_tasks_reverse_all_dones();
            if !error_status {
                list();
            }

            Ok(())
        }
        "deadline-add" | "dl-a" => {
            let error_status = deadline_tasks_add(
                args.item_identifier
                    .context("didn't provide a deadline add argument")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "deadline-addonlydate" | "dl-aod" => {
            let error_status = deadline_tasks_add_no_time(
                args.item_identifier
                    .context("didn't provide a deadline-addonlydate argument")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "deadline-addonlytime" | "dl-aot" => {
            let error_status = deadline_tasks_add_no_date(
                args.item_identifier
                    .context("didn't provide a deadline-addonlytime argument")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "deadline-done" | "dl-d" => {
            let error_status = deadline_tasks_done(
                args.item_identifier
                    .context("didn't provide a deadline-done argument")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "deadline-rmtodo" | "dl-rmt" => {
            let error_status = deadline_tasks_rmtodo(
                args.item_identifier
                    .context("didn't provide a deadline-rmtodo argument")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "deadline-cleartodo" | "dl-ct" if args.item_identifier.is_none() => {
            let error_status = deadline_tasks_clear_todo();
            if !error_status {
                list();
            }

            Ok(())
        }
        "deadline-doneall" | "dl-da" if args.item_identifier.is_none() => {
            let error_status = deadline_tasks_done_all();
            if !error_status {
                list();
            }

            Ok(())
        }
        "deadline-editall" | "dl-ea" => {
            let error_status = deadline_tasks_edit_all(
                args.item_identifier
                    .context("didn't provide arguments for deadline-editall")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "deadline-edittask" | "dl-eta" => {
            let error_status = deadline_tasks_edit_task(
                args.item_identifier
                    .context("didn't provide arguments for deadline-edittask")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "deadline-editdate" | "dl-ed" => {
            let error_status = deadline_tasks_edit_date(
                args.item_identifier
                    .context("didn't provide arguments for deadline-editdate")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "deadline-edittime" | "dl-eti" => {
            let error_status = deadline_tasks_edit_time(
                args.item_identifier
                    .context("didn't provide arguments for deadline-edittime")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "deadline-clearboth" | "dl-cb" if args.item_identifier.is_none() => {
            let error_status = clear_deadline_tasks();
            if !error_status {
                list();
            }

            Ok(())
        }
        "deadline-rmdone" | "dl-rmd" => {
            let error_status = deadline_tasks_rmdone(
                args.item_identifier
                    .context("didn't provide arguments for deadline-rmdone")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "deadline-notdone" | "dl-nd" => {
            let error_status = deadline_tasks_not_done(
                args.item_identifier
                    .context("didn't provide arguments for deadline-notdone")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "deadline-cleardone" | "dl-cd" if args.item_identifier.is_none() => {
            let error_status = deadline_tasks_clear_done();
            if !error_status {
                list();
            }

            Ok(())
        }
        "deadline-notdoneall" | "dl-nda" if args.item_identifier.is_none() => {
            let error_status = deadline_tasks_notdoneall();
            if !error_status {
                list();
            }

            Ok(())
        }
        "deadline-editdatetime" | "dl-edt" => {
            let error_status = deadline_tasks_edit_datetime(
                args.item_identifier
                    .context("didn't provide arguments for deadline-editdatetime")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-add" | "rp-a" => {
            let error_status = repeating_tasks_add(
                args.item_identifier
                    .context("didn't provide arguments for repeating-add")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-addstart" | "rp-as" => {
            let error_status = repeating_tasks_add_start_datetime(
                args.item_identifier
                    .context("didn't provide arguments for repeating-addstart")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-addend" | "rp-ae" => {
            let error_status = repeating_tasks_add_end(
                args.item_identifier
                    .context("didn't provide arguments for repeating-addend")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-done" | "rp-d" => {
            let error_status = repeating_tasks_done(
                args.item_identifier
                    .context("didn't provide arguments for repeating-done")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-reset" | "repeating-donereset" | "rp-r" | "rp-dr" => {
            let error_status = repeating_tasks_reset_original_datetime_to_now(
                args.item_identifier
                    .context("didn't provide arguments for repeating-reset")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-notdone" | "rp-nd" => {
            let error_status = repeating_tasks_not_done(
                args.item_identifier
                    .context("didn't provide arguments for repeating-notdone")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-rmtodo" | "rp-rmt" => {
            let error_status = repeating_tasks_rmtodo(
                args.item_identifier
                    .context("didn't provide arguments for repeating-rmtodo")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-rmdone" | "rp-rmd" => {
            let error_status = repeating_tasks_rmdone(
                args.item_identifier
                    .context("didn't provide arguments for repeating-rmdone")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-doneall" | "rp-da" if args.item_identifier.is_none() => {
            let error_status = repeating_tasks_doneall();
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-notdoneall" | "rp-nda" if args.item_identifier.is_none() => {
            let error_status = repeating_tasks_not_done_all();
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-cleartodo" | "rp-ct" if args.item_identifier.is_none() => {
            let error_status = repeating_tasks_clear_todo();
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-cleardone" | "rp-cd" if args.item_identifier.is_none() => {
            let error_status = repeating_tasks_clear_done();
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-clearboth" | "rp-cb" if args.item_identifier.is_none() => {
            let error_status = clear_repeating_tasks();
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-start" | "rp-s" => {
            let show_starts = repeating_tasks_show_start(
                args.item_identifier
                    .context("didn't provide arguments for repeating-start")?,
            );
            let writer = &mut std::io::stdout();
            writeln!(writer, "{}", show_starts).expect("writeln failed");
            Ok(())
        }
        "repeating-resetall" | "rp-ra" | "repeating-doneresetall" | "rp-dra"
            if args.item_identifier.is_none() =>
        {
            let error_status = repeating_tasks_resetall();
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-startall" | "rp-sa" if args.item_identifier.is_none() => {
            let show_starts = repeating_tasks_showstartall();
            let writer = &mut std::io::stdout();
            writeln!(writer, "{}", show_starts).expect("writeln failed");
            Ok(())
        }
        "repeating-editall" | "rp-ea" => {
            let error_status = repeating_tasks_edit_all(
                args.item_identifier
                    .context("didn't provide arguments for repeating-editall")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-edittask" | "rp-eta" => {
            let error_status = repeating_tasks_edit_task(
                args.item_identifier
                    .context("didn't provide arguments for repeating-edittask")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-editinterval" | "rp-ei" => {
            let error_status = repeating_tasks_edit_interval(
                args.item_identifier
                    .context("didn't provide arguments for repeating-editinterval")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-editintervalunit" | "rp-eiu" => {
            let error_status = repeating_tasks_edit_interval_unit(
                args.item_identifier
                    .context("didn't provide arguments for repeating-editintervalunit")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        // note: I don't like unit, it's too vague. but time unit is also too long
        "repeating-editunit" | "rp-eu" => {
            let error_status = repeating_tasks_edit_time_unit(
                args.item_identifier
                    .context("didn't provide arguments for repeating-editunit")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-editstart" | "rp-es" => {
            let error_status = repeating_tasks_edit_start(
                args.item_identifier
                    .context("didn't provide arguments for repeating-editstart")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "repeating-editend" | "rp-ee" => {
            let error_status = repeating_tasks_edit_end(
                args.item_identifier
                    .context("didn't provide arguments for repeating-editend")?,
            );
            if !error_status {
                list();
            }

            Ok(())
        }
        "clearall" | "ca" if args.item_identifier.is_none() => {
            let error_status = clear_all_lists();
            if !error_status {
                list();
            }

            Ok(())
        }
        "clearall-regular" | "ca-r" if args.item_identifier.is_none() => {
            let error_status = clear_regular_tasks();
            if !error_status {
                list();
            }

            Ok(())
        }
        "clearall-deadline" | "ca-d" if args.item_identifier.is_none() => {
            let error_status = clear_deadline_tasks();
            if !error_status {
                list();
            }

            Ok(())
        }
        "clearall-repeating" | "ca-rp" if args.item_identifier.is_none() => {
            let error_status = clear_repeating_tasks();
            if !error_status {
                list();
            }

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
        "Invalid command. Please try again, or try chartodo help"
    )
    .expect("writeln failed");
}

fn help() {
    let writer = &mut std::io::stdout();
    writeln!(
        writer,
        "
    CHARTODO is a simple command-line-interface (CLI) todo list application

    Note that for commands that take positions, the general format is always the following:
        chartodo ~command ~position(s)
        e.g., chartodo rmtodo 1, or chartodo rmtodo 5 1 2 12 3

    help | h
        show help
        example: chartodo help
    list | l
        show the todo list
        example: chartodo list
    clearall | ca
        clear everything (TODO, DEADLINE, REPEATING)
        example: chartodo ca
    clearall-regular | ca-r
        clear all regular todo and done tasks
        example: chartodo ca-r
    clearall-deadline | ca-d
        clear all deadline todo and done tasks
        example: chartodo ca-d
    clearall-repeating | ca-rp
        clear all repeating todo and done tasks
        example: chartodo ca-rp

    TODO:
        add | a
            add an item to the todo list. To add a multi-word item, replace space with something like -
            example: chartodo add item
            example: chartodo add new-item
            example: chartodo add 1st-item 2nd-item 3rd-item
        done | d
            change one or several todo item(s) to done, using the task's position to specify
            example: chartodo done 3
            example: chartodo d 5 1 3 2
        notdone | nd
            reverses a done item back to a todo item
            example: chartodo nd 3
            example: chartodo notdone 3 2 1 5
        rmtodo | rmt
            remove a todo item from the list, using a numbered position to specify which one(s)
            example: chartodo rmt 4
            example: chartodo rmt 4 3 2
        rmdone | rmd
            removes a done item at the specified position
            example: chartodo rmd 4
            example: chartodo rmdone 1 2 3
        doneall | da
            change all todo items to done
            example: chartodo da
        notdoneall | nda
            reverses all done items back to todo
            example: chartodo nda
        cleartodo | ct
            clear the todo list
            example: chartodo cleartodo
        cleardone | cd
            clear the done list
            example: chartodo cd
        clearboth | cb
            clear both todo and done lists
            example: chartodo clearall
        edit | e
            changes a todo item, with its position specified, to what you want
            example: chartodo edit 3 change-item-to-this

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
        deadline-notdone | dl-nd
            reverses a deadline done item back to todo
            example: chartodo dl-nd 1
            example: chartodo dl-nd 1 2 3 4 5
        deadline-rmtodo | dl-rmt
            remove one or several todo item(s)
            example: chartodo dl-rmt 1
            example: chartodo dl-rmt 1 2 3 4 5
        deadline-rmdone | dl-rmd
            removes a deadline done item
            example: chartodo dl-rmd 1
            example: chartodo dl-rmd 1 2 3 4 5
        deadline-doneall | dl-da
            mark the entire deadline todo list as done
            example: chartodo dl-da
        deadline-notdoneall | dl-nda
            reverses all deadline done items back to todo
            example: chartodo dl-nda
        deadline-cleartodo | deadline-ct
            clear the deadline todo list
            example: chartodo dl-ct
        deadline-cleardone | dl-cd
            clears the deadline done list
            example: chartodo dl-cd
        deadline-clearboth | dl-cb
            clears both of the deadline todo and done lists
            example: chartodo dl-cb
        deadline-editall | dl-ea
            edit all the parameters of a deadline todo task
            example: chartodo dl-ea 1 new-item 2150-01-01 00:00
        deadline-edittask | dl-eta
            edit the task parameter of a deadline todo task
            example: chartodo dl-eta 1 new-item
        deadline-editdate | dl-ed
            edit the date parameter of a deadline todo task
            example: chartodo dl-ed 1 2150-01-1
        deadline-edittime | dl-eti
            edit the time parameter of a deadline todo task
            example: chartodo dl-eti 1 23:59
        deadline-editdatetime | dl-edt
            edit the date and time parameter of a deadline todo task
            example: chartodo dl-edt 1 2100-01-01 13:00

    REPEATING:
        repeating-add | rp-a
            add a repeating task. the task starts from your current date and time
            note that for the repeating time interval, only the following time units are allowed:
                seconds, minutes, hours, days, weeks, months, years
            example: chartodo rp-a gym 2 days
            example: chartood rp-a gym 2 days mow 1 week
        repeating-addstart | rp-as
            add a repeating task that starts on your specified datetime
            example: chartodo rp-as task 3 days 2099-01-01 00:00
            example: charotodo rp-as task 3 days 2099-01-01 00:00 task2 4 days 2100-01-01 03:03
        repeating-addend | rp-ae
            add a repeating task that ends on your specified datetime
            example: chartodo rp-ae task 3 days 2099-01-01 00:00
            example: charotodo rp-ae task 3 days 2099-01-01 00:00 task2 4 days 2100-01-01 03:03
        repeating-done | rp-d
            mark repeating todos as done
            example: chartodo rp-d 1
            example: chartodo rp-d 1 2 3 4 5
        repeating-reset | repeating-donereset | rp-r | rp-dr
            reset the starting datetime of a repeating task to your current date and time
                functionally, this can also be used to mark a repeating task as 'done' but
                immediately start the interval again with your current date and time
            example: chartodo rp-r 1 | chartodo rp-dr 1
            example: chartodo rp-r 1 2 3 4 5 | chartodo rp-dr 1 2 3 4 5
        repeating-notdone | rp-nd
            reverse repeating dones back to todo
            example: chartodo rp-nd 1
            example: chartodo rp-nd 1 2 3 4 5
        repeating-rmtodo | rp-rmt
            remove a repeating todo task
            example: chartodo rp-rmt 1
            example: chartodo rp-rmt 1 2 3 4 5
        repeating-rmdone | rp-rmd
            remove one/several repeating done task(s)
            example: chartodo rp-rmd 1
            example: chartodo rp-rmd 1 2 3 4 5
        repeating-doneall | rp-da
            mark all repeating tasks as done
            example: chartodo rp-da
        repeating-notdoneall | rp-nda
            reverse all finished repeating tasks back to todo
            example: chartodo rp-nda
        repeating-cleartodo | rp-ct
            delete all of the repeating todo tasks
            example: chartodo rp-ct
        repeating-cleardone | rp-cd
            delete all of the finished repeating tasks
            example: chartodo rp-cd
        repeating-clearboth | rp-cb
            clear the repeating todo and done lists
            example: chartodo rp-cb
        repeating-start | rp-s
            show the starting datetime of one or more repeating tasks
            example: chartodo rp-s 1
            example: chartodo rp-s 1 2 3 4 5
        repeating-resetall | repeating-doneresetall | rp-ra | rp-dra
            resets the starting datetime of all repeating tasks to your current date and time
            example: chartodo rp-ra | chartodo rp-dra
        repeating-startall | rp-sa
            show the starting datetime of all repeating tasks
            example: chartodo rp-sa 
        repeating-editall | rp-ea
            edit all the parameters of a repeating task: task, interval, time unit, and starting/ending datetime
            example: chartodo rp-ea 1 new-repeating-task 3 days start 2000-01-01
            example: chartodo rp-ea 1 new-repeating-task 3 days end 2100-01-01
        repeating-edittask | rp-eta
            edit the task parameter of a repeating task
            example: chartodo rp-eta 1 new-task
        repeating-editinterval | rp-ei
            edit the interval of a repeating task
            example: chartodo rp-ei 1 3
            '1' would be the position of the repeating task and '3' would be the new interval,
                i.e., change it to '3 days'
        repeating-editunit | rp-eu
            edit the time unit of a repeating task
            example: chartodo rp-eu 1 weeks
        repeating-editintervalunit | rp-eiu
            edit the interval and time unit of a repeating task
            example: chartodo rp-eiu 1 3 days
        repeating-editstart | rp-es
            edit the starting datetime of a repeating task
            example: chartodo rp-es 2100-12-24 13:08
        repeating-editend | rp-ee
            edit the ending datetime of a repeating task
            example: chartodo rp-ee 2100-12-24 13:08
    "
    )
    .expect("writeln failed");
}
