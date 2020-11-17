fn main() {
    pkg_config::Config::new().probe("libsystemd").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
