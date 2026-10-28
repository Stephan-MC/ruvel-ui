use crate::app::http::controllers::WelcomeController;
use ruvel::routing::Router;

pub fn routes(router: &mut Router) {
    router.get(
        "/",
        |req| async move { WelcomeController.welcome(req).await },
    );
    router.get("/welcome", |_req| async move { "Hello World from Ruvel" });
}
