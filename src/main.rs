extern crate clap;
mod lib;
use lib::*;
use clap::*;
fn main(){
    /*
    clap_app!(myapp =>
        (version: "1.0")
        (author: "Kevin K. <kbknapp@gmail.com>")
        (about: "Does awesome things")
        (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@arg INPUT: +required "Sets the input file to use")
        (@arg debug: -d ... "Sets the level of debugging information")
        (@subcommand test =>
            (about: "controls testing features")
            (version: "1.3")
            (author: "Someone E. <someone_else@other.com>")
            (@arg verbose: -v --verbose "Print test information verbosely")
        )
    )
    .get_matches();

    */

    //! Init: 
    //! - installs mdbook | not yet
    //! - Creates MKProjects-Books Directory
    //! 
    //! Get (Subcommand): 
    //! - arg: book-name **required**
    //! - arg: format -f default tex | md not available yet
    //! 
    //! List: -l --list 
    //! lists out alll finished books
    //! 
    //! status 
    //! shows status on planned books

    let pkg = clap_app!(books_pkg => 
        (version: crate_version!())
        (author: "Mustafif Khan <Mustafif0929@gmail.com>")
        (@arg init: "Initializes the MKProjects Books Directory")
        (@arg list: -l --list "Lists out Currently finished books")
        (@arg status: "Gives status of the planned books")
        /*(@subcommand get =>
            (about: "Gets the latest MKProj Book")
            (@arg book-name: +required +takes_value "MKProjects Book Name")
            (@arg format: -f --format +takes_value "Format of the book")
        )*/
        (@arg get: -g --get +takes_value "Gets the latest MKProject Book")
    ).get_matches();
}