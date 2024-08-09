//! Contains functionality involved with source tracking and string interning.
//!
//! See https://doc.rust-lang.org/beta/nightly-rustc/rustc_span/index.html

use std::{
    cell::RefCell,
    fmt::Debug,
};
mod symbol;

thread_local! {
    static SESSION_GLOBALS: RefCell<SessionGlobals> = RefCell::new(SessionGlobals::new());
}

pub fn with_session_globals<R, F>(f: F) -> R
where
    F: FnOnce(&mut SessionGlobals) -> R,
{
    SESSION_GLOBALS.with_borrow_mut(f)
}

/// Convenient access to the global interner
pub struct SessionGlobals {
    pub interner: symbol::Interner,
    // TODO: Look at what SourceMaps are in `rustc`
    // See https://doc.rust-lang.org/beta/nightly-rustc/rustc_span/source_map/struct.SourceMap.html
}

impl SessionGlobals {
    pub fn new() -> Self {
        Self {
            interner: symbol::Interner::fresh(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpanData {
    pub hi: Offset,
    pub lo: Offset,
}

pub const DUMMY_SPAN: Span = Span { offs: 0, len: 0 };

/// Struct representing location of a token inside of a source file.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Span {
    offs: u32,
    len: u16,
}

impl Default for Span {
    fn default() -> Self {
        DUMMY_SPAN
    }
}

impl Span {
    pub fn new(mut lo: Offset, mut hi: Offset) -> Self {
        if lo > hi {
            std::mem::swap(&mut lo, &mut hi);
        }

        Span {
            offs: lo.0,
            len: (hi.0 - lo.0) as u16,
        }
    }

    pub fn data(&self) -> SpanData {
        let len = self.len as u32;
        SpanData {
            lo: Offset(self.offs),
            hi: Offset(self.offs + len),
        }
    }
}

/// Newtype representing the start of a span from a given source.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Offset(pub u32);

/// Newtype index for interned strings.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymbolIdx {
    priv_idx: u32,
}

impl SymbolIdx {
    pub const fn from_u32(idx: u32) -> Self {
        Self { priv_idx: idx }
    }
}

impl Debug for SymbolIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Idx {}", self.priv_idx))
    }
}
/// Interned string.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Symbol(SymbolIdx);

impl Symbol {
    pub const fn new(idx: u32) -> Self {
        Symbol(SymbolIdx::from_u32(idx))
    }

    pub fn intern(string: &str) -> Self {
        with_session_globals(|sess| sess.interner.intern(string))
    }
}
/// Interned identifier with location.
pub struct Ident {
    name: Symbol,
    span: Span,
}
