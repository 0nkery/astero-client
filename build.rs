extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &["proto/astero.proto", "proto/mmob.proto"],
        &["proto/"]
    ).unwrap();
}
