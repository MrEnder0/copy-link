#[macro_use]
extern crate rocket;

#[get("/<text>")]
fn index(text: String) -> String {
    format!("You entered: {}", text)
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index]);

    Ok(rocket.into())
}
