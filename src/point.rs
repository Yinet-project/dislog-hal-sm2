use crate::{EccError, NewU833, ScalarInner};
use core::fmt::Debug;
use dislog_hal::DisLogPoint;
use dislog_hal::{Bytes, Scalar};
use hex::{FromHex, ToHex};
use lazy_static::*;
use num_bigint::BigUint;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::string::String;

pub struct PointInner {
    pub(crate) data: libsm::sm2::ecc::Point,
}

lazy_static! {
    pub static ref ECC_CTX: libsm::sm2::ecc::EccCtx = libsm::sm2::ecc::EccCtx::new();
    pub static ref ECC_ZERO_DESC: NewU833 = NewU833([
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
        write!(f, "Point{{\n\tbytes: {:?},\n}}", &self.to_bytes())
    }
}

impl Bytes for PointInner {
    type BytesType = NewU833;
    type Error = EccError;
    fn from_bytes(bytes: &[u8]) -> Result<Self, EccError> {
        assert_eq!(bytes.len(), 33);
        if ECC_ZERO_DESC.0[..] == bytes[..] {
            return Ok(Self::zero());
        }
        match ECC_CTX.bytes_to_point(bytes) {
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

    fn get_x(&self) -> Scalar<Self::Scalar> {
        let (x_1, _) = ECC_CTX.to_affine(&self.data);
        let byte = x_1.to_biguint().to_bytes_le();

        let mut num = [0u8; 32];
        num[..byte.len()].clone_from_slice(&byte[..]);

        Scalar(ScalarInner::from_bytes(&num).unwrap())
    }

    fn get_y(&self) -> Scalar<Self::Scalar> {
        let (_, y_1) = ECC_CTX.to_affine(&self.data);
        let byte = y_1.to_biguint().to_bytes_le();

        let mut num = [0u8; 32];
        num[..byte.len()].clone_from_slice(&byte[..]);

        Scalar(ScalarInner::from_bytes(&num).unwrap())
    }
}

impl Serialize for PointInner {
    fn serialize<SE>(&self, serializer: SE) -> Result<SE::Ok, SE::Error>
    where
        SE: Serializer,
    {
        serializer.serialize_str(&self.to_bytes().encode_hex_upper::<String>())
    }
}

impl<'de> Deserialize<'de> for PointInner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let d_str = String::deserialize(deserializer)
            .map_err(|_| serde::de::Error::custom(format_args!("invalid hex string")))?;
        let d_byte = Vec::<u8>::from_hex(d_str)
            .map_err(|_| serde::de::Error::custom(format_args!("invalid hex string")))?;
        PointInner::from_bytes(d_byte.as_slice())
            .map_err(|_| serde::de::Error::custom(format_args!("invalid hex string")))
    }
}
