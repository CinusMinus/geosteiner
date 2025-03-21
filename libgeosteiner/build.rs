use bindgen::builder;
use std::path::Path;

const GEOSTEINER_DIR: &str = "external/geosteiner-5.3";

fn main() {
    //let gmp = pkg_config::Config::new().atleast_version("6.2").probe("gmp").unwrap();
    //eprintln!("{:?}", gmp);
    //println!("cargo:rustc-link-lib=gmp");
    let dst = autotools::Config::new(GEOSTEINER_DIR)
        .insource(true)
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


    let bindings = builder().header(Path::new(GEOSTEINER_DIR).join("geosteiner.h").to_str().unwrap())
        .allowlist_type("gst_.*")
        .allowlist_function("gst_.*")
        .allowlist_var("GST_.*")
        .generate()
        .unwrap();
    bindings.write_to_file("src/bindings.rs").unwrap();
}