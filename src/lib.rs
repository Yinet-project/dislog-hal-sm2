mod scalar;
pub use scalar::ScalarInner;

mod point;
pub use point::PointInner;

#[derive(Debug)]
pub enum EccError {
    ParseError,
}
