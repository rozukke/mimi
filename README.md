# mimi

[![GitHub license](https://img.shields.io/github/license/rozukke/mimi.svg)](https://github.com/rozukke/mimi/blob/main/LICENSE)
[![Creator rozukke](https://img.shields.io/badge/Creator-rozukke-f497af.svg)](https://github.com/rozukke)
[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-b7410e.svg)](https://www.rust-lang.org)

`mimi` is a simple C compiler built with Rust following Nora Sandler's "Writing a C Compiler - Build a Real Programming Language from Scratch (2024)".

## Usage
Use the compiler with the command `mimi <file.c> [OPTIONS]`. There are a few available options, listed below.

**General options:**
- `--help` or `-h`: View the help menu.
- `--version` or `-v`: Print the current version of `mimi`.

**Options for testing:**
- `--lex` or `-l`: Only run the compiler up to the lexer stage.
- `--parse` or `-p`: Only run the compiler up to the parser stage.
- `--codegen` or `-c`: Only run the compiler up to the codegen stage.
