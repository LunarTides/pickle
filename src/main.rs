use std::{
    fs::{read_to_string, remove_file, write},
    process::Command,
};

use clap::{command, Parser as ClapParser};
use lexer::Lexer;
use parser::Parser;

mod compiler;
mod lexer;
mod parser;

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_path: String,

    #[arg(short, long)]
    output_path: String,

    #[arg(
        long,
        default_value = "false",
        help = "Disables the assembler. Writes the assembly file to disk."
    )]
    no_assemble: bool,
}

fn main() {
    let args = Args::parse();

    let text = read_to_string(args.file_path).unwrap();

    let lexer = Lexer::default();
    let tokens = lexer.tokenize(text);

    let mut parser = Parser::default();
    let root_node = parser.parse(tokens);

    let asm = compiler::compile(root_node);
    write(format!("{}.asm", args.output_path), asm).unwrap();

    if !args.no_assemble {
        Command::new("nasm")
            .args([
                "-felf64",
                format!("{}.asm", args.output_path).as_str(),
                "-o",
                format!("{}.o", args.output_path).as_str(),
            ])
            .status()
            .unwrap();

        Command::new("mold")
            .args([
                format!("{}.o", args.output_path).as_str(),
                "-o",
                args.output_path.as_str(),
            ])
            .status()
            .unwrap();

        remove_file(format!("{}.o", args.output_path)).unwrap();
        remove_file(format!("{}.asm", args.output_path)).unwrap();
    }

    println!(
        "Compilation successful. File outputted at: {}",
        if args.no_assemble {
            format!("{}.asm", args.output_path)
        } else {
            args.output_path
        },
    );
}
