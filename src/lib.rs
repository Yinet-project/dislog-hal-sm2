mod scalar;
pub use scalar::ScalarInner;

mod point;
pub use point::PointInner;

use core::convert::AsRef;
use core::fmt::Debug;
#[derive(Debug)]
pub enum EccError {
    ParseError,
}

pub struct NewU833(pub [u8; 33]);

impl Debug for NewU833 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "[{:?}]", &self)
    }
}

impl PartialEq for NewU833 {
    fn ne(&self, other: &Self) -> bool {
        self.0[..] != other.0[..]
    }

    fn eq(&self, other: &Self) -> bool {
        self.0[..] == other.0[..]
    }
}

impl Clone for NewU833 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl AsRef<[u8; 33]> for NewU833 {
    fn as_ref(&self) -> &[u8; 33] {
        &self.0
    }
}
