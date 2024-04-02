use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct Instruction: u16 {
        const REF_STAR_A = 1 << 12;
        const ALU_U = 1 << 10;
        const ALU_OP1 = 1 << 9;
        const ALU_OP0 = 1 << 8;
        const ALU_ZX = 1 << 7;
        const ALU_SW = 1 << 6;
        const COND_LT = 1 << 2;
        const COND_EQ = 1 << 1;
        const COND_GT = 1 << 0;
    }
}

#[derive(Debug)]
pub struct Input {
    pub i: Instruction,
    pub a: u16,
    pub d: u16,
    pub star_a: u16,
}

#[derive(Debug)]
pub struct Output {
    pub r: u16,
    pub jump: bool,
}

pub fn alu(input: &Input) -> Output {
    let x = input.d;

    let y = if input.i.contains(Instruction::REF_STAR_A) {
        input.star_a
    } else {
        input.a
    };

    let (x, y) = if input.i.contains(Instruction::ALU_SW) {
        (y, x)
    } else {
        (x, y)
    };

    let x = if input.i.contains(Instruction::ALU_ZX) {
        0
    } else {
        x
    };

    let op0 = input.i.contains(Instruction::ALU_OP0);
    let op1 = input.i.contains(Instruction::ALU_OP1);

    let r: u16 = if input.i.contains(Instruction::ALU_U) {
        // Arithmetic Unit
        match (op1, op0) {
            (false, false) => x.wrapping_add(y),
            (true, false) => x.wrapping_sub(y),
            (false, true) => x.wrapping_add(1),
            (true, true) => x.wrapping_sub(1),
        }
    } else {
        // Logic Unit
        match (op1, op0) {
            (false, false) => x & y,
            (true, false) => x | y,
            (false, true) => x ^ y,
            (true, true) => !x,
        }
    };

    // Condition
    let eq = r == 0;
    let lt = r | 0x8000 != 0;
    let gt = !lt && !eq;

    let lt = input.i.contains(Instruction::COND_LT) && lt;
    let eq = input.i.contains(Instruction::COND_EQ) && eq;
    let gt = input.i.contains(Instruction::COND_GT) && gt;

    let jump = lt || eq || gt;

    Output { r, jump }
}
