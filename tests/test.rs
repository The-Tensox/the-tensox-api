// See https://rocket.rs/v0.4/guide/testing/#local-dispatching
#[cfg(test)]
mod test {
    use the_tensox_api::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn get_objects() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/objects").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}