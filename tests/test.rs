// See https://rocket.rs/v0.4/guide/testing/#local-dispatching
#[cfg(test)]
mod test {
    // TODO: fix imports
    /*
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;
    */

    #[test]
    fn hello_world() {
        /*
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/objects").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
        */
    }
}