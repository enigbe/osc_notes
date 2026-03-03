use std::ops::{BitAnd, BitOr, BitXor};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Bit {
    Zero = 0,
    One = 1,
}

impl BitAnd for Bit {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Bit::One, Bit::One) => Bit::One,
            _ => Bit::Zero,
        }
    }
}

impl BitOr for Bit {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Bit::Zero, Bit::Zero) => Bit::Zero,
            _ => Bit::One,
        }
    }
}

impl BitXor for Bit {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Bit::Zero, Bit::One) | (Bit::One, Bit::Zero) => Bit::One,
            _ => Bit::Zero,
        }
    }
}

/// A vector of bits (binary digits) where each element is 0 or 1,
/// stored in LSB-first order. The length represents the number of
/// digits in the number.
/// e.g.
/// 5   (101 in binary)     =   [1, 0, 1]       (LSB=1 at index 0)
/// 6   (110 in binary)     =   [0, 1, 1]       (LSB=0 at index 0)
/// 8   (1000 in binary)    =   [0, 0, 0, 1]    (LSB=0 at index 0)
/// 12  (1100 in binary)    =   [0, 0, 1, 1]    (LSB=0 at index 0)
type BitVec = Vec<Bit>;

fn serial_adder(op1: BitVec, op2: BitVec) -> BitVec {
    let max_digits = op1.len().max(op2.len());

    let mut sum: BitVec = Vec::with_capacity(max_digits + 1); // +1 to account for possible final carry
    let mut carry = Bit::Zero;

    for i in 0..max_digits {
        let x_i = if i < op1.len() { op1[i] } else { Bit::Zero };
        let y_i = if i < op2.len() { op2[i] } else { Bit::Zero };

        let s_i = (x_i ^ y_i) ^ carry;
        carry = (x_i & y_i) | ((x_i ^ y_i) & carry);

        sum.push(s_i);
    }

    if carry == Bit::One {
        sum.push(carry);
    }

    sum
}

fn main() {}
