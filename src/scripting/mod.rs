use std::fs;
use mlua::prelude::*;
use crate::game_object::component::{Component, MessengerComponent};
use crate::game_object::GameObject;

pub fn run_engine() -> LuaResult<()> {
    let lua = Lua::new();
    let script: String = fs::read_to_string("test-project/main.lua")?.parse().unwrap();

    let mut test_go = GameObject::new("Test".to_string());
    let component: Component = Component::Messenger(
        MessengerComponent {
            message: "Hello, world!".to_string(),
            has_sent: false,
        }
    );

    test_go.components.push(
        component
    );

    let go_table =  lua.create_table()?;
    go_table.set("test_object", test_go)?;
    lua.globals().set("game_objects", go_table)?;

    let _ = lua.load(script.as_str()).exec()?;
    Ok(())
}

