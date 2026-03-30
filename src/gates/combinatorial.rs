use super::{and, or, not, Bit};

pub fn mux(a: Bit, b: Bit, s:Bit) -> Bit {
    or(and(a, not(s)), and(b, s))
}

pub fn dmux(ain: Bit, s:Bit) -> (Bit, Bit) {
    (and(ain, not(s)), and(ain, s))
}
