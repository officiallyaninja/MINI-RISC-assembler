use std::collections::HashMap;

use crate::types::{Address, BitPos, Reg};

#[derive(Debug)]
pub enum Op {
    ADD(Reg, Reg, Reg),
    MUL(Reg, Reg),
    SUB(Reg, Reg, Reg),
    DIV(Reg, Reg),
    NOT(Reg, Reg),
    AND(Reg, Reg, Reg),
    OR(Reg, Reg, Reg),
    XOR(Reg, Reg, Reg),
    INC(Reg, Reg),
    CMP(Reg, Reg),
    RR(Reg, Reg),
    RL(Reg, Reg),
    SETB(Reg, BitPos),
    CLRB(Reg, BitPos),
    CPLB(Reg, BitPos),
    SETF(BitPos),
    CLRF(BitPos),
    CPLF(BitPos),
    LOADBR(String),
    JF(BitPos),
    LOAD(Reg, Reg),
    STORE(Reg, Reg),
    LBL(Reg, u8),
    LBH(Reg, u8),
    MOV(Reg, Reg),
    MOVOUT(Reg),
    MOVIN(Reg),
    MOVB(BitPos),
    HALT,
    // Psuedo Instructions
    Label(String),
    LoadByte(Reg, u8, u8),
}

impl Op {
    pub fn to_verilog(&self, labels: &HashMap<String, Address>) -> String {
        let opcode = self.opcode();
        let mut result = format!("{{{opcode}, ");
        result += &match self {
            Op::ADD(out, in1, in2) => format!("{out}, {in1}, {in2}, 2'bx"),
            Op::MUL(in1, in2) => format!("REG0, {in1}, {in2}, 2'bx"), // output reg is hardcoded
            Op::SUB(out, in1, in2) => format!("{out}, {in1}, {in2}, 2'bx"),
            Op::DIV(in1, in2) => format!("REG0, {in1}, {in2}, 2'bx"),
            Op::NOT(out, in1) => format!("{out}, {in1}, 5'bx"),
            Op::AND(out, in1, in2) => format!("{out}, {in1}, {in2}, 2'bx"),
            Op::OR(out, in1, in2) => format!("{out}, {in1}, {in2}, 2'bx"),
            Op::XOR(out, in1, in2) => format!("{out}, {in1}, {in2}, 2'bx"),
            Op::INC(out, in1) => format!("{out}, {in1}, 5'bx"),
            Op::CMP(in1, in2) => format!("3'bx, {in1}, {in2}, 2'bx"),
            Op::RR(out, in1) => format!("{out}, {in1}, 5'bx"),
            Op::RL(out, in1) => format!("{out}, {in1}, 5'bx"),
            Op::SETB(reg, reg_bit_pos) => format!("{reg}, {reg}, {reg_bit_pos}, 1'bx"),
            Op::CLRB(reg, reg_bit_pos) => format!("{reg}, {reg}, {reg_bit_pos}, 1'bx"),
            Op::CPLB(reg, reg_bit_pos) => format!("{reg}, {reg}, {reg_bit_pos}, 1'bx"),
            Op::SETF(flag_bit_pos) => format!("6'bx, {flag_bit_pos}, 1'bx"),
            Op::CLRF(flag_bit_pos) => format!("6'bx, {flag_bit_pos}, 1'bx"),
            Op::CPLF(flag_bit_pos) => format!("6'bx, {flag_bit_pos}, 1'bx"),
            Op::LOADBR(label) => format!("{}", labels[label]),
            Op::JF(flag_bit_pos) => format!("6'bx, {flag_bit_pos}, 1'bx"),
            Op::LOAD(dest_reg, source_addr_reg) => format!("{dest_reg}, {source_addr_reg}, 5'bx"),
            Op::STORE(dest_addr_reg, source_reg) => {
                format!("3'bx, {dest_addr_reg}, {source_reg}, 2'bx")
            }
            Op::LBL(reg, byte) => format!("{reg}, 8'd{byte}"),
            Op::LBH(reg, byte) => format!("{reg}, 8'd{byte}"),
            Op::MOV(dest, source) => format!("{dest}, {source}, 5'bx"),
            Op::MOVOUT(reg) => format!("3'bx, {reg}, 5'bx"),
            Op::MOVIN(reg) => format!("{reg}, 8'bx"),
            Op::MOVB(port_bit_pos) => format!("3'bx, {port_bit_pos}, 4'bx"),
            Op::HALT => format!("11'bx"),
            _ => panic!("have not implemented to verilog for {self:?}"),
        };
        result + "};"
    }

    fn opcode(&self) -> String {
        match self {
            Op::ADD(_, _, _) => "ADD",
            Op::MUL(_, _) => "MUL",
            Op::SUB(_, _, _) => "SUB",
            Op::DIV(_, _) => "DIV",
            Op::NOT(_, _) => "NOT",
            Op::AND(_, _, _) => "AND",
            Op::OR(_, _, _) => "OR",
            Op::XOR(_, _, _) => "XOR",
            Op::INC(_, _) => "INC",
            Op::CMP(_, _) => "CMP",
            Op::RR(_, _) => "RR",
            Op::RL(_, _) => "RL",
            Op::SETB(_, _) => "SETB",
            Op::CLRB(_, _) => "CLRB",
            Op::CPLB(_, _) => "CPLB",
            Op::SETF(_) => "SETF",
            Op::CLRF(_) => "CLRF",
            Op::CPLF(_) => "CPLF",
            Op::LOADBR(_) => "LOADBR",
            Op::JF(_) => "JF",
            Op::LOAD(_, _) => "LOAD",
            Op::STORE(_, _) => "STORE",
            Op::LBL(_, _) => "LBL",
            Op::LBH(_, _) => "LBH",
            Op::MOV(_, _) => "MOV",
            Op::MOVOUT(_) => "MOVOUT",
            Op::MOVIN(_) => "MOVIN",
            Op::MOVB(_) => "MOVB",
            Op::HALT => "HALT",
            _ => panic!("have not implemented opcode for {self:?}"),
        }
        .to_string()
    }

    pub fn unpack(self) -> Vec<Self> {
        use Op::*;
        match self {
            LoadByte(reg, upper, lower) => vec![LBH(reg, upper), LBL(reg, lower)],
            // if its already a single op
            op => vec![op],
        }
    }
}
