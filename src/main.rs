use std::collections::BTreeMap;
use handlebars::Handlebars;

#[macro_use]
extern crate rocket;

#[get("/<text>")]
fn index(text: String) -> String {
    let mut handlebars = Handlebars::new();
    let source = "You entered: {{text}}";
    let mut data = BTreeMap::new();
    data.insert("text".to_string(), text);
    
    handlebars.render_template(source, &data).unwrap()
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index]);

    Ok(rocket.into())
}
