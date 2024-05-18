use std::{
    io::{self, Write},
    process,
};

mod excution;

mod parser;

fn main() {
    let mut line = String::new();
    loop {
        print!("todo-app #> ");
        io::stdout().flush().expect("error while fulshing");
        std::io::stdin()
            .read_line(&mut line)
            .map_err(|err| {
                eprintln!("{}", err);
                process::exit(1)
            })
            .ok();
        match parser::parse_line(&line) {
            Err(err) => {
                err.handle();
                line.clear();
                continue;
            }
            Ok(cmd) => {
                println!("{:?}", cmd);
            }
        };
        line.clear();
    }
}
