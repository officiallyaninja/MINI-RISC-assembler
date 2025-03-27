use crate::{
    instruction::Op,
    types::{BitPos, Reg},
};

pub enum Line {
    Move { source: Data, destination: Data },
    MathOneArg(OneArgOp, Reg, Reg),
    MathTwoArg(TwoArgOp, Reg, Reg, Reg),
    MathNoOutTwoArg(NoOutTwoArgOp, Reg, Reg), // DIV, MUL, CMP
    BitOp(BitOp, Bit),
    Movb(BitPos),
    JumpIf(BitPos), // Flag Bit Pos
    LoadBranch(String),
    Label(String),
    Halt,
}

pub enum OneArgOp {
    Not,
    Inc,
    Rl,
    Rr,
}

pub enum TwoArgOp {
    Add,
    Sub,
    And,
    Or,
    Xor,
}

pub enum NoOutTwoArgOp {
    Mul,
    Div,
    Cmp,
}

pub enum BitOp {
    Set,
    Clear,
    Complement,
}

pub enum Bit {
    Reg(Reg, BitPos),
    Flag(BitPos),
}

pub enum Data {
    Immediate(u16),
    Direct(Reg),
    Indirect(Reg),
    In,
    Out,
}

impl Line {
    pub fn from_str(line: &str) -> Self {
        let line = line.trim().to_lowercase();
        let (op, args): (String, Vec<&str>) = {
            match line.split_once(' ') {
                Some((first, rest)) => (
                    first.into(),
                    rest.split(',').map(|arg| arg.trim()).collect(),
                ),
                None => (line.clone(), vec![]),
            }
        };

        if args.is_empty() {
            if op == "halt" {
                return Self::Halt;
            }
            if let Some((op, _)) = op.split_once(':') {
                return Self::Label(op.into());
            } else {
                panic!("ERR: [{line}], idk what this is but it's wrong")
            }
        }

        match op.as_ref() {
            "mov" => {
                if args.len() != 2 {
                    panic!("incorrect argument length for {}", op.to_uppercase());
                }
                let destination = {
                    let arg = args[0];
                    if arg.starts_with('@') {
                        let (_, arg) = arg.split_once('@').unwrap();
                        Data::Indirect(Reg::from_str(arg))
                    } else if arg.starts_with('#') {
                        let (_, arg) = arg.split_once('#').unwrap();
                        Data::Immediate(arg.parse().expect("immediate data cannot be parsed"))
                    } else if arg == "in" {
                        Data::In
                    } else if arg == "out" {
                        Data::Out
                    } else {
                        Data::Direct(Reg::from_str(arg))
                    }
                };

                let source = {
                    let arg = args[1];
                    if arg.starts_with('@') {
                        let (_, arg) = arg.split_once('@').unwrap();
                        Data::Indirect(Reg::from_str(arg))
                    } else if arg.starts_with('#') {
                        let (_, arg) = arg.split_once('#').unwrap();
                        Data::Immediate(arg.parse().expect("immediate data cannot be parsed"))
                    } else if arg == "in" {
                        Data::In
                    } else if arg == "out" {
                        Data::Out
                    } else {
                        Data::Direct(Reg::from_str(arg))
                    }
                };

                Self::Move {
                    destination,
                    source,
                }
            }
            "not" | "inc" | "rl" | "rr" => {
                if args.len() != 2 {
                    panic!("incorrect argument length for {}", op.to_uppercase());
                }
                let op = match op.as_ref() {
                    "not" => OneArgOp::Not,
                    "inc" => OneArgOp::Inc,
                    "rl" => OneArgOp::Rl,
                    "rr" => OneArgOp::Rr,
                    _ => unreachable!(),
                };
                Self::MathOneArg(op, Reg::from_str(args[0]), Reg::from_str(args[1]))
            }
            "add" | "sub" | "and" | "or" | "xor" => {
                if args.len() != 3 {
                    panic!("incorrect argument length for {}", op.to_uppercase());
                }
                let op = match op.as_ref() {
                    "add" => TwoArgOp::Add,
                    "sub" => TwoArgOp::Sub,
                    "and" => TwoArgOp::And,
                    "or" => TwoArgOp::Or,
                    "xor" => TwoArgOp::Xor,
                    _ => unreachable!(),
                };
                Self::MathTwoArg(
                    op,
                    Reg::from_str(args[0]),
                    Reg::from_str(args[1]),
                    Reg::from_str(args[2]),
                )
            }
            "mul" | "div" | "cmp" => {
                if args.len() != 2 {
                    panic!("incorrect argument length for {}", op.to_uppercase());
                }
                let op = match op.as_ref() {
                    "mul" => NoOutTwoArgOp::Mul,
                    "div" => NoOutTwoArgOp::Div,
                    "cmp" => NoOutTwoArgOp::Cmp,
                    _ => unreachable!(),
                };
                Self::MathNoOutTwoArg(op, Reg::from_str(args[0]), Reg::from_str(args[1]))
            }
            "set" | "clr" | "cpl" => {
                if (args.len() != 1) {
                    panic!("incorrect argument length for {}", op.to_uppercase());
                }
                let bit_op = match op.as_ref() {
                    "set" => BitOp::Set,
                    "clr" => BitOp::Clear,
                    "cpl" => BitOp::Complement,
                    _ => unreachable!(),
                };
                match args[0].split_once('.') {
                    Some((reg, bit_pos)) => Self::BitOp(
                        bit_op,
                        Bit::Reg(
                            Reg::from_str(reg),
                            BitPos(bit_pos.parse().expect("invalid bit index")),
                        ),
                    ),
                    None => Self::BitOp(bit_op, Bit::Flag(BitPos::from_flag(args[0]))),
                }
            }
            "movb" => {
                if args.len() != 1 {
                    panic!("incorrect argument length for {}", op.to_uppercase());
                }

                let index = args[0]
                    .split_once("IN.")
                    .expect("must specify movb from IN reg")
                    .1
                    .parse()
                    .expect("cannot parse bit index");
                Self::Movb(BitPos(index))
            }
            "jf" => {
                if args.len() != 1 {
                    panic!("incorrect argument length for {}", op.to_uppercase());
                }
                Self::JumpIf(BitPos::from_flag(&args[0]))
            }
            "loadbr" => {
                if args.len() != 1 {
                    panic!("incorrect argument length for {}", op.to_uppercase());
                }
                Self::LoadBranch(args[0].into())
            }
            _ => {
                panic!("ERR: [{line}], invalid operator")
            }
        }
    }
    pub fn to_instruction(self) -> Op {
        use Op::*;
        match self {
            Line::Move {
                destination,
                source,
            } => match (destination, source) {
                (Data::Direct(dest), Data::Direct(src)) => MOV(dest, src),

                (Data::Direct(dest), Data::Indirect(src)) => LOAD(dest, src),
                (Data::Direct(reg), Data::Immediate(data)) => {
                    LoadByte(reg, data.to_be_bytes()[0], data.to_be_bytes()[1])
                }
                (Data::Direct(dest), Data::In) => MOVIN(dest),

                (Data::Indirect(dest), Data::Direct(src)) => STORE(dest, src),
                (Data::Out, Data::Direct(src)) => MOVOUT(src),

                (Data::Immediate(_), _) => {
                    panic!("ERR: you cannot have immediate data as destination")
                }
                (Data::Indirect(reg), _) => {
                    panic!("ERR: must use indirect access with direct access")
                }
                (_, Data::Indirect(reg)) => {
                    panic!("ERR: must use indirect access with direct access")
                }
                (Data::In, _) => panic!("ERR: Input register can never be destination"),
                (_, Data::In) => {
                    panic!("ERR: Input register data can only be read to general purpose register")
                }
                (_, Data::Out) => panic!("ERR: Output register can never be source"),
                (Data::Out, _) => panic!(
                    "ERR: Output register can only be written to from general purpose register"
                ),
            },
            Line::MathOneArg(op, reg_out, reg_in) => match op {
                OneArgOp::Not => NOT(reg_out, reg_in),
                OneArgOp::Inc => INC(reg_out, reg_in),
                OneArgOp::Rl => RL(reg_out, reg_in),
                OneArgOp::Rr => RR(reg_out, reg_in),
            },

            Line::MathTwoArg(op, reg_out, reg1, reg2) => match op {
                TwoArgOp::Add => ADD(reg_out, reg1, reg2),
                TwoArgOp::Sub => SUB(reg_out, reg1, reg2),
                TwoArgOp::And => AND(reg_out, reg1, reg2),
                TwoArgOp::Or => OR(reg_out, reg1, reg2),
                TwoArgOp::Xor => XOR(reg_out, reg1, reg2),
            },

            Line::MathNoOutTwoArg(op, reg1, reg2) => match op {
                NoOutTwoArgOp::Mul => MUL(reg1, reg2),
                NoOutTwoArgOp::Div => DIV(reg1, reg2),
                NoOutTwoArgOp::Cmp => CMP(reg1, reg2),
            },
            Line::BitOp(op, bit) => match op {
                BitOp::Set => match bit {
                    Bit::Reg(reg, bit_pos) => SETB(reg, bit_pos),
                    Bit::Flag(bit_pos) => SETF(bit_pos),
                },
                BitOp::Clear => match bit {
                    Bit::Reg(reg, bit_pos) => CLRB(reg, bit_pos),
                    Bit::Flag(bit_pos) => CLRF(bit_pos),
                },
                BitOp::Complement => match bit {
                    Bit::Reg(reg, bit_pos) => CPLB(reg, bit_pos),
                    Bit::Flag(bit_pos) => CPLF(bit_pos),
                },
            },
            Line::Movb(bit_pos) => MOVB(bit_pos),
            Line::JumpIf(bit_pos) => JF(bit_pos),
            Line::Label(name) => Label(name),
            Line::LoadBranch(name) => LOADBR(name),
            Line::Halt => HALT,
        }
    }
}
