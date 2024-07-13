# CHARTODO

This is an all-in-one TODO cli application that's of my own design. I wanted to make a todo list that I and other people would actually use. It's currently a WIP.

## Why the name CHARTODO

I needed a unique name, and I like Pokemon and I also like Charmander and Charizard, and there's a joke about Rust being 'blazingly fast', so it seemed perfect to combine CHAR and TODO.

## How to install and run

First, make sure you have [Rust installed](https://doc.rust-lang.org/book/ch01-01-installation.html). After that, there are several ways to install the program (ordered by recommendation):

1. (upcoming, not yet available on crates.io)
```sh-session
cargo install chartodo
```
2. 
```sh-session
cargo install --git https://github.com/DashikiBulbasaur/chartodo.git --branch master
```
3. clone the repo 


Then, to run, either

1. on your terminal, type chartodo [COMMAND], e.g., `chartodo list`
2. if you cloned the repo, go to it using your terminal, and type cargo run [COMMAND], e.g., `cargo run list`

## Usage
```sh-session
Commands (argument chaining is available where it makes sense):

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
        deadline-addnotime | dl-ant
            adds a deadline task. no time is specified and it defaults to 00:00
            example: chartodo dl-ant midnight 2099-12-12
            example: chartodo dl-ant homework1-due 2100-01-01 homework2 2200-01-01
        deadline-clearboth | dl-cb
            clears both of the deadline todo and done lists
            example: chartodo dl-cb
```

### Tips on usage

1. Commands that take positions as arguments will sometimes ignore/reject invalid inputs such as a) non-numbers, b) 0, c) empty strings (if you can somehow do that in the terminal), and d) bigger index than the todo/done list you're trying to access
2. Max character len for todo items is 15, and the max len for done lists is 10. This is arbitrary and can be changed in the future, to an extent, upon a user's request.
3. Max character len for tasks is 40. This isn't arbitrary as I don't want to encourage super-long tasks that wrap in the terminal and look ugly. 

## Milestones

The following functionalities are done
- [x] list
- [x] help
- [x] clearall

- [x] add 'x'
- [x] done 'x'
- [x] rmtodo 'x'
- [x] cleartodo
- [x] doneall
- [x] cleardone
- [x] clearboth
- [x] rmdone 'x'
- [x] notdone 'x'
- [x] edit 'x' 'abc'
- [ ] addtoplace 'item' 'position' (may no longer be under consideration)
- [ ] changeprio 'x-y' (may no longer be under consideration)
- [ ] switchprio 'x-y' (may no longer be under consideration)

- [ ] deadline task commands
- [ ] repeating task commands

Some major milestones
- [x] finish the basic functionalities
- [x] finish argument chaining
- [ ] add deadline-based todo items
- [ ] add repeating todo items
- [ ] available on crates.io
- [ ] finish the advanced functionalities
