use crate::app::http::controllers::WelcomeController;
use ruvel::{
    http::{Response, Status},
    routing::Router,
};

pub fn routes(router: &mut Router) {
    router.get("/", |_req| async move { () });
    router.get("/user/{id}", |req| async move {
        req.param("id").unwrap_or_default().to_string()
    });
    router.post("/user", |_req| async move { Response::ok().json("") });
    router.get("/status", |_req| async move { (Status::Ok, "All Good") });
    router.get("/welcome", |req| async move {
        WelcomeController.welcome(req).await
    });
}
