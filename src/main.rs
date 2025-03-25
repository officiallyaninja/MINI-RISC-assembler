mod instruction;
mod parse_file;
mod types;
use std::collections::HashMap;

use instruction::{Address, Op, Reg};

fn main() {
    use Op::*;
    let name = "IO_test_INC";
    let instructions: Vec<Op> = vec![
        //
        MOVIN(Reg(0)),
        INC(Reg(0), Reg(0)),
        MOVOUT(Reg(0)),
        HALT,
    ];
    print!("{}", to_verilog(name, instructions))
}

pub fn to_verilog(name: &str, instructions: Vec<Op>) -> String {
    if name.split_whitespace().count() != 1 {
        panic!("invalid name: {name:?}");
    }
    let mut labels: HashMap<&str, Address> = HashMap::new();
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
