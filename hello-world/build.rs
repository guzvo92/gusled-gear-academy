use hello_world_io::HelloMetadata;
fn main() {
    gear_wasm_builder::build_with_metadata::<HelloMetadata>();
}

//con esto hace que apareza el .metahash en el root