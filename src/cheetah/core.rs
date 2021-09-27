use std::io;

/// The starting point to a Cheetah program. An `Application` contains a list of `Scene`s, along with
/// an active `Scene` currently in play.
pub struct Application {
    pub scenes: Vec<Scene>,
    pub active_scene: Scene,
}

impl Application {
    pub fn run(&mut self) {
        println!("Cheetah Engine running... (type 'exit' to terminate)");

        let mut input = String::new();

        loop {
            println!("Scenes in application:");
            for scene in &self.scenes {
                println!("{}", scene.name);
            }

            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            input = String::from(input.trim_end());

            if input.contains("exit") {
                break;
            }
            
            self.add_scene(&input.as_str());
            
            println!();
            println!("Added a scene!");
        }

        println!("Finishing up...");
    }

    fn add_scene(&mut self, name: &str) {
        self.scenes.push(Scene::from(name));
    }
}

/// A `Scene` in the application. This could be a game level, or a window in an application
#[derive(Clone)]
pub struct Scene  {
    pub name: String, // Simply used for debugging purposes while the Scene is not actually implemented
}

impl Scene {
    pub fn from(name: &str) -> Scene {
        Scene { name: String::from(name) }
    }
}

