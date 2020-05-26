use crate::{EccError, PointInner};
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Debug;
use dislog_hal::{Bytes, DisLogPoint, ScalarNumber};
use hex::{FromHex, ToHex};
use num_bigint::BigUint;
use num_traits::identities::One;
use num_traits::identities::Zero;
use rand::RngCore;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct ScalarInner {
    pub(crate) data: BigUint,
}

impl ScalarInner {
    pub(crate) fn mod_order(mut self) -> Self {
        self.data %= Self::order().data;
        if self.data < BigUint::zero() {
            self.data += Self::order().data;
        }
        self
    }
}

impl Bytes for ScalarInner {
    type BytesType = [u8; 32];
    type Error = EccError;
    fn from_bytes(bytes: &[u8]) -> Result<Self, EccError> {
        // 实际上sm2的实现支持其他长度，此处为了对外统一
        assert!(bytes.len() == 32 || bytes.len() == 64);
        let ret = BigUint::from_bytes_le(&bytes[..]);
        Ok(Self { data: ret }.mod_order())
    }

    fn to_bytes(&self) -> Self::BytesType {
        let mut ret = [0u8; 32];
        let output = self.data.to_bytes_le();
        ret[..output.len()].clone_from_slice(&output[..]);
        ret
    }
}

impl Clone for ScalarInner {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl PartialEq for ScalarInner {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl Debug for ScalarInner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(
            f,
            "Scalar{{\n\tbytes: {:?},\n\torder: {:?},\n}}",
            &self.data.to_bytes_le(),
            &PointInner::order().data.to_bytes_le()
        )
    }
}

impl ScalarNumber for ScalarInner {
    type Point = PointInner;

    fn random<R: RngCore>(rng: &mut R) -> Self {
        let mut input = [0u8; 32];
        rng.fill_bytes(&mut input);

        loop {
            if let Ok(ret) = Self::from_bytes(&input) {
                if ret != Self::zero() {
                    return ret;
                }
            }
        }
    }

    fn order() -> Self {
        PointInner::order()
    }

    fn zero() -> Self {
        Self {
            data: BigUint::zero(),
        }
    }

    fn one() -> Self {
        Self {
            data: BigUint::one(),
        }
    }

    fn add(&self, rhs: &ScalarInner) -> ScalarInner {
        Self {
            data: &self.data + &rhs.data,
        }
        .mod_order()
    }

    fn mul(&self, rhs: &Self) -> Self {
        Self {
            data: &self.data * &rhs.data,
        }
        .mod_order()
    }

    fn inv(&self) -> Self {
        Self {
            data: PointInner::inv_order(&self.data),
        }
        .mod_order()
    }

    fn neg(&self) -> Self {
        Self {
            data: Self::order().data - &self.data,
        }
        .mod_order()
    }
}

impl Serialize for ScalarInner {
    fn serialize<SE>(&self, serializer: SE) -> Result<SE::Ok, SE::Error>
    where
        SE: Serializer,
    {
        serializer.serialize_str(&self.to_bytes().encode_hex_upper::<String>())
    }
}

impl<'de> Deserialize<'de> for ScalarInner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let d_str = String::deserialize(deserializer)
            .map_err(|_| serde::de::Error::custom(format_args!("invalid hex string")))?;
        let d_byte = Vec::<u8>::from_hex(d_str)
            .map_err(|_| serde::de::Error::custom(format_args!("invalid hex string")))?;
        ScalarInner::from_bytes(d_byte.as_slice())
            .map_err(|_| serde::de::Error::custom(format_args!("invalid hex string")))
    }
}
