mod wrappers;

use std::fs::read_to_string;
use std::error::Error;

use rupy_compiler::compile::{compile_program, CompileOpts};
use rupy_bytecode::{Instruction, ConstantData};
use rupy_parser::parser::parse_program;





fn main() -> Result<(), Box<dyn Error>>{
    let test = read_to_string("./test.py")?;
    let program = parse_program(&test)?;

    let opts = CompileOpts::default();
    let code = compile_program(
        program.as_ref(),
        "./test.py".into(),
        opts,
    )?;

    let constants = code.constants;
    println!("{:?}", constants);
    // t insts = match code {
    //   ConstantData::Code { code } => {
    //       println!("{:?}", &code.constants);
    //       &code.instructions
    //   },
    //   _ => panic!("party!")
    //

    for instruction in code.instructions.as_ref() {
        println!("{:?}", instruction);
    }

    Ok(())
}
