//! Build script for entity registry generation
//!
//! This build script discovers all Lifeguard entities in the source directory
//! and generates a registry module that can be used by the CLI tool to generate
//! SQL migrations from compiled entities.

use lifeguard_migrate::build_script;
use std::env;
use std::path::Path;

fn main() {
    // Discover entities in source directory
    let source_dir = Path::new("src");
    
    println!("cargo:rerun-if-changed=src");
    
    let entities = match build_script::discover_entities(source_dir) {
        Ok(entities) => {
            if entities.is_empty() {
                println!("cargo:warning=No entities found in src/ directory");
                return;
            }
            println!("cargo:warning=Discovered {} entities", entities.len());
            entities
        }
        Err(e) => {
            println!("cargo:warning=Failed to discover entities: {}", e);
            return;
        }
    };
    
    // Generate registry module in OUT_DIR
    let out_dir = env::var("OUT_DIR")
        .expect("OUT_DIR environment variable not set");
    let registry_path = Path::new(&out_dir).join("entity_registry.rs");
    
    match build_script::generate_registry_module(&entities, &registry_path) {
        Ok(()) => {
            println!("cargo:warning=Generated entity registry at {:?}", registry_path);
        }
        Err(e) => {
            println!("cargo:warning=Failed to generate registry: {}", e);
        }
    }
}
