use cheetah::core::{Application, Scene};
use std::rc::Rc;

fn main() {
    let scene1 = Rc::new(Scene::from("Scene 1"));
    let scene2 = Rc::new(Scene::from("Scene 2"));
    let scenes = vec![Rc::clone(&scene1), Rc::clone(&scene2)];
    let app = Application { scenes, active_scene: scene1 };

    println!("Scenes in game:");
    for scene in app.scenes {
        println!("{}", scene.as_ref().name);
    }
}