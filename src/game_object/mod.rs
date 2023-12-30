use uuid::Uuid;

pub struct GameObject {
    name: String,
    uuid: Uuid,
    components: Vec<Box<dyn Component>>
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


pub trait Component {
    fn tick(&mut self);
    fn get_name(self) -> String;
}