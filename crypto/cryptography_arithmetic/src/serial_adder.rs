use crate::common::{Bit, BitVec, full_adder};

fn serial_adder(op1: BitVec, op2: BitVec) -> BitVec {
    let max_digits = op1.len().max(op2.len());

    let mut sum: BitVec = Vec::with_capacity(max_digits + 1); // +1 to account for possible final carry
    let mut carry = Bit::Zero;

    for i in 0..max_digits {
        let x_i = if i < op1.len() { op1[i] } else { Bit::Zero };
        let y_i = if i < op2.len() { op2[i] } else { Bit::Zero };

        let (s_i, carry_out) = full_adder(x_i, y_i, carry);
        carry = carry_out;

        sum.push(s_i);
    }

    if carry == Bit::One {
        sum.push(carry);
    }

    sum
}
