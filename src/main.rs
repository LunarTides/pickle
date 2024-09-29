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
}

fn main() {
    let args = Args::parse();

    let text = read_to_string(args.file_path).unwrap();

    let lexer = Lexer::default();
    let tokens = lexer.tokenize(text);

    let parser = Parser::default();
    let exit_node = parser.parse(tokens);

    let asm = compiler::compile(exit_node);
    write(format!("{}.asm", args.output_path), asm).unwrap();

    Command::new("nasm")
        .args([
            "-felf64",
            format!("{}.asm", args.output_path).as_str(),
            "-o",
            format!("{}.o", args.output_path).as_str(),
        ])
        .status()
        .unwrap();

    Command::new("ld")
        .args([
            format!("{}.o", args.output_path).as_str(),
            "-o",
            args.output_path.as_str(),
        ])
        .status()
        .unwrap();

    remove_file(format!("{}.asm", args.output_path)).unwrap();
    remove_file(format!("{}.o", args.output_path)).unwrap();

    println!(
        "Compilation successful. File outputted at: {}",
        args.output_path
    );
}
