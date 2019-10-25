// See https://rocket.rs/v0.4/guide/testing/#local-dispatching
#[cfg(test)]
mod test {
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;
    use the_tensox_api::rocket;

    #[test]
    fn get_objects() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/objects").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_object() {
        // Well get and post tests are identical ...
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/objects")
            .header(ContentType::JSON)
            .body(r#"{ "position_x": 0.0 }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let id = response.body_string().unwrap();
        let id: Vec<&str> = id.split("\"").collect();
        let mut response = client.get(format!("/objects/{}", id[3])).dispatch();
        assert!(response.body().is_some());
        assert!(response.body_string().unwrap().contains(&id[3]));
        client.delete("/objects").dispatch();
    }

    #[test]
    fn post_object() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/objects")
            .header(ContentType::JSON)
            .body(r#"{ "position_x": 0.0 }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let id = response.body_string().unwrap();
        let id: Vec<&str> = id.split("\"").collect();
        let mut response = client.get(format!("/objects/{}", id[3])).dispatch();
        assert!(response.body().is_some());
        assert!(response.body_string().unwrap().contains(&id[3]));
        client.delete("/objects").dispatch();
    }

    #[test]
    fn update_object() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/objects")
            .header(ContentType::JSON)
            .body(r#"{ "position_x": 0.0 }"#)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_some());
        let id = response.body_string().unwrap();
        let id: Vec<&str> = id.split("\"").collect();
        let response = client
            .put(format!("/objects/{}", id[3]))
            .header(ContentType::JSON)
            .body(r#"{ "position_x": 1.27 }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let mut response = client.get(format!("/objects/{}", id[3])).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_some());
        assert!(response.body_string().unwrap().contains("1.27"));
        client.delete("/objects").dispatch();
    }

    #[test]
    fn delete_object() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/objects")
            .header(ContentType::JSON)
            .body(r#"{ "position_x": 0.0 }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let id = response.body_string().unwrap();
        let id: Vec<&str> = id.split("\"").collect();
        let mut response = client.delete(format!("/objects/{}", id[3])).dispatch();
        assert!(response.body().is_some());
        assert!(response.body_string().unwrap().contains(&id[3]));
        client.delete("/objects").dispatch();
    }

    #[test]
    fn delete_all() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        client.delete("/objects").dispatch();
        let response = client
            .post("/objects")
            .header(ContentType::JSON)
            .body(r#"{ "position_x": 0.0 }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response = client.delete("/objects").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
