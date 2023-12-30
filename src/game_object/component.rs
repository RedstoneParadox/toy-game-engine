use mlua::{FromLua, IntoLua, Lua, Value};
use mlua::Value::Table;

pub enum Component {
    Messenger {
        message: String,
        has_sent: bool
    }
}

impl IntoLua<'_> for Component {
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        let table = lua.create_table()?;

        match self {
            Component::Messenger { message, has_sent } => {
                table.set("message", message)?;
                table.set("has_sent", has_sent)?;
            }
        }

        return Ok(Table(table))
    }
}

impl FromLua<'_> for Component {
    fn from_lua<'lua>(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        if let Table(table) = value {

        }

        return Err(mlua::Error::FromLuaConversionError {
            from: "table",
            to: "Component",
            message: None,
        })
    }
}

impl Component {
    pub(crate) fn tick(self) {

    }
}