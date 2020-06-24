use core::entity;
use std::io::prelude::*;
use std::net::TcpStream;
use std::io::{stdin,stdout,Write};

fn render(msgs: &Vec<String>) {
    for msg in msgs {
        // Note: should be println but we aren't stripping newline from msg
        print!("Zac: {}", msg);
    }
}

fn main() -> std::io::Result<()> {
    let mut messages = vec![];

    print!("{}[2J", 27 as char);
    
    loop {
        let mut s = String::new();
        print!(">> ");

        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");

        print!("{}[2J", 27 as char);

        messages.push(s);
        render(&messages);

    }
    Ok(())
}
