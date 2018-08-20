pub mod sms;
use sms::{Message, Contact};
use std::io::{BufReader, BufRead};
use std::fs::File;

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
        for contact in contacts {
            println!("{name} ({number})", name=contact.contact_name, number=contact.address);
            println!("\tTo   (Messsages/Chars): {message_to}/{chars_to}", message_to=contact.count_to, chars_to=contact.length_to);
            println!("\tFrom (Messsages/Chars): {message_from}/{chars_from}", message_from=contact.count_from, chars_from=contact.length_from);
        }
    }
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
    let mut contacts: Vec<Contact> = Vec::new();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim_left().starts_with("<sms ") {
            let message = sms::read_xml_line(&line);
            record(message, &mut contacts);
        }
    }
    contacts
}

fn record(message: Message, contacts: &mut Vec<Contact>) {
    let index = contacts.iter().rposition(|ref c| c.address == message.address); // Use r position because most recently used contacts tend to be at the end

    match index {
        Some(n) => {
            let contact = contacts.remove(n);
            contacts.push (
                Contact {
                    address:      contact.address,
                    contact_name: contact.contact_name,
                    count_to:     contact.count_to    + if message.type_ == 2 {1} else {0},
                    length_to:    contact.length_to   + if message.type_ == 2 {message.body.chars().count()} else {0},
                    count_from:   contact.count_from  + if message.type_ == 1 {1} else {0},
                    length_from:  contact.length_from + if message.type_ == 1 {message.body.chars().count()} else {0},
                }
            );
        }
        None => {
            contacts.push (
                Contact {
                    address:      message.address.to_string(),
                    contact_name: message.contact_name.to_string(),
                    count_to:     if message.type_ == 2 {1} else {0},
                    length_to:    if message.type_ == 2 {message.body.chars().count()} else {0},
                    count_from:   if message.type_ == 1 {1} else {0},
                    length_from:  if message.type_ == 1 {message.body.chars().count()} else {0},
                }
            )
        }
    }
}
