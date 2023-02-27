use std::path::Path;

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    // Generate client module.
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("routeguide-client/src")
        .compile(
            &[manifest_dir.join("protos/route_guide.proto")],
            &[manifest_dir.join("protos")],
        )
        .expect("failed to compile protos for client");

    // Generate server module.
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .out_dir("routeguide-server/src")
        .compile(
            &[manifest_dir.join("protos/route_guide.proto")],
            &[manifest_dir.join("protos")],
        )
        .expect("failed to compile protos for server");
}
