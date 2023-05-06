use std::collections::BTreeMap;
use handlebars::Handlebars;

#[macro_use]
extern crate rocket;

#[get("/<text>")]
fn index(text: String) -> String {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("index", "templates/index.hbs").unwrap();
    handlebars.insert("text", &text);
    
    handlebars.render_template(source, &data).unwrap()
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index]);

    Ok(rocket.into())
}
