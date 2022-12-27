
pub trait Draw {
    fn draw(&self);
}

// this is a syandin for any struct that might
// implement draw
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    println!("Hello, world!");
}
