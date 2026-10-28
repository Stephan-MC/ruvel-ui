use ruvel::http::{Controller, Request, Responsable, Response};

#[derive(Default)]
pub struct WelcomeController;

impl Controller for WelcomeController {
    fn index(&self, _request: Request) -> impl Responsable {
        Response::ok().html("<mark>Welcome</mark> to Ruvel")
    }
}

impl WelcomeController {
    pub async fn welcome(&self, _request: Request) -> &'static str {
        "Welcome to Ruvel Again"
    }
}
