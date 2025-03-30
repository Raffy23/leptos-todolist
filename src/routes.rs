use leptos_router::AsPath;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Route(pub(crate) &'static str);

impl Into<&'static str> for Route {
    fn into(self) -> &'static str {
        self.0
    }
}

impl AsPath for Route {
    fn as_path(&self) -> &'static str {
        self.0
    }
}

pub(crate) const HOME: Route = Route("/");
pub(crate) const LOGIN: Route = Route("/login");
pub(crate) const LOGOUT: Route = Route("/logout");
pub(crate) const REGISTER: Route = Route("/register");
