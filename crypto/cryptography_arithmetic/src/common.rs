use std::ops::{BitAnd, BitOr, BitXor};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bit {
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
pub type BitVec = Vec<Bit>;

/// Computes the sum and carry-out of two single-bit inputs and a carry-in.
///
/// Implements a standard full adder circuit:
/// - `s_i = x_i ⊕ y_i ⊕ carry_in`
/// - `carry_out = (x_i ∧ y_i) ∨ ((x_i ⊕ y_i) ∧ carry_in)`
///
/// # Arguments
/// * `x_i` - First single-bit input.
/// * `y_i` - Second single-bit input.
/// * `carry_in` - Carry bit from a previous addition stage.
///
/// # Returns
/// A tuple `(s_i, carry_out)` where `s_i` is the sum bit and `carry_out`
/// is the carry bit to propagate to the next stage.
pub fn full_adder(x_i: Bit, y_i: Bit, carry_in: Bit) -> (Bit, Bit) {
    let s_i = (x_i ^ y_i) ^ carry_in;
    let carry_out = (x_i & y_i) | ((x_i ^ y_i) & carry_in);

    (s_i, carry_out)
}
