fn main () {
    // This links OpenBLAS statically and exports the needed environment variables
    openblas_src::Build::new().target("x86_64").static_link(true).compile();
}
