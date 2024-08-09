//! A simple C compiler written in Rust.
//!
//! Generally follows Nora Sandler's "Writing a C Compiler - Build a Real Programming Language from
//! Scratch (2024)". A lot of the structure and some specific code segments are either heavily
//! inspired or partially taken from the Rust compiler `rustc`.
//!
//! Current features:
//! - Compiler driver
//! - Lexing of a reduced subset of C
//! - String interning
//!
//! Planned features:
//! - Implementation up to arm64 assembly emmision
//! - Most basic language features
//! - Optimisation passes
//! - Maybe even compiling hello_world.c ...

use clap::Parser;
use core::str;
use lexer::TokenKind;
use miette::{miette, IntoDiagnostic, Result};
use std::{fs, path::Path, process::Command};

mod lexer;
mod parser;
mod span;

use crate::lexer::tokenize;

/// A simple C compiler written in Rust.
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

// TODO: Abstracted cleanup step for compilation artifacts on error
// https://doc.rust-lang.org/std/panic/fn.set_hook.html
fn main() -> Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.file);

    // Run preprocessor
    let preprc_path = path.with_extension("i");
    if let Ok(output) = Command::new("gcc")
        .args([
            "-E",
            "-P",
            path.to_str().unwrap(),
            "-o",
            preprc_path.to_str().unwrap(),
        ])
        .output()
    {
        if !output.status.success() {
            return Err(miette!(
                "Could not preprocess {}: \n{}",
                path.to_str().unwrap(),
                str::from_utf8(&output.stderr).into_diagnostic()?
            ));
        }
    } else {
        return Err(miette!(
            "Could not run gcc preprocessor. Check that everything is installed correctly."
        ));
    }

    // Read and cleanup preprocessed file
    let file = fs::read_to_string(&preprc_path).map_err(|err| {
        miette!(
            "The source file {} could not be accessed: {}",
            preprc_path.to_str().unwrap(),
            err
        )
    })?;
    std::fs::remove_file(preprc_path).into_diagnostic()?;

    // Compilation stages
    if args.lex {
        todo!()
    } else if args.parse {
        todo!()
    } else if args.codegen {
        todo!()
    }

    // Print tokens in file
    for tok in tokenize(&file).filter(|tok| tok.kind != TokenKind::Whitespace) {
        print!("{:?} ", tok);
        println!(
            "{}",
            &file[tok.span.data().lo.0 as usize..tok.span.data().hi.0 as usize]
        );
    }

    crate::span::with_session_globals(|sess| {
        for sym in sess.interner.iter() {
            println!("{:?}", sym);
        }
    });

    let asm_path = path.with_extension("s");
    let bin_path = path.with_extension("");
    // Assembler & linker step
    Command::new("gcc")
        .args([asm_path.to_str().unwrap(), "-o", bin_path.to_str().unwrap()])
        .output()
        .into_diagnostic()?;
    // Clean up emitted assembly
    std::fs::remove_file(&asm_path).map_err(|err| {
        miette!(
            "Could not delete generated assembly file {}: {}",
            asm_path.to_str().unwrap(),
            err
        )
    })?;

    Ok(())
}
