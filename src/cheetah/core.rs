use super::logger;
use std::io;

/// The starting point to a Cheetah program. An `Application` contains a list of `Scene`s, along with
/// an active `Scene` currently in play.
pub struct Application {
    pub scenes: Vec<Scene>,
    pub active_scene: Scene,
}

impl Application {
    pub fn run(&mut self) {
        logger::infoln("Cheetah Engine running... (type 'exit' to terminate)");

        let mut input = String::new();

        loop {
            logger::infoln("Scenes in application:");
            for scene in &self.scenes {
                logger::infoln(scene.name.as_str());
            }

            input.clear();
            match io::stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(_) => {
                    logger::errorln("Failed to read line!");
                }
            }
            input = String::from(input.trim_end());

            if input.contains("exit") {
                break;
            }

            self.add_scene(&input.as_str());

            logger::infoln("");
            logger::infoln("Added a scene!");
        }

        logger::infoln("Finishing up...");
    }

    fn add_scene(&mut self, name: &str) {
        self.scenes.push(Scene::from(name));
    }
}

/// A `Scene` in the application. This could be a game level, or a window in an application
#[derive(Clone)]
pub struct Scene {
    pub name: String, // Simply used for debugging purposes while the Scene is not actually implemented
}

impl Scene {
    pub fn from(name: &str) -> Scene {
        Scene {
            name: String::from(name),
        }
    }
}
