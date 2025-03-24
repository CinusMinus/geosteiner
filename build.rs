use std::env;
use bindgen::builder;
use std::path::{Path, PathBuf};

const GEOSTEINER_DIR: &str = "external/geosteiner-5.3";

fn main() {
    //let gmp = pkg_config::Config::new().atleast_version("6.2").probe("gmp").unwrap();
    //eprintln!("{:?}", gmp);
    //println!("cargo:rustc-link-lib=gmp");
    let dst = autotools::Config::new(GEOSTEINER_DIR)
        .insource(true) // unfortunately, geosteiner does not support out-of-source builds
        .config_option("without-cplex", None)
        .config_option("without-gmp", None)
        .cflag("-w") // to find actual errors more easily. geosteiner is full of unsused-variable warnings.
        .fast_build(true)
        .try_build().unwrap();
    eprintln!("geosteiner output {:?}", dst.display());
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=geosteiner");
    println!("cargo:rustc-link-search=native={}/lp_solve_2.3", dst.display());
    println!("cargo:rustc-link-lib=static=LPS");
    // in source build pollutes the source directory with build files
    println!("cargo::rerun-if-changed=build.rs");

    let bindings = builder().header(Path::new(GEOSTEINER_DIR).join("geosteiner.h").to_str().unwrap())
        .allowlist_type("gst_.*")
        .allowlist_function("gst_.*")
        .allowlist_var("GST_.*")
        .generate()
        .unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).unwrap();
}