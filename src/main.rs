use bevy::prelude::*;

fn main() {
    let arg = "hello";
    App::build()
        .add_system(Box::new(move |mut cmd: Commands| {
            println!("this system uses the moved arg: {:?}", arg);
            let id = cmd.spawn().id();
            println!("also it spawned an entity {:?}", id);
        }).system())
        .run();
}
