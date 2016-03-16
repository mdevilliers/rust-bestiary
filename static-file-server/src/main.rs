extern crate iron;
extern crate staticfile;
extern crate mount;

use std::path::Path;

use iron::Iron;
use staticfile::Static;
use mount::Mount;
use std::env;

fn main() {

        // print out the env vars
        for (key, value) in env::vars_os() {
            println!("{:?}: {:?}", key, value);
        }

        let mut mount = Mount::new();

        // Serve the shared Web folder at /
        mount.mount("/", Static::new(Path::new("web/")));

        println!("Server running on http://localhost:3000/");
        Iron::new(mount).http("127.0.0.1:3000").unwrap();
}
