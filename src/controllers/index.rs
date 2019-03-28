use std::collections::BTreeMap;
extern crate pencil;

use pencil::{Request, Response, PencilResult};
use crate::write_html;

pub fn index(request: &mut Request) -> PencilResult {
    use std::fmt::Write;
    let mut out = String::new();
    write_html!(&mut out,
        html[
            head[title["My page"]]
            body[
                h1["Welcome to my page"]
                p["This is awesome"]
                p["Do you agree?"]
    ]]);
    Ok(Response::from(out))
}

pub fn hello(request: &mut Request) -> PencilResult {
    let mut context = BTreeMap::new();
    context.insert("name".to_string(), "template".to_string());
    return request.app.render_template("hello.html", &context);
}