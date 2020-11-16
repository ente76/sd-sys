fn main() {
    pkg_config::Config::new().probe("systemd").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
