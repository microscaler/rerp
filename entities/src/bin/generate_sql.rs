//! Binary to generate SQL migrations from entity registry
//!
//! This binary uses the compiled entity registry to generate SQL CREATE TABLE statements
//! for all entities in the accounting domain.

use accounting_entities::entity_registry;
use chrono::Utc;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = PathBuf::from("../../migrations/generated");

    // Create output directory if it doesn't exist
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
        println!("📁 Created output directory: {}", output_dir.display());
    }

    println!("🔨 Generating SQL migrations from entity registry...");

    // Generate SQL for all entities
    let sql_results = entity_registry::generate_sql_for_all()
        .map_err(|e| format!("Failed to generate SQL: {}", e))?;

    if sql_results.is_empty() {
        println!("⚠️  No SQL generated - no entities found in registry");
        return Ok(());
    }

    println!("✅ Generated SQL for {} entities", sql_results.len());

    // Get metadata for service path lookup
    let metadata = entity_registry::all_entity_metadata();

    // Group SQL by service path
    let mut sql_by_service: HashMap<String, Vec<(String, String)>> = HashMap::new();

    for (table_name, sql) in sql_results {
        // Extract service path from metadata
        let service_path = metadata
            .iter()
            .find(|m| m.table_name == table_name)
            .map(|m| m.service_path.to_string())
            .unwrap_or_else(|| "default".to_string());

        sql_by_service
            .entry(service_path)
            .or_insert_with(Vec::new)
            .push((table_name, sql));
    }

    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();

    // Generate SQL files per service
    for (service_path, entities) in &sql_by_service {
        // Determine output file path based on service
        let service_output_dir = if service_path != "default" && !service_path.is_empty() {
            output_dir.join(&service_path)
        } else {
            output_dir.clone()
        };

        // Create service-specific output directory
        if !service_output_dir.exists() {
            fs::create_dir_all(&service_output_dir)?;
        }

        let output_file =
            service_output_dir.join(format!("{}_generated_from_entities.sql", timestamp));

        let mut sql_content = String::new();
        sql_content.push_str("-- Migration: Generated from Lifeguard entities\n");
        sql_content.push_str(&format!("-- Service: {}\n", service_path));
        sql_content.push_str(&format!("-- Version: {}\n", timestamp));
        sql_content.push_str(&format!(
            "-- Generated: {}\n\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));
        sql_content
            .push_str("-- This migration was automatically generated from entity definitions.\n");
        sql_content.push_str("-- DO NOT EDIT MANUALLY - regenerate from entities instead.\n\n");

        // Add SQL for each entity
        for (table_name, sql) in entities {
            sql_content.push_str(&format!("-- Table: {}\n", table_name));
            sql_content.push_str(&sql);
            sql_content.push_str("\n\n");
        }

        // Write the complete SQL file for this service
        fs::write(&output_file, sql_content)?;
        println!(
            "✅ Generated SQL migration for {}: {}",
            service_path,
            output_file.display()
        );
    }

    println!(
        "✅ Success - Generated {} migration file(s)",
        sql_by_service.len()
    );

    Ok(())
}
