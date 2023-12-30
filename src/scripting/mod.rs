use std::fs;
use mlua::prelude::*;

pub fn run_engine() -> LuaResult<()> {
    let lua = Lua::new();
    let script: String = fs::read_to_string("test-project/main.lua")?.parse().unwrap();

    let _ = lua.load(script.as_str()).exec()?;
    Ok(())
}

