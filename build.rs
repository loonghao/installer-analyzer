use std::env;
use std::path::Path;

fn main() {
    // Tell Cargo to rerun this build script if the template file changes
    println!("cargo:rerun-if-changed=templates/report.html");

    // Verify that the template file exists
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let template_path = Path::new(&manifest_dir)
        .join("templates")
        .join("report.html");

    if !template_path.exists() {
        panic!(
            "Template file not found at: {}\nPlease ensure templates/report.html exists in the project root.",
            template_path.display()
        );
    }

    println!("cargo:rustc-env=TEMPLATE_PATH={}", template_path.display());
}
