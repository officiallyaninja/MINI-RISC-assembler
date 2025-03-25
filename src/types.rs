use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub struct Reg(pub u8);
#[derive(Clone, Copy, Debug)]
pub struct RegBitPos(pub u8);
#[derive(Clone, Copy, Debug)]
pub struct FlagBitPos(pub u8);
#[derive(Clone, Copy, Debug)]
pub struct PortBitPos(pub u8);
#[derive(Clone, Copy, Debug)]
pub struct Address(pub u16);

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num = self.0;
        if num > 7 {
            panic!("invalid reg number: {num}")
        };
        write!(f, "REG{num}")
    }
}

impl Display for RegBitPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pos = self.0;
        if pos > 16 {
            panic!("invalid reg bit pos: {pos}");
        };
        write!(f, "4'd{pos}")
    }
}

impl Display for FlagBitPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pos = self.0;
        if pos > 16 {
            panic!("invalid flag bit pos: {pos}");
        };
        write!(f, "4'd{pos}")
    }
}

impl Display for PortBitPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pos = self.0;
        if pos > 16 {
            panic!("invalid port bit pos: {pos}");
        };
        write!(f, "4'd{pos}")
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let addr = self.0;
        if addr > (1 << 11) {
            panic!("invalid address: {addr}");
        };
        write!(f, "11'd{addr}")
    }
}
