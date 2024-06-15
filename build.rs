fn main() {
    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/helloworld.proto"], &["proto"])
        .unwrap();
}
