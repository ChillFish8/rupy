#![allow(unused)]


use wasmer::{Store, Module, Instance, Value, imports, JIT};
use wasmer_compiler_cranelift::Cranelift;

use std::fs::read_to_string;

pub fn create_store() -> Store {
    let compiler = Cranelift::new();
    Store::new(&JIT::new(compiler).engine())
}

pub fn run_wat(code: &str) -> anyhow::Result<()> {
    let store = create_store();
    let module = Module::new(&store, &module_wat)?;

    let import_object = imports! {};
    let _ = Instance::new(&module, &import_object)?;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let module_wat = read_to_string("../wasm/tests/refs.wat")?;

        run_wat(&module_wat)?;

        Ok(())
    }
}
