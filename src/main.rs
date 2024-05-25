/* TODO items
* 1. add cli parsing arguments
* status: DONE
*
* 2. add the commands in lib
*
* 3. add --help (how u do that?)
* status: still need to add help sections for the various commands
*
* 4. check the subcommands/optional commands in clap
* status: DONE?
*
* 5. add the `add x` functionality
* */

use chartodo::list;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The action taken
    command: String,
    /// If applicable, the name/position of the TODO/DONE item
    item_identifier: Option<String>,
    /// If changing a TODO item, this is where you specify what to change it to
    edit_todo: Option<String>,
}

fn main() {
    let args = Cli::parse();

    if &args.command == "list" {
        list();
    }
}
