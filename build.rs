fn main() {
    tonic_build::configure()
        .build_server(false)
        .build_client(false)
        .compile_protos(&["google/rpc/status.proto"], &["proto"])
        .expect("cannot compile Protocol Buffers");
}
