extern crate pkg_config;

fn main() {
    pkg_config::find_library("he-profiler").unwrap();
}
