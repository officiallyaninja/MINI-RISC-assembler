pub enum Line {
    Move { source: Data, destination: Data },
    AluOneArg(),
    AluTwoArg(),
    AluSpecial(), // DIV, MUL, CMP
    BitOp(),
    JumpIf(),
    Label(),
}

pub enum Data {
    Reg(Register),
}
