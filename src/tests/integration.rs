#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use crate::routes::user::init;

    #[actix_rt::test]
    async fn test_get_user() {
        let mut app = test::init_service(App::new().configure(init)).await;
        let req = test::TestRequest::get().uri("/user").to_request();
        let resp: crate::models::user::User = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.name, "John Doe");
    }

    #[actix_rt::test]
    async fn test_create_user() {
        let mut app = test::init_service(App::new().configure(init)).await;
        let user = crate::models::user::User { id: 1, name: "Jane Doe".to_string() };
        let req = test::TestRequest::post().uri("/user").set_json(&user).to_request();
        let resp: crate::models::user::User = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.name, "Jane Doe");
    }
}
