pub type Bit = bool;
pub mod combinatorial;

pub fn nand(a: Bit, b: Bit) -> Bit {
    !(a && b)
}

pub fn not(a: Bit) -> Bit {
    nand(a, a)
}

pub fn and(a: Bit, b: Bit) -> Bit {
    not(nand(a, b))
}

pub fn or(a: Bit, b: Bit) -> Bit {
    nand(not(a), not(b))
}

pub fn xor(a: Bit, b: Bit) -> Bit {
    or(and(a, not(b)), and(not(a), b))
}

