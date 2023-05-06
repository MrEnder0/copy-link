use std::collections::BTreeMap;
use handlebars::Handlebars;
use rocket::response::{content::RawHtml, Redirect};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/Hello!!!")
}

#[get("/<text>")]
fn text(text: String) -> RawHtml<String> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("index", "templates/index.hbs").unwrap();
    let mut data = BTreeMap::new();
    data.insert("text", text);
    let handlebars_output = handlebars.render("index", &data).unwrap();

    //render as html with css
    RawHtml(handlebars_output)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, text])
}