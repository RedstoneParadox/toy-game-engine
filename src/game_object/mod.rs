pub mod component;

use mlua::{FromLua, IntoLua, Lua, Value};
use mlua::prelude::LuaUserData;
use mlua::Value::Table;
use uuid::Uuid;
use component::Component;

pub struct GameObject {
    pub name: String,
    pub uuid: Uuid,
    pub components: Vec<Component>
}

impl GameObject {
    pub(crate) fn new(name: String) -> GameObject {
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

impl FromLua<'_> for GameObject {
    fn from_lua(value: Value<'_>, lua: &'_ Lua) -> mlua::Result<Self> {
        if let Table(table) = value {
            let name: String = table.get("name")?;
            let uuid: Uuid = Uuid::from_u128(table.get("uuid")?);
            let components_table: Value = table.get("components")?;
            let mut components: Vec<Component> = vec![];

            if let Table(table2) = components_table {
                let components_sequence = table2.sequence_values::<Component>();
                for component in components_sequence {
                    components.push(component?)
                }
            }

            return Ok(GameObject {
                name,
                uuid,
                components
            })
        }

        return Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "GameObject",
            message: None,
        })
    }
}

