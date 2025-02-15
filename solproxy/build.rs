fn main() {
    tonic_build::compile_protos("../proto/solproxy.proto").unwrap();
}
