use std::{fs::File, io::Read, panic::panic_any};

use clap::{Parser, builder::Str};
pub mod ast;
pub mod interpreter;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    olang1
);

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the file to execute
    #[arg(short, long)]
    path: String,
}

use interpreter::Interpreter;

fn main() {
    let args = Args::parse();
    let mut file = match File::open(args.path) {
        Ok(f) => f,
        Err(e) => {
            panic!("Failed to open file: {e}");
        }
    };
    let mut source = String::new();
    _ = file
        .read_to_string(&mut source)
        .unwrap_or_else(|e| panic!("Failed to read file: {e}"));

    //     let source = r#"
    // def greet(name)
    //     puts "Hello, " + name + "!";
    // end;

    // greet("world");

    // x = 10;
    // y = 3;
    // puts x + y;

    // def factorial(n)
    //     if n < 2 then
    //         return 1;
    //     end;
    //     return n * factorial(n - 1);
    // end;

    // result = factorial(5);
    // puts result;

    // i = 1;
    // while i < 6 do
    //     puts i;
    //     i = i + 1;
    // end;

    // if 10 > 5 then
    //     puts "ten is greater";
    // else
    //     puts "this won't print";
    // end;
    // "#;

    let parser = olang1::ProgramParser::new();
    let program = parser.parse(&source).unwrap_or_else(|e| {
        eprintln!("Parse error: {e}");
        std::process::exit(1);
    });

    let mut interp = Interpreter::new();
    if let Err(e) = interp.run(program) {
        eprintln!("Runtime error: {e}");
        std::process::exit(1);
    }
}
