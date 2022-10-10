mod errors;
mod route;
mod schemas;

use rocket::Route;

lazy_static! {
    pub static ref ROUTES: Vec<Route> = routes![
        route::compatible_file_extensions,
        route::slide_size,
        route::slide,
        route::thumbnail
    ];
}
