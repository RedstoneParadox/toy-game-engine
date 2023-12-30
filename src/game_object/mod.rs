mod component;

use mlua::{FromLua, IntoLua, Lua, Value};
use mlua::prelude::LuaUserData;
use mlua::Value::Table;
use uuid::Uuid;
use component::Component;

pub struct GameObject {
    name: String,
    uuid: Uuid,
    components: Vec<Component>
}

impl GameObject {
    fn new(name: String) -> GameObject {
        GameObject {
            name,
            uuid: Uuid::new_v4(),
            components: vec![],
        }
    }
    fn tick(self) {
        for mut component in self.components {
            component.tick()
        }
    }
}

impl IntoLua<'_> for GameObject {
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        let go_table = lua.create_table()?;
        let component_array = lua.create_table()?;


        for component in self.components {
            component_array.push(component);
        }

        go_table.set("name", self.name)?;
        go_table.set("uuid", self.uuid.as_u128())?;
        go_table.set("components", component_array)?;

        return Ok(Table(go_table))
    }
}

