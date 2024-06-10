# CHARTODO

This is a simple TODO cli application that's of my own design. It's currently a WIP and is far from usable. For more information, please look at planning.txt. I wrote it before I started on the program and it's what I use for, well, planning. 

## Why the name CHARTODO

I needed a unique name, and I like Pokemon and I also like Charmander and Charizard, and there's a joke about Rust being 'blazingly fast', so it seemed perfect to combine CHAR and TODO.

## How to install and run

First, make sure you have Rust installed. There are several ways to install:

1.
```sh-session
cargo install --git https://github.com/DashikiBulbasaur/chartodo.git --branch main
```
2. clone the repo
3. (upcoming)
```sh-session
cargo install chartodo
```

Then, to run, either

1. if you cloned the repo, go to it using your terminal, and type cargo run [ACTION], e.g., `cargo run list`
2. (upcoming) on your terminal, type chartodo [ACTION], e.g., `chartodo list`

## Usage

Show the TODO and DONE list 

```sh-session
$ chartodo list
CHARTODO
1: this
2: is
3: the
4: todo
5: list
-----
DONE
1: this
2: is
3: the
4: done
5: list
```

Add a TODO item 

```sh-session
$ chartodo add item
'item' has been added to todo

CHARTODO
1: this
2: is
3: the
4: todo
5: list
6: item
-----
DONE
1: this
2: is
3: the
4: done
5: list
```

To add a multi-word item, replace the space character with a character such as _

```sh-session
$ chartodo add multi_word_item
'multi_word_item' has been added to todo

CHARTODO
1: this
2: is
3: the
4: todo
5: list
6: item
7: multi_word_item
-----
DONE
1: this
2: is
3: the
4: done
5: list
```

## Milestones

The following functionalities are done
- [x] list 
- [x] add 'x'
- [ ] done 'x'
- [ ] rmtodo 'x'
- [ ] cleartodo
- [ ] rmdone 'x'
- [ ] notdone 'x'
- [ ] cleardone
- [ ] clearall
- [ ] help
- [ ] edit 'x' 'abc'
- [ ] changeprio 'x-y'
- [ ] switchprio 'x-y'
- [ ] addtoplace 'item' 'position'

Some major milestones 
- [ ] finish unit tests
- [ ] finish integration tests
- [ ] available on crates.io
- [ ] finish the basic functionalities
- [ ] finish the advanced functionalities
