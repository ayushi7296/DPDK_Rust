fn main() {
    // Use pkg-config to locate DPDK libraries
    let dpdk_libs = pkg_config::Config::new()
        .probe("libdpdk")
        .expect("Failed to find DPDK using pkg-config");

    // Add linker search paths
    for path in &dpdk_libs.link_paths {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    // Link all DPDK libraries
    for lib in &dpdk_libs.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate bindings");

    bindings
        .write_to_file(std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Failed to write bindings");

    // Write bindings to a file
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}