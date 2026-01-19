use tonic::{service::Interceptor, Request, Status};

#[derive(Clone)]
pub struct AuthInterceptor {
    token: String,
}

impl AuthInterceptor {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        match request.metadata().get("authorization") {
            Some(t) if t == &format!("Bearer {}", self.token) => Ok(request),
            _ => Err(Status::unauthenticated("Invalid or missing token")),
        }
    }
}
