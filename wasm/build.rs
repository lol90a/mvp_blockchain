use sails_idl_gen::program;
use std::{env, fs, fs::File, path::PathBuf};
use app::BlockchainProgram;

fn main() {
    gwasm_builder::build();

    let manifest_dir_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let idl_file_dir_path = manifest_dir_path.join("target");

    // Ensure the target directory exists
    if !idl_file_dir_path.exists() {
        fs::create_dir_all(&idl_file_dir_path).expect("Could not create target directory");
    }

    let idl_file_path = idl_file_dir_path.join("Sails.idl");

    let idl_file = File::create(idl_file_path).expect("Could not create .idl file in target directory");

    // Generate the IDL for the BlockchainProgram and catch any errors
    if let Err(e) = program::generate_idl::<BlockchainProgram>(idl_file) {
        eprintln!("Error generating IDL: {:?}", e);
        panic!("Failed to generate IDL");
    }
}
