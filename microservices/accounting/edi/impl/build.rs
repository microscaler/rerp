use lifeguard_migrate::build_script::{discover_entities, generate_registry_module};
use std::env;
use std::path::PathBuf;

fn main() {{
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let src_dir = PathBuf::from("src");

    // Registry + generate_sql_for_all(): entities ordered by same-crate FKs (lifeguard-migrate).
    println!("cargo:rerun-if-changed=src/models");
    let entities = discover_entities(&src_dir).expect("Failed to discover entities");

    let registry_path = out_dir.join("entity_registry.rs");
    generate_registry_module(&entities, &registry_path).expect("Failed to generate registry");
}}
