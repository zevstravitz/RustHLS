extern crate gotham;

use gotham::handler::assets::FileOptions;
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};

pub fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Need to pass an arg which is the path to serve");
    let addr = "127.0.0.1:7878";
    println!(
        "Listening for requests at http://{} from path {:?}",
        addr, path
    );

    let router = build_simple_router(|route| {
        route.get("/demo.m3u8").to_file("assets/demo.m3u8");
    });

    gotham::start(addr, router)
}