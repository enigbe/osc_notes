use crate::common::{Bit, BitVec, full_adder};

fn carry_ripple_adder(op1: BitVec, op2: BitVec, word_size: usize) -> BitVec {
    let mut sum: BitVec = Vec::with_capacity(word_size + 1);
    let mut carry = Bit::Zero;

    for i in 0..word_size {
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
