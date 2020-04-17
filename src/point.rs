use crate::{EccError, NewU833, ScalarInner};
use core::fmt::Debug;
use dislog_hal::Bytes;
use dislog_hal::DisLogPoint;
use lazy_static::*;
use num_bigint::BigUint;

pub struct PointInner {
    pub(crate) data: cryptape_sm::sm2::ecc::Point,
}

lazy_static! {
    static ref ECC_CTX: cryptape_sm::sm2::ecc::EccCtx = cryptape_sm::sm2::ecc::EccCtx::new();
    static ref ECC_ZERO_DESC: NewU833 = NewU833([
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ]);
}

impl PointInner {
    pub fn inv_order(x: &BigUint) -> BigUint {
        ECC_CTX.inv_n(x)
    }
}

impl Debug for PointInner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "Scalar{{\n\tbytes: {:?},\n}}", &self.to_bytes())
    }
}

impl Bytes for PointInner {
    type BytesType = NewU833;
    type Error = EccError;
    fn from_bytes(bytes: Self::BytesType) -> Result<Self, EccError> {
        if *ECC_ZERO_DESC == bytes {
            return Ok(Self::zero());
        }
        match ECC_CTX.bytes_to_point(&bytes.0) {
            Ok(x) => Ok(Self { data: x }),
            Err(_) => Err(EccError::ParseError),
        }
    }

    fn to_bytes(&self) -> Self::BytesType {
        if self == &Self::zero() {
            return ECC_ZERO_DESC.clone();
        }
        let mut ret = [0u8; 33];
        ret.clone_from_slice(&ECC_CTX.point_to_bytes(&self.data, true)[0..33]);
        NewU833(ret)
    }
}

impl Clone for PointInner {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl PartialEq for PointInner {
    fn ne(&self, other: &Self) -> bool {
        !ECC_CTX.eq(&self.data, &other.data)
    }

    fn eq(&self, other: &Self) -> bool {
        ECC_CTX.eq(&self.data, &other.data)
    }
}

impl DisLogPoint for PointInner {
    type Scalar = ScalarInner;

    fn order() -> Self::Scalar {
        Self::Scalar {
            data: ECC_CTX.get_n().clone(),
        }
    }

    fn zero() -> Self {
        Self {
            data: ECC_CTX.zero(),
        }
    }

    fn one() -> Self {
        Self {
            data: ECC_CTX.generator(),
        }
    }

    fn generator() -> Self {
        Self {
            data: ECC_CTX.generator(),
        }
    }

    fn add(&self, rhs: &Self) -> Self {
        if ECC_CTX.eq(&self.data, &rhs.data) {
            Self {
                data: ECC_CTX.double(&ECC_CTX.generator()),
            }
        } else {
            Self {
                data: ECC_CTX.add(&self.data, &rhs.data),
            }
        }
    }

    fn mul(&self, rhs: &Self::Scalar) -> Self {
        Self {
            data: ECC_CTX.mul(&rhs.data, &self.data),
        }
    }

    fn neg(&self) -> Self {
        Self {
            data: ECC_CTX.neg(&self.data),
        }
    }
}
