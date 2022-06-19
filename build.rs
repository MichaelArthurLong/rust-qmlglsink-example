extern crate cpp_build;
extern crate pkg_config;

fn main() {
    // This whole thing is probably half-broken/incomplete because we need
    // to run `cargo clean -p whateverthispackageiscalled` every time
    // anything in here or any C++ code(inside a cpp! macro) is changed

    // Libraries to be imported for use with cpp crate
    let libs = ["gstreamer-1.0", "Qt5Qml", "Qt5Quick"];

    let mut config = cpp_build::Config::new();

    for library in libs {
        let lib = library;
        for path in pkg_config::probe_library(lib).unwrap().include_paths {
            config.include(&path);
        }
    }

    config.build("src/main.rs");
}
