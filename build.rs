#[cfg(target_os = "linux")]
fn main() {
    for lib in &["X11", "xcb", "X11-xcb", "Xau", "Xdmcp"] {
        println!("cargo:rustc-link-lib=static={}", lib);
    }
}

#[cfg(target_os = "windows")]
fn main() {

}