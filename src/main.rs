use std::sync::{LazyLock, Mutex};

use helpers::Maths;
use rocket::{get, launch, post, routes};
mod helpers;

static MATHS: LazyLock<Mutex<Maths>> = LazyLock::new(|| Mutex::new(Maths::new(None)));

#[get("/")]
fn index() -> &'static str {
    "Maths-Lang-API: Version: 0.1.0!"
}

#[post("/run", data = "<input>")]
fn run(input: String) -> String {
    let mut maths = MATHS.lock().unwrap();
    maths.run(&input);

    if let Some(result) = maths.get_result() {
        format!("{}", result)
    } else {
        "0.0".to_string()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, run]) 
}

#[cfg(test)]
mod tests {
    use rocket::local::blocking::Client;
    use super::rocket;

    #[test]
    fn test_run_route() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/run")
            .body("let result = @CubeRt(27)")
            .dispatch();
        
        assert_eq!(response.status(), rocket::http::Status::Ok);
        assert_eq!(response.into_string().unwrap(), "3");
    }
}