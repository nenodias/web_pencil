extern crate pencil;

use pencil::{Pencil};

mod routes;
mod controllers;
mod utils;

fn main() {
    let mut app = Pencil::new("");
    app.enable_static_file_handling();// Enable static files
    app.register_template("hello.html");
    routes::init(&mut app);
    app.run("127.0.0.1:5000");
}