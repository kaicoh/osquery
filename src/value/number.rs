use core::hash::{Hash, Hasher};
use serde::ser::{Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    n: N,
}

#[derive(Debug, Copy, Clone)]
enum N {
    PosInt(u64),
    /// Always less than zero.
    NegInt(i64),
    /// Flways finite.
    Float(f64),
}

impl PartialEq for N {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (N::PosInt(a), N::PosInt(b)) => a == b,
            (N::NegInt(a), N::NegInt(b)) => a == b,
            (N::Float(a), N::Float(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for N {}

impl Hash for N {
    fn hash<H: Hasher>(&self, h: &mut H) {
        match *self {
            N::PosInt(i) => i.hash(h),
            N::NegInt(i) => i.hash(h),
            N::Float(f) => {
                if f == 0.0f64 {
                    // There are 2 zero representations, +0, and -0, which
                    // compare equal but have defferent bits. We use the +0 hash
                    // for both so that hash(+0) == hash(-0).
                    0.0f64.to_bits().hash(h)
                } else {
                    f.to_bits().hash(h)
                }
            }
        }
    }
}

impl Number {
    /// Returns true if the `Number` is an integer between `i64::MIN` and
    /// `i64::MAX`.
    ///
    /// For any Number on which `is_i64` returns true, `as_i64` is guaranteed to
    /// return the integer value.
    pub fn is_i64(&self) -> bool {
        match self.n {
            N::PosInt(v) => v <= i64::max_value() as u64,
            N::NegInt(_) => true,
            N::Float(_) => false,
        }
    }

    /// Returns true if the `Number` is an integer between zero and `u64::MAX`.
    ///
    /// For any Number on which `is_u64` return true, `as_u64` is guaranteed to
    /// return the integer value.
    pub fn is_u64(&self) -> bool {
        match self.n {
            N::PosInt(_) => true,
            N::NegInt(_) | N::Float(_) => false,
        }
    }

    /// Returns true if the `Number` can be represented by f64.
    ///
    /// For any Number on which `is_f64` returns true, `as_f64` is guaranteed to
    /// return the floating point value.
    pub fn is_f64(&self) -> bool {
        match self.n {
            N::Float(_) => true,
            N::PosInt(_) | N::NegInt(_) => false,
        }
    }

    /// If the `Number` is an integer, represent it as i64 if possible. Returns
    /// None otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        match self.n {
            N::PosInt(n) => {
                if n <= i64::max_value() as u64 {
                    Some(n as i64)
                } else {
                    None
                }
            }
            N::NegInt(n) => Some(n),
            N::Float(_) => None,
        }
    }

    /// If the `Number` is an integer, represent it as u64 if possible. Returns
    /// None otherwise.
    pub fn as_u64(&self) -> Option<u64> {
        match self.n {
            N::PosInt(n) => Some(n),
            N::NegInt(_) | N::Float(_) => None,
        }
    }

    /// Represents the number as f64 if possible. Returns None otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        match self.n {
            N::PosInt(n) => Some(n as f64),
            N::NegInt(n) => Some(n as f64),
            N::Float(n) => Some(n),
        }
    }

    /// Converts a finite `f64` to a `Number`.
    pub fn from_f64(f: f64) -> Option<Number> {
        if f.is_finite() {
            Some(Number { n: N::Float(f) })
        } else {
            None
        }
    }
}

impl Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.n {
            N::PosInt(u) => serializer.serialize_u64(u),
            N::NegInt(i) => serializer.serialize_i64(i),
            N::Float(f) => serializer.serialize_f64(f),
        }
    }
}

macro_rules! impl_from_unsigned {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for Number {
                fn from(u: $ty) -> Self {
                    let n = N::PosInt(u as u64);
                    Number { n }
                }
            }
         )*
    };
}

macro_rules! impl_from_signed {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for Number {
                fn from(i: $ty) -> Self {
                    let n = if i < 0 {
                        N::NegInt(i as i64)
                    } else {
                        N::PosInt(i as u64)
                    };
                    Number { n }
                }
            }
         )*
    };
}

impl_from_unsigned!(u8, u16, u32, u64, usize);
impl_from_signed!(i8, i16, i32, i64, isize);
