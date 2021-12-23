
struct ImageSpec {
    specs: Vec<Spec>,
}

enum Spec {
    Resize(Resize),
    Crop(Crop),
}

struct Resize {
    width: u32,
    height: u32,
}




fn main() {
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}