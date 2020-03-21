use std::cmp::Ordering;
use std::convert::TryInto;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Sub};

#[derive(Clone, PartialEq, Copy)]
pub enum Value {
    IntegerValue(i32),
    FloatValue(f32),
    BooleanValue(bool),
    Unit,
}

impl Value {
    pub fn expect_bool(self) -> bool {
        match self {
            Value::BooleanValue(val) => val,
            _ => unreachable!("Runtime error: expected boolean value")
        }
    }
    pub fn expect_int(self) -> i32 {
        match self {
            Value::IntegerValue(val) => val,
            _ => unreachable!("Runtime error: expected int value")
        }
    }
    pub fn expect_float(self) -> f32 {
        match self {
            Value::FloatValue(val) => val,
            _ => unreachable!("Runtime error: expected float value")
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Value::IntegerValue(_) => write!(f, "integer"),
            Value::FloatValue(_) => write!(f, "float"),
            Value::BooleanValue(_) => write!(f, "boolean"),
            Value::Unit => write!(f, "unit")
        }
    }
}

impl Add for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::IntegerValue(this) => Value::IntegerValue(this + rhs.expect_int()),
            Value::FloatValue(this) => Value::FloatValue(this + rhs.expect_float()),
            _ => unreachable!("Error while executing: {}, {} in an addition block", self, rhs)
        }
    }
}

impl Sub for Value {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Value::IntegerValue(this) => Value::IntegerValue(this - rhs.expect_int()),
            Value::FloatValue(this) => Value::FloatValue(this - rhs.expect_float()),
            _ => unreachable!("Error while executing: {}, {} in a subtraction block", self, rhs)
        }
    }
}

impl Mul for Value {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Value::IntegerValue(this) => Value::IntegerValue(this * rhs.expect_int()),
            Value::FloatValue(this) => Value::FloatValue(this * rhs.expect_float()),
            _ => unreachable!("Error while executing: {}, {} in a multiplication block", self, rhs)
        }
    }
}

impl Div for Value {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Value::IntegerValue(this) => Value::IntegerValue(this / rhs.expect_int()),
            Value::FloatValue(this) => Value::FloatValue(this / rhs.expect_float()),
            _ => unreachable!("Error while executing: {}, {} in a division block", self, rhs)
        }
    }
}

impl BitXor for Value {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        match self {
            Value::IntegerValue(this) => Value::IntegerValue(this.pow(rhs.expect_int().try_into().unwrap())),
            Value::FloatValue(this) => Value::FloatValue(this.powf(rhs.expect_float())),
            Value::BooleanValue(this) => Value::BooleanValue(this ^ rhs.expect_bool()),
            _ => unreachable!("Error while executing: {}, {} in a pow/xor block", self, rhs)
        }
    }
}

impl BitAnd for Value {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        match self {
            Value::IntegerValue(this) => Value::IntegerValue(this & rhs.expect_int()),
            Value::BooleanValue(this) => Value::BooleanValue(this && rhs.expect_bool()),
            _ => unreachable!("Error while executing: {}, {} in a and block", self, rhs)
        }
    }
}

impl BitOr for Value {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        match self {
            Value::IntegerValue(this) => Value::IntegerValue(this | rhs.expect_int()),
            Value::BooleanValue(this) => Value::BooleanValue(this || rhs.expect_bool()),
            _ => unreachable!("Error while executing: {}, {} in a or block", self, rhs)
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Value::IntegerValue(this) => this.partial_cmp(&other.expect_int()),
            Value::FloatValue(this) => this.partial_cmp(&other.expect_float()),
            Value::BooleanValue(this) => this.partial_cmp(&other.expect_bool()),
            _ => unreachable!("Error while executing: {}, {} in a eq block", self, other)
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match self {
            Value::IntegerValue(this) => this.lt(&other.expect_int()),
            Value::FloatValue(this) => this.lt(&&other.expect_float()),
            Value::BooleanValue(this) => this.lt(&other.expect_bool()),
            _ => unreachable!("Error while executing: {}, {} in a eq block", self, other)
        }
    }

    fn le(&self, other: &Self) -> bool {
        match self {
            Value::IntegerValue(this) => this.le(&other.expect_int()),
            Value::FloatValue(this) => this.le(&&other.expect_float()),
            Value::BooleanValue(this) => this.le(&other.expect_bool()),
            _ => unreachable!("Error while executing: {}, {} in a eq block", self, other)
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match self {
            Value::IntegerValue(this) => this.gt(&other.expect_int()),
            Value::FloatValue(this) => this.gt(&&other.expect_float()),
            Value::BooleanValue(this) => this.gt(&other.expect_bool()),
            _ => unreachable!("Error while executing: {}, {} in a eq block", self, other)
        }
    }

    fn ge(&self, other: &Self) -> bool {
        match self {
            Value::IntegerValue(this) => this.ge(&other.expect_int()),
            Value::FloatValue(this) => this.ge(&&other.expect_float()),
            Value::BooleanValue(this) => this.ge(&other.expect_bool()),
            _ => unreachable!("Error while executing: {}, {} in a eq block", self, other)
        }
    }
}
