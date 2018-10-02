extern crate protoc_rust_grpc;

use std::env;
use std::process::Command;

fn main() {
    let proto_gen_dir = "src/gen/proto";
    let proto_gen_abs_dir = &format!("{}/{}", env::var("CARGO_MANIFEST_DIR").unwrap(), proto_gen_dir);
    println!("{}", proto_gen_abs_dir);

    Command::new("mkdir").args(&["-p", proto_gen_abs_dir]).status().unwrap();

    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: proto_gen_dir,
        includes: &["proto/googleapis"],
        input: &["proto/googleapis/google/datastore/v1beta3/datastore.proto",
                 "proto/googleapis/google/datastore/v1beta3/entity.proto",
                 "proto/googleapis/google/datastore/v1beta3/query.proto",
                 "proto/googleapis/google/type/latlng.proto"],
        rust_protobuf: true,
        ..Default::default()
    }).expect("protoc-rust-grpc");
}
