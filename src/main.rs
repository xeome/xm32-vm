mod assembler;
mod logger;
mod screen;
mod vm;

use log::info;

fn main() {
    logger::init_logger();

    let filename = "src/fibonacci.xasm";
    let code = std::fs::read_to_string(filename).expect("Failed to read code.txt");

    let assembler = assembler::Assembler::new();

    info!("Assembling file: {}", filename);
    let bytecode = assembler.assemble(&code);

    info!("Bytecode: {:?}", bytecode);

    let mut vm = vm::VM::new();
    let result = String::new();
    info!("Initializing VM");
    vm.init(bytecode, Some(result));
    info!("Running code");
    vm.run().expect("Failed to run code");
}
