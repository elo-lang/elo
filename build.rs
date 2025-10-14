fn main() {
    println!("cargo:rustc-link-lib=tcc"); 
    println!("cargo:rerun-if-env-changed=TCC_LIB_DIR");
}