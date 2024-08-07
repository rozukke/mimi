use clap::{Error, Parser};
use lexer::TokenKind;
use std::{fs, path::Path, process::Command};

mod lexer;

use crate::lexer::tokenize;

/// A simple C compiler writted in Rust.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// C file to run compilation on
    file: String,

    /// Do not run past lexer
    #[arg(short, long)]
    lex: bool,

    /// Do not run past parser
    #[arg(short, long)]
    parse: bool,

    /// Do not run past assembly generation
    #[arg(short, long)]
    codegen: bool,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let path = Path::new(&args.file);

    // Run preprocessor
    let preprc_path = path.with_extension("i");
    Command::new("gcc")
        .args([
            "-E",
            "-P",
            path.to_str().unwrap(),
            "-o",
            preprc_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    // Read and cleanup preprocessed file
    let file = fs::read_to_string(&preprc_path)?;
    std::fs::remove_file(preprc_path)?;

    // Compilation stages
    if args.lex {
        todo!()
    } else if args.parse {
        todo!()
    } else if args.codegen {
        todo!()
    }

    // Print tokens in file
    for tok in tokenize(&file) {
        if tok.kind != TokenKind::Whitespace {
            println!("{tok:?}");
            println!(
                "{}",
                &file[tok.span.loc() as usize..(tok.span.loc() + tok.span.len() as u32) as usize]
            );
        }
    }

    let asm_path = path.with_extension("s");
    let bin_path = path.with_extension("");
    Command::new("gcc")
        .args([asm_path.to_str().unwrap(), "-o", bin_path.to_str().unwrap()])
        .output()
        .unwrap();
    match std::fs::remove_file(&asm_path) {
        Ok(_) => (),
        Err(err) => println!(
            "Could not remove file {}: {err}",
            asm_path.to_str().unwrap()
        ),
    }

    Ok(())
}
