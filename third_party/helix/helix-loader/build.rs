const MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
const MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
const PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
const UPSTREAM_REVISION: &str = "079a789e";

fn main() {
    // The vendored source is not a nested Git checkout; querying Git would
    // accidentally stamp the OpenMuse product revision onto the Helix binary.
    let minor = if MINOR.len() == 1 {
        format!("0{MINOR}")
    } else {
        MINOR.to_string()
    };
    let calver = if PATCH == "0" {
        format!("{MAJOR}.{minor}")
    } else {
        format!("{MAJOR}.{minor}.{PATCH}")
    };
    println!(
        "cargo:rustc-env=BUILD_TARGET={}",
        std::env::var("TARGET").unwrap()
    );
    println!(
        "cargo:rustc-env=VERSION_AND_GIT_HASH={} ({UPSTREAM_REVISION})",
        calver
    );
}
