//! Stores a DEFINE INDEX config definition
use crate::key::category::Categorise;
use crate::key::category::Category;
use derive::Key;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize, Key)]
#[non_exhaustive]
pub struct Ix<'a> {
    __: u8,
    _a: u8,
    pub ns: &'a str,
    _b: u8,
    pub db: &'a str,
    _c: u8,
    pub tb: &'a str,
    _d: u8,
    _e: u8,
    _f: u8,
    pub ix: &'a str,
}

pub fn new<'a>(ns: &'a str, db: &'a str, tb: &'a str, ix: &'a str) -> Ix<'a> {
    Ix::new(ns, db, tb, ix)
}

pub fn prefix(ns: &str, db: &str, tb: &str) -> Vec<u8> {
    let mut k = super::all::new(ns, db, tb).encode().unwrap();
    k.extend_from_slice(b"!ix\x00");
    k
}

pub fn suffix(ns: &str, db: &str, tb: &str) -> Vec<u8> {
    let mut k = super::all::new(ns, db, tb).encode().unwrap();
    k.extend_from_slice(b"!ix\xff");
    k
}

impl Categorise for Ix<'_> {
    fn categorise(&self) -> Category {
        Category::IndexDefinition
    }
}

impl<'a> Ix<'a> {
    pub fn new(ns: &'a str, db: &'a str, tb: &'a str, ix: &'a str) -> Self {
        Self {
            __: b'/',
            _a: b'*',
            ns,
            _b: b'*',
            db,
            _c: b'*',
            tb,
            _d: b'!',
            _e: b'i',
            _f: b'x',
            ix,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn key() {
        use super::*;
        #[rustfmt::skip]
		let val = Ix::new(
			"testns",
			"testdb",
			"testtb",
			"testix",
		);
        let enc = Ix::encode(&val).unwrap();
        assert_eq!(enc, b"/*testns\0*testdb\0*testtb\0!ixtestix\0");

        let dec = Ix::decode(&enc).unwrap();
        assert_eq!(val, dec);
    }
}
