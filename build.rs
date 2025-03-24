use bindgen::builder;
use std::path::{Path, PathBuf};
use std::{env, fs};

const GEOSTEINER_DIR: &str = "external/geosteiner-5.3";

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in src.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        let dst = dst.join(path.file_name().unwrap());
        if path.is_dir() {
            copy_dir_all(&path, &dst)?;
        } else {
            fs::copy(&path, &dst)?;
        }
    }
    Ok(())
}

fn main() {
    // copy the geosteiner source code from `external/geosteiner` to the OUT_DIR and build there
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let geosteiner_build_dir = out_dir.join("geosteiner");
    copy_dir_all(Path::new(GEOSTEINER_DIR), &geosteiner_build_dir).unwrap();

    //let gmp = pkg_config::Config::new().atleast_version("6.2").probe("gmp").unwrap();
    //eprintln!("{:?}", gmp);
    //println!("cargo:rustc-link-lib=gmp");

    // build geosteiner. Might fail a few times, but should succeed eventually. The autoconf seems to be broken.
    let mut dst = Err(Default::default());
    for _ in 0..5 {
        dst = autotools::Config::new(&geosteiner_build_dir)
            .insource(true) // unfortunately, geosteiner does not support out-of-source builds
            .config_option("without-cplex", None)
            .config_option("without-gmp", None)
            .cflag("-w") // to find actual errors more easily. geosteiner is full of unsused-variable warnings.
            .make_args(vec!["-j1".to_string()])
            .fast_build(true)
            .try_build();
        if dst.is_ok() {
            break;
        }
    }
    let dst = dst.unwrap();
    eprintln!("geosteiner output {:?}", dst.display());
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=geosteiner");
    println!(
        "cargo:rustc-link-search=native={}/lp_solve_2.3",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=LPS");
    // in source build pollutes the source directory with build files
    println!("cargo::rerun-if-changed={}", GEOSTEINER_DIR);

    let bindings = builder()
        .header(geosteiner_build_dir.join("geosteiner.h").to_str().unwrap())
        .allowlist_type("gst_.*")
        .allowlist_function("gst_.*")
        .allowlist_var("GST_.*")
        .generate()
        .unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
