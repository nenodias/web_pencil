extern crate pencil;

use pencil::{Pencil};

use crate::controllers::index;

pub fn init(app: &mut Pencil) {
    app.get("/", "index", index::index);
    app.get("/hello", "hello", index::hello);
    app.get("/json", "json", index::json);
}