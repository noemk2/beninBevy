use bevy::prelude::*;

fn main() {
    // println!("Hello, world!");
    App::new().add_system(hello_world).run()
}

pub fn hello_world() {
    println!("Hello world")
}
