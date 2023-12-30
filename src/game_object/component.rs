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
        let name = self.get_name();

        table.set("name", name)?;

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
            let name: String = table.get("name")?;

            return  match name.as_str() {
                "messenger" => {
                    let message: String = table.get("message").unwrap_or_default();
                    let has_sent: bool = table.get("has_sent").unwrap_or_default();

                    Ok(Component::Messenger {message, has_sent})
                },
                _ => Err(mlua::Error::FromLuaConversionError {
                    from: "",
                    to: "",
                    message: Some(format!("component '{}' does not exist.", name)),
                })
            };
        }

        return Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Component",
            message: None,
        })
    }
}

impl Component {
    pub(crate) fn tick(&mut self) {

    }

    pub fn get_name(&self) -> String {
        return match self {
            Component::Messenger { .. } => "messenger",
        }.to_string()
    }
}