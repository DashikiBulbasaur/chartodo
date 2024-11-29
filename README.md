# CHARTODO

Chartodo is a feature-rich all-in-one todo application (to the best of my abilities). I wanted to make a todo list that I and other people would actually use. 

https://github.com/user-attachments/assets/56e800b3-ef57-4aa8-9dbb-c3f16babfce4

## How to install and run

First, make sure you have [Rust installed](https://doc.rust-lang.org/book/ch01-01-installation.html). After that, there are several ways to install the program (ordered by recommendation):

1. (upcoming, not yet available on crates.io)
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
Commands (argument chaining is available where it makes sense):
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
            change a todo item to done, using a numbered position to specify which one(s)
            example: chartodo done 3
            example: chartodo d 5 1 3 2
        notdone | nd
            reverses a done item back to a todo item
            example: chartodo nd 3
            example: chartodo notdone 3 2 1 5
        rmtodo | rmt
            remove a todo item from the list, using a numbered position to specify which one
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
                minutes, hours, days, weeks, months, years
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
```

### Tips on usage

1. Commands that take positions as arguments will ignore/reject invalid inputs such as a) non-numbers, b) 0, c) empty strings (if you can somehow do that in the terminal), and d) bigger index than the todo/done list you're trying to access
2. Spaces in the program are used to differentiate separate arguments, so multi-word tasks can instead be separated by a character such as -, e.g., multi-word-task-item
3. For the repeating tasks, the maximum interval for the repeating time is 4294967295, e.g., 4294967295 days.

## Milestones

- [x] finish the regular task commands
- [x] finish argument chaining
- [x] switch file format to json
- [x] add deadline-based todo items
- [x] add repeating todo items
- [x] testing complete for launch
- [ ] available on crates.io
- [ ] launched
- [ ] switch to proper argument chaining (if possible), i.e., chartodo -a ... -d ... -rmt ...
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
