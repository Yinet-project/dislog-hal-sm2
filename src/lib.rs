mod scalar;
pub use scalar::ScalarInner;

mod point;
pub use point::{PointInner, ECC_CTX};

use core::convert::AsRef;
use core::fmt::Debug;
use hex::{FromHex, FromHexError};
#[derive(Debug)]
pub enum EccError {
    ParseError,
}

pub struct NewU833(pub [u8; 33]);

impl FromHex for NewU833 {
    type Error = FromHexError;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        match <[u8; 33]>::from_hex(hex) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(err),
        }
    }
}

impl AsRef<[u8]> for NewU833 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Debug for NewU833 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut info = f.debug_list();
        for i in 0..self.0.len() {
            info.entry(&self.0[i]);
        }
        info.finish()
    }
}

impl PartialEq for NewU833 {
    fn eq(&self, other: &Self) -> bool {
        self.0[..] == other.0[..]
    }
}

impl Clone for NewU833 {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
