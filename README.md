# CHARTODO

Chartodo is a simple CLI todo list program written in Rust, which includes many features that I thought would be useful. Essentially, I wanted to make a todo list I would actually use.

You can create tasks with a deadline, and tasks that repeat on a set interval.

![gif](chartodo_example_2.gif)

## How to install and run

First, make sure you have [Rust installed](https://doc.rust-lang.org/book/ch01-01-installation.html). After that, there are several ways to install the program (ordered by recommendation):

1. 
```sh-session
cargo install chartodo
```
2. via github
```sh-session
cargo install --git https://github.com/DashikiBulbasaur/chartodo.git --branch master
```
3. clone the repository
---
Then, to run, either

1. on your terminal, type chartodo [COMMAND], e.g., `chartodo list`
2. if you cloned the repo, go to it using your terminal, and type cargo run [COMMAND], e.g., `cargo run list`

## Usage
```sh-session
    Commands:
        If a command says it has chaining, it means you can include multiple separate tasks or positions
        If a command says it has range positioning, it means you can include position arguments that's a range,
        e.g., 1-6, 5-10, 3-11, 1-100
        Date format is always in year-month-day, e.g., 2099-12-25
        Time format is always in a 24-hour format, e.g., 13:58. Note that there is no space between hour and minute
        Only the following time units are allowed in repeating tasks: minutes, hours, days, weeks, months, years

        A TIP BEFORE STARTING: it's helpful to memorize regular todo/done commands, since most repeating/deadline commands
        are simply done by prefixing repeating/deadline before regular commands
            Example:
                chartodo add [task] -> adds a regular todo task
                chartodo deadline-add [task] [ending date] [ending time] -> adds a deadline todo task
                chartodo repeating-add [task] [interval] [time unit] -> adds a repeating todo task

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

        REGULAR TODO:
            add | a
                add an item to the todo list. To add a multi-word item, replace space with something like -. Has chaining
                format: chartodo add [task]
                example: chartodo add new-item
                example: chartodo add 1st-item 2nd-item 3rd-item
            done | d
                change a todo item to done, using numbered positions to specify which one(s). Has chaining and range positioning
                format: chartodo done [position]
                example: chartodo done 3
                example: chartodo d 5 1 3 2 6-9 11-15
            rmtodo | rmt
                remove a todo item from the list using numbered positions. Has chaining and range positioning
                format: chartodo rmtodo [position]
                example: chartodo rmt 4 1 5 11-15
            doneall | da
                change all todo items to done
                format: chartodo doneall
            cleartodo | ct
                clear the todo list
                format: chartodo cleartodo
            clearboth | cb
                clear both todo and done lists
                format: chartodo clearboth
            edit | e
                changes a todo item, with its position specified, to what you want
                format: chartoo edit [position] [new task]
                example: chartodo edit 3 change-item-to-this
        REGULAR DONE:
            notdone | nd
                reverses a done item back to a todo item using numbered positions. Has chaining and range positioning
                format: chartodo notdone [position]
                example: chartodo nd 3 4-6
            rmdone | rmd
                removes a done item using numbered positions. Has chaining and range positioning
                format: chartodo rmdone [position]
                example: chartodo rmd 4 5-8
            notdoneall | nda
                reverses all done items back to todo
                format: chartodo notdoneall
            cleardone | cd
                clear the done list
                format: chartodo cleardone
            

        DEADLINE TODO:
            deadline-add | dl-a
                adds a task with a day and time limit. Has chaining
                format: chartodo deadline-add [deadline task] [ending date] [ending time]
                example: chartodo dl-a go-on-a-run 2099-01-01 08:00
                example: chartodo dl-a go-shopping 2030-12-01 13:00 go-bowling 2030-12-01 15:30
            deadline-addonlydate | dl-aod
                adds a deadline task. only the date is specified and time defaults to 00:00. Has chaining
                format: chartodo deadline-addonlydate [deadline task] [ending date]
                example: chartodo dl-aod midnight 2099-12-12
            deadline-addonlytime | dl-aot
                adds a deadline task. only the time is specified and date defaults to current date. Has chaining
                format: chartodo deadline-addonlytime [deadline task] [ending time]
                example: chartodo dl-aot homework-due-today 23:59
            deadline-done | dl-d
                mark one/several deadline task(s) as done using numbered positions. Has chaining and range positioning
                format: chartodo deadline-done [position]
                example: chartodo dl-d 1
                example: chartodo dl-d 1 2 3 4 5 7-9
            deadline-rmtodo | dl-rmt
                remove one or several todo item(s) using numbered positions. Has chaining and range positioning
                format: chartodo deadline-rmtodo [position]
                example: chartodo dl-rmt 1 5-9
            deadline-doneall | dl-da
                mark the entire deadline todo list as done
                format: chartodo deadline-doneall
            deadline-cleartodo | deadline-ct
                clear the deadline todo list
                format: chartodo deadline-cleartodo
            deadline-clearboth | dl-cb
                clears both of the deadline todo and done lists
                format: chartodo deadline-clearboth
            deadline-editall | dl-ea
                edit all the parameters of a deadline todo task
                format: chartodo deadline-editall [position] [new deadline task] [new ending date] [new ending time]
                example: chartodo dl-ea 1 new-item 2150-01-01 00:00
            deadline-edittask | dl-eta
                edit the task parameter of a deadline todo task
                format: chartodo deadline-edittask [position] [new deadline task]
                example: chartodo dl-eta 1 new-item
            deadline-editdate | dl-ed
                edit the date parameter of a deadline todo task
                format: chartodo deadline-editdate [position] [new ending date]
                example: chartodo dl-ed 1 2150-01-1
            deadline-edittime | dl-eti
                edit the time parameter of a deadline todo task
                format: chartodo deadline-edittime [position] [new ending time]
                example: chartodo dl-eti 1 23:59
            deadline-editdatetime | dl-edt
                edit the date and time parameter of a deadline todo task
                format: chartodo deadline-editdatetime [position] [new ending date] [new ending time]
                example: chartodo dl-edt 1 2100-01-01 13:00
        DEADLINE DONE:
            deadline-notdone | dl-nd
                reverses a deadline done item back to todo using numbered positions. Has chaining and range positioning
                format: chartodo deadline-notdone [position]
                example: chartodo dl-nd 5 4 1 7-9
            deadline-rmdone | dl-rmd
                removes a deadline done item using numbered position. Has chaining and range positioning
                format: chartodo deadline-rmdone [position]
                example: chartodo dl-rmd 3 2 1 5-7
            deadline-notdoneall | dl-nda
                reverses all deadline done items back to todo
                format: chartodo deadline-notdoneall
            deadline-cleardone | dl-cd
                clears the deadline done list
                format: chartodo deadline-cleardone
        

        REPEATING TODO:
        note: Only the following time units are allowed in repeating tasks: minutes, hours, days, weeks, months, years
            repeating-add | rp-a
                add a repeating task with a set interval. the task starts from your current date and time. Has chaining
                format: chartodo repeating-add [repeating task] [interval] [time unit]
                example: chartodo rp-a gym 2 days
                example: chartood rp-a gym 2 days mow 1 week
            repeating-addstart | rp-as
                add a repeating task that starts on your specified datetime. Has chaining
                format: chartodo repeating-addstart [repeating task] [interval] [time unit] [starting date] [starting time]
                example: chartodo rp-as task 3 days 2099-01-01 00:00
            repeating-addend | rp-ae
                add a repeating task that ends on your specified datetime. Has chaining
                format: chartodo repeating-addend [repeating task] [interval] [time unit] [ending date] [ending time]
                example: chartodo rp-ae task 3 days 2099-01-01 00:00
            repeating-done | rp-d
                mark repeating todos as done. Has chaining and range positioning
                format: chartodo repeating-done [position]
                example: chartodo rp-d 1
                example: chartodo rp-d 1 2 3 4 5 7-9
            repeating-reset | repeating-donereset | rp-r | rp-dr
                reset the starting datetime of a repeating task to your current date and time. Has chaining and range positioning
                    functionally, this can also be used to mark a repeating task as 'done' but
                    immediately start the interval again with your current date and time
                format: chartodo repeating-reset [position]
                example: chartodo rp-dr 1 3-5
            repeating-rmtodo | rp-rmt
                remove a repeating todo task. Has chaining and range positioning
                format: chartodo repeating-rmtodo [position]
                example: chartodo rp-rmt 1 3-5
            repeating-start | rp-s
                show the starting datetime of one or more repeating tasks. Has chaining and range positioning
                format: chartodo repeating-start [position]
                example: chartodo rp-s 1 3-5
            repeating-doneall | rp-da
                mark all repeating tasks as done
                format: chartodo repeating-doneall
            repeating-cleartodo | rp-ct
                delete all of the repeating todo tasks
                format: chartodo repeating-cleartodo
            repeating-clearboth | rp-cb
                clear the repeating todo and done lists
                format: chearotod repeating-clearboth
            repeating-resetall | repeating-doneresetall | rp-ra | rp-dra
                resets the starting datetime of all repeating tasks to your current date and time
                format: chartodo repeating-resetall
            repeating-startall | rp-sa
                show the starting datetime of all repeating tasks
                format: chartodo repeating-startall
            repeating-editall | rp-ea
                edit all the parameters of a repeating task: task, interval, time unit, and starting/ending datetime
                You must specify whether it's the starting or ending datetime using keywords 'start' or 'end'
                format: chartodo repeating-editall [position] [new repeating task] [interval] [time unit] start/end [date] [time]
                example: chartodo rp-ea 1 new-repeating-task 3 days start 2000-01-01 00:00
                example: chartodo rp-ea 1 new-repeating-task 3 days end 2100-01-01 00:00
            repeating-edittask | rp-eta
                edit the task parameter of a repeating task
                format: chartodo repeating-edittask [position] [new repeating task]
                example: chartodo rp-eta 1 new-task
            repeating-editinterval | rp-ei
                edit the interval of a repeating task
                format: chartodo repeating-editinterval [position] [interval]
                example: chartodo rp-ei 1 3
            repeating-editunit | rp-eu
                edit the time unit of a repeating task
                format: chartodo repeating-editunit [position] [time unit]
                example: chartodo rp-eu 1 weeks
            repeating-editintervalunit | rp-eiu
                edit the interval and time unit of a repeating task
                format: chartodo repeating-editintervalunit [position] [interval] [time unit]
                example: chartodo rp-eiu 1 3 days
            repeating-editstart | rp-es
                edit the starting datetime of a repeating task
                format: chartodo repeating-editstart [position] [starting date] [starting time]
                example: chartodo rp-es 1 2100-12-24 13:08
            repeating-editend | rp-ee
                edit the ending datetime of a repeating task
                format: chartodo repeating-editend [position] [ending date] [ending time]
                example: chartodo rp-ee 1 2100-12-24 13:08
        REPEATING DONE:
            repeating-notdone | rp-nd
                reverse repeating dones back to todo. Has chaining and range positioning
                format: chartodo repeating-notdone [position]
                example: chartodo rp-nd 1 3-5
            repeating-rmdone | rp-rmd
                remove one/several repeating done task(s). Has chaining and range positioning
                format: chartodo repeating-rmdone [position]
                example: chartodo rp-rmd 1 3-5
            repeating-notdoneall | rp-nda
                reverse all finished repeating tasks back to todo
                format: chartodo repeating-notdoneall
            repeating-cleardone | rp-cd
                delete all of the finished repeating tasks
                format: chartodo repeating-cleardone      
```

### Tips on usage

1. Commands that take positions as arguments will ignore/reject invalid inputs such as a) non-numbers, b) 0, c) empty strings (if you can somehow do that in the terminal), and d) bigger index than the todo/done list you're trying to access
2. Spaces in the program are used to differentiate separate arguments, so multi-word tasks can instead be separated by a character such as -, e.g., multi-word-task-item
3. For the repeating tasks, the maximum interval for the repeating time is 4294967295, e.g., 4294967295 days.
4. Note that this program doesn't work on WSL

