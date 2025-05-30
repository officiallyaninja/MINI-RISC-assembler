mod instruction;
mod parse_file;
mod types;
use std::{
    collections::HashMap,
    env::{self, args},
    fs,
};

use instruction::Op;
use parse_file::Line;
use types::{Address, Reg};

fn main() {
    let mut args = env::args();
    _ = args.next();
    let file_name = args.next().expect("CLI ERR: no argument given for file");
    let name = args.next().unwrap_or(file_name.clone());

    let instructions: Vec<_> = fs::read_to_string(file_name)
        .expect("CLI ERR: could not open file")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| Line::from_str(line).to_instruction())
        .collect();

    print!("{}", to_verilog(&name, instructions))
}

pub fn to_verilog(name: &str, instructions: Vec<Op>) -> String {
    if name.split_whitespace().count() != 1 {
        panic!("invalid name: {name:?}");
    }
    let mut labels: HashMap<String, Address> = HashMap::new();
    let mut i = 0;
    let mut final_instructions = Vec::new();
    for instruction in instructions
        .into_iter()
        .flat_map(|op: Op| op.unpack())
        .collect::<Vec<Op>>()
    {
        if let Op::Label(label) = instruction {
            labels.insert(label, Address(i));
        } else {
            final_instructions.push(instruction);
            i += 1;
        }
    }
    let mut result: Vec<String> = vec![
        format!("task {name};"),
        "begin".to_string(),
        format!("$display(\"{name}\");"),
    ];
    for (i, instruction) in final_instructions.iter().enumerate() {
        result.push(format!(
            "  instruction_mem[{i}] = {}",
            instruction.to_verilog(&labels)
        ));
    }
    result.extend(vec!["end".into(), "endtask".into()]);

    result.join("\n")
}
