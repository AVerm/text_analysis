pub mod sms;
use sms::{Message, Contact};
use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        // In the future, I would like it if this program could check for
        // standard input and use that instead of a file if there were no
        // additional arguments. This would mean ars.len() == 1 (the first
        // argument is the program name!), but it probably makes sense to do a
        // check for standard input before any of this so the conditionals can
        // still be generic over 1 and 0
        print_help();
    }
    else if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_help();
    }
    else if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
        print_version();
    }
    else {
        let filename = args.get(args.len() - 1).unwrap(); // Just assume that the first argument is the filename. This is a TODO for sure
        let file = match File::open(filename) {
            Ok(f)    => f,
            Err(err) => panic!("File at {} could not be opened for reading\nDetails: {}", filename, err),
        };
        let reader = BufReader::new(file); // Using a reader with the BufRead trait lets the function be more generic
    
        let contacts = analyze(reader);
        // May want to pack this into a function (accepting a function that is applied to each item!) later
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

fn analyze<R: BufRead>(reader: R) -> Vec<Contact> {
    let mut contacts: Vec<Contact> = Vec::new();

    let mut error_count = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim_left().starts_with("<sms ") {
            let message = Message::read_from_xml(&line);
            match message {
                Ok(msg) => record(msg, &mut contacts),
                Err(_err) => error_count += 1,
            }
        }
    }
    eprintln!("{} errors while reading", error_count);
    contacts
}

fn record(message: Message, contacts: &mut Vec<Contact>) {
    let index = contacts.iter().rposition(|ref contact| contact.address == message.address); // Use r position because most recently used contacts tend to be at the end
    let contact = match index {
        Some(n) => contacts.remove(n),
        None => Contact::new(message.contact_name, message.address),
    };
    contacts.push(contact.record(message));
}
