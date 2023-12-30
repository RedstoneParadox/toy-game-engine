pub struct GameObject {
    name: String,
    uuid: String,
    components: Vec<Box<dyn Component>>
}

impl GameObject {
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