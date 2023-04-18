use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // Compile our proto
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("trace_descriptor.bin"))
        // We want to be able to convert proto to json, this enables that.
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        // Prost will automatically include it's own prosy_types for google Timestamp etc.
        // But this doesn't have Serialice and Deseriealize so we switch it off.
        // Maybe we could use https://github.com/fdeantoni/prost-wkt
        .compile_well_known_types(true)
        .compile(&["api.proto"], &["./"])
        .unwrap();
}
