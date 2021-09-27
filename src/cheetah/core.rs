use std::io;
use std::rc::Rc;

pub struct Scene  {
    pub name: String, // Simply used for debugging purposes while the Scene is not actually implemented
}

impl Scene {
    pub fn from(name: &str) -> Scene {
        Scene { name: String::from(name) }
    }
}

pub struct Application {
    pub scenes: Vec<Rc<Scene>>,
    pub active_scene: Rc<Scene>,
}

impl Application {
    pub fn run() {
        println!("Cheetah Engine running... (type 'exit' to terminate)");
        let mut input = String::new();

        loop {
            io::stdin().read_line(&mut input).expect("Failed to read input");
            if input.contains("exit") {
                break;
            }
        }

        println!("Finishing up...");
    }
}
