pub mod sms;
use sms::{Message, Contact};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_help();
    }
    else if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
        print_version();
    }
    else if args.len() <= 1 {
        print_help();
    }
    else {
        let contacts = analyze(args);
    }

    // println!("{:#?}", args);
}

fn print_help() {
        println!("Help!"); // TODO
}

fn print_version() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");
    println!("{name} {version}", name=name, version=version);
    println!("License GPLv3: GNU GPL version 3 <http://gnu.org/licenses/gpl.html>.");
    println!("There is NO WARRANTY, to the extent permitted by law.");
    println!();
    println!("Written by {authors}.", authors=authors);
}

fn analyze(args: Vec<String>) -> Vec<Contact> {
    let filename = args.get(args.len() - 1).unwrap();
    let contacts: Vec<Contact> = Vec::new();
    // TODO
    contacts
}
