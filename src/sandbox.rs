use cheetah::core::{Application, Scene};

fn main() {
    let scene1 = Scene::from("Scene 1");
    let scene2 = Scene::from("Scene 2");
    let scenes = vec![scene1.clone(), scene2];
    let mut app = Application {
        scenes,
        active_scene: scene1,
    };
    app.run();
}
