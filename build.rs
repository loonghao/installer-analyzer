use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Tell Cargo to rerun this build script if frontend files change
    println!("cargo:rerun-if-changed=frontend/src");
    println!("cargo:rerun-if-changed=frontend/index.html");
    println!("cargo:rerun-if-changed=frontend/package.json");
    println!("cargo:rerun-if-changed=frontend/vite.config.js");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let frontend_dir = Path::new(&manifest_dir).join("frontend");
    let dist_dir = frontend_dir.join("dist");
    let template_path = dist_dir.join("template.html");

    // Check if frontend directory exists
    if !frontend_dir.exists() {
        panic!("Frontend directory not found at: {}", frontend_dir.display());
    }

    // Check if node_modules exists, if not run npm install
    let node_modules = frontend_dir.join("node_modules");
    if !node_modules.exists() {
        println!("cargo:warning=Installing frontend dependencies...");
        let npm_install = Command::new("npm")
            .args(&["install"])
            .current_dir(&frontend_dir)
            .status()
            .expect("Failed to run npm install");

        if !npm_install.success() {
            panic!("npm install failed");
        }
    }

    // Build the frontend if template doesn't exist or is outdated
    let should_build = !template_path.exists() || {
        // Check if any source files are newer than the template
        let template_modified = template_path.metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

        // Check main source files
        let src_files = [
            frontend_dir.join("src/main.ts"),
            frontend_dir.join("index.html"),
            frontend_dir.join("package.json"),
            frontend_dir.join("vite.config.js"),
        ];

        src_files.iter().any(|file| {
            if let Ok(metadata) = file.metadata() {
                if let Ok(modified) = metadata.modified() {
                    return modified > template_modified;
                }
            }
            false
        })
    };

    if should_build {
        println!("cargo:warning=Building frontend...");
        let npm_build = Command::new("npm")
            .args(&["run", "build"])
            .current_dir(&frontend_dir)
            .status()
            .expect("Failed to run npm build");

        if !npm_build.success() {
            panic!("npm build failed");
        }

        // Verify that the template file was created
        if !template_path.exists() {
            panic!(
                "Template file not found after build at: {}\nFrontend build may have failed.",
                template_path.display()
            );
        }
    }

    // Set environment variable for the template path
    println!("cargo:rustc-env=TEMPLATE_PATH={}", template_path.display());
    println!("cargo:warning=Using template at: {}", template_path.display());
}
