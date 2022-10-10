mod route;
pub(crate) mod schemas;

use rocket::Route;

lazy_static! {
    pub static ref ROUTES: Vec<Route> = routes![route::status];
}