## Milestones

- [x] finish the regular task commands
- [x] finish argument chaining
- [x] switch file format to json
- [x] add deadline-based todo items
- [x] add repeating todo items
- [x] testing complete for launch
- [x] available on crates.io
- [x] add range position arguments to all applicable commands
- [ ] switch to proper argument chaining (if possible), i.e., chartodo -a ... -d ... -rmt ...
- [ ] decrease size of binary if possible, possibly through refactoring
- [ ] finish the advanced functionalities?
---
Potential features:
- [ ] regular: addtoplace 'item' 'position' (may no longer be under consideration)
- [ ] regular: changeprio 'x-y' (may no longer be under consideration)
- [ ] regular: switchprio 'x-y' (may no longer be under consideration)
- [ ] deadline: if possible, smarter system where if a date/time argument isn't present after a dl-a command, then it defaults to the current date/00:00. afterwards, possibly remove dl-aod and dl-aot
- [ ] repeating: if possible, smarter system on rp-as, rp-ae commands where if a date/time argument isn't present, then it defaults to the current date/00:00. note that for both deadline and repeating, i can just default to recognizing today/zero/0/midnight keywords
- [ ] sections within lists, e.g., (types of) homework, chores, etc.
- [ ] if possible, a move to a more conventional cli-style with Clap
- [ ] all: argument chaining on edit commands
