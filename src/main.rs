mod assembler;
mod wrappers;
mod traits;

use std::fs::read_to_string;
use std::ffi::CStr;

use anyhow::Result;

use rupy_parser::parser::parse_program;

use crate::assembler::build_program;


fn main() -> Result<()> {
    let test = read_to_string("./test.py")?;
    let program = parse_program(&test)?;
    let lua_code = build_program(program)?;
    println!("{}", &lua_code);

    unsafe {
        let lua = luajit2_sys::luaL_newstate();
        luajit2_sys::luaL_openlibs(lua);
        let script_data = lua_code;
        let script_name = b"run_script\0";
        luajit2_sys::luaL_loadbuffer(
            lua,
            script_data.as_ptr() as _,
            script_data.len() as _,
            script_name.as_ptr() as _,
        );
        luajit2_sys::lua_pcall(lua, 0, 1, 0);
        let idx = luajit2_sys::lua_gettop(lua);
        let s = luajit2_sys::lua_tostring(lua, idx);
        let result = CStr::from_ptr(s).to_string_lossy().to_string();
        luajit2_sys::lua_close(lua);

        println!("result: {}", result);
    }


    Ok(())
}
