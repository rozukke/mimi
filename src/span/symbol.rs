use super::Symbol;

use bumpalo::Bump;
use indexmap::{set::Iter, IndexSet};
use rustc_hash::FxHasher;
use std::hash::BuildHasherDefault;

pub type FxIndexSet<V> = IndexSet<V, BuildHasherDefault<FxHasher>>;

pub(crate) struct Interner {
    // TODO: Become more familiar with how this works on a technical level.
    // See https://doc.rust-lang.org/beta/nightly-rustc/rustc_arena/struct.DroplessArena.html
    arena: Bump,
    strings: FxIndexSet<&'static str>,
}

impl Interner {
    /// Intern keywords
    fn prefill(init: &[&'static str]) -> Self {
        Interner {
            arena: Default::default(),
            strings: init.iter().copied().collect(),
        }
    }

    #[inline]
    pub fn intern(&mut self, string: &str) -> Symbol {
        // String already interned
        if let Some(idx) = self.strings.get_index_of(string) {
            return Symbol::new(idx as u32);
        }

        let string: &str = self.arena.alloc_str(string);
        let string: &'static str = unsafe { &*(string as *const str) };

        let (idx, is_new) = self.strings.insert_full(string);
        debug_assert!(is_new);

        Symbol::new(idx as u32)
    }

    pub fn fresh() -> Self {
        Self::prefill(&["int", "return"])
    }

    pub fn iter(&self) -> Iter<'_, &str> {
        self.strings.iter()
    }
}

mod kw {
    use super::Symbol;
    pub const INT: Symbol = Symbol::new(0);
    pub const RETURN: Symbol = Symbol::new(1);
}
