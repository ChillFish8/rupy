
/// Wraps a given set of bytecode instructions in a module block.
///
/// This function automatically defines memory so should not be included
/// in the code as memory is interpreted as part of the module.
pub fn wrap_with_module(code: &[String]) -> String {
    format!(r#"
    (module
        (memory $1 1)
        {}
    )"#, code.join("\n"))
}

pub fn wrap_with_function(name: &str, code: &[String]) -> String {
    format!(r#"
    (func $module/{}
        {}
    )"#, name, code.join("\n"))
}


#[cfg(test)]
mod tests {
    use super::*;
    use rupy_runtime::run_wat;

    #[test]
    fn test_wrap_with_module() -> anyhow::Result<()> {
        let code = vec![
            "(table $0 1 funcref)".to_string(),
            "(export \"fib\" (func $module/fib))".to_string(),
            "(export \"memory\" (memory $0))".to_string(),
        ];

        let out = wrap_with_module(code.as_ref());
        run_wat(&out)?;
        Ok(())
    }

    #[test]
    fn test_wrap_with_function() -> anyhow::Result<()> {


        Ok(())
    }

    #[test]
    fn test_wrap_func_with_wrap_module() -> anyhow::Result<()> {


        Ok(())
    }
}