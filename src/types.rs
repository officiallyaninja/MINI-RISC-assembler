use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub struct Reg(pub u8);
#[derive(Clone, Copy, Debug)]
pub struct BitPos(pub u8);
// #[derive(Clone, Copy, Debug)]
// pub struct RegBitPos(pub u8);
// #[derive(Clone, Copy, Debug)]
// pub struct FlagBitPos(pub u8);
// #[derive(Clone, Copy, Debug)]
// pub struct PortBitPos(pub u8);
#[derive(Clone, Copy, Debug)]
pub struct Address(pub u16);

impl Reg {
    pub fn from_str(str: &str) -> Self {
        match str.to_lowercase().as_ref() {
            "r0" => Reg(0),
            "r1" => Reg(1),
            "r2" => Reg(2),
            "r3" => Reg(3),
            "r4" => Reg(4),
            "r5" => Reg(5),
            "r6" => Reg(6),
            "r7" => Reg(7),
            _ => panic!("Invalid register string {str}"),
        }
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num = self.0;
        if num > 7 {
            panic!("invalid reg number: {num}")
        };
        write!(f, "REG{num}")
    }
}

impl Display for BitPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pos = self.0;
        if pos > 16 {
            panic!("invalid bit pos: {pos}");
        };
        write!(f, "4'd{pos}")
    }
}

impl BitPos {
    pub fn from_flag(name: &str) -> Self {
        match name.to_lowercase().as_ref() {
            "c" | "carry" => BitPos(0),
            "v" | "overflow" => BitPos(1),
            "cmp" | "compare" => BitPos(2),
            "eq" | "equal" => BitPos(3),
            "io" => BitPos(4),
            "p" | "parity" => BitPos(5),
            "n" | "negative" => BitPos(6),
            "z" | "zero" => BitPos(7),
            _ => panic!("invalid flag bit"),
        }
    }
}

// impl Display for RegBitPos {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let pos = self.0;
//         if pos > 16 {
//             panic!("invalid reg bit pos: {pos}");
//         };
//         write!(f, "4'd{pos}")
//     }
// }
//
// impl Display for FlagBitPos {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let pos = self.0;
//         if pos > 16 {
//             panic!("invalid flag bit pos: {pos}");
//         };
//         write!(f, "4'd{pos}")
//     }
// }
//
// impl Display for PortBitPos {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let pos = self.0;
//         if pos > 16 {
//             panic!("invalid port bit pos: {pos}");
//         };
//         write!(f, "4'd{pos}")
//     }
// }
//
impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let addr = self.0;
        if addr > (1 << 11) {
            panic!("invalid address: {addr}");
        };
        write!(f, "11'd{addr}")
    }
}
