use clap::{Error, Parser};
use std::{fs, path::Path, process::Command};

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
    let _file = fs::read_to_string(&preprc_path).unwrap();
    std::fs::remove_file(preprc_path).unwrap();

    // Compilation stages
    if args.lex {
        todo!()
    } else if args.parse {
        todo!()
    } else if args.codegen {
        todo!()
    }

    // Run assembler & linker
    let asm_path = path.with_extension("s");
    let bin_path = path.with_extension("");
    Command::new("gcc")
        .args([asm_path.to_str().unwrap(), "-o", bin_path.to_str().unwrap()])
        .output()
        .unwrap();

    // Assembly file cleanup
    match std::fs::remove_file(&asm_path) {
        Ok(_) => (),
        Err(err) => println!(
            "Could not remove file {}: {err}",
            asm_path.to_str().unwrap()
        ),
    }

    Ok(())
}
