use anyhow::Result;

use crate::traits::WrappingType;


pub fn get_local_define(assignees: &Vec<String>) -> String {
    let define_locals: String = assignees.join(", ");
    format!("local {}", define_locals)
}


pub fn assign(assignees: Vec<String>, value: impl WrappingType, global: bool) -> Result<String> {
    let mut out = Vec::with_capacity(assignees.len() + 1);

    if !global {
        let define_local = get_local_define(&assignees);
        out.push(define_local);
    }

    for assign in assignees {
        out.push(format!("{} = {}", assign, value.to_lua()?))
    }

    Ok(out.join("\n"))
}


#[cfg(test)]
mod wrapper_tests {
    use super::*;

    #[test]
    fn test_get_local_define() {
        let sample_input = vec!["x".to_string(), "y".to_string()];

        let out = get_local_define(&sample_input);

        assert_eq!("local x, y", out.as_str())
    }

    #[test]
    fn test_assign() -> Result<()> {
        let sample_input = vec!["x".to_string(), "y".to_string()];
        let sample_value = "abc";

        let out = assign(sample_input, sample_value)?;

        assert_eq!("local x, y\nx = \"abc\"\ny = \"abc\"", out.as_str());

        Ok(())
    }

}