//! Generate SQL migrations from accounting entities
//!
//! This binary generates SQL CREATE TABLE statements from all Lifeguard entities
//! in the accounting domain.

// Import modules, not structs, so we can access Entity nested structs
use accounting_entities::accounting::{
    general_ledger,
    invoice,
    accounts_receivable,
    accounts_payable,
    bank_sync,
    asset,
    budget,
    edi,
    financial_reports,
};

use lifeguard_migrate::sql_generator;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = PathBuf::from("../../migrations/generated");
    
    // Create output directory if it doesn't exist
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
        println!("📁 Created output directory: {}", output_dir.display());
    }
    
    println!("🔨 Generating SQL migrations from entities...\n");
    
    // Group entities by service
    let mut sql_by_service: std::collections::HashMap<String, Vec<(String, String)>> = std::collections::HashMap::new();
    
    // Helper to generate SQL for each entity using its concrete Entity type
    // We'll call Entity::table_definition() directly on each Entity type
    
    // General Ledger entities
    // Entity is a sibling struct in the same module, not nested inside the struct
    {
        type Entity = general_ledger::chart_of_accounts::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("general_ledger".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = general_ledger::account::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("general_ledger".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = general_ledger::journal_entry::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("general_ledger".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = general_ledger::journal_entry_line::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("general_ledger".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = general_ledger::account_balance::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("general_ledger".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    // Invoice entities
    {
        type Entity = invoice::invoice::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("invoice".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = invoice::invoice_line::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("invoice".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    // Accounts Receivable entities
    {
        type Entity = accounts_receivable::customer_invoice::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("accounts_receivable".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = accounts_receivable::ar_payment::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("accounts_receivable".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = accounts_receivable::ar_payment_application::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("accounts_receivable".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = accounts_receivable::ar_aging::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("accounts_receivable".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    // Accounts Payable entities
    {
        type Entity = accounts_payable::vendor_invoice::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("accounts_payable".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = accounts_payable::ap_payment::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("accounts_payable".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = accounts_payable::ap_payment_application::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("accounts_payable".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = accounts_payable::ap_aging::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("accounts_payable".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    // Bank Sync entities
    {
        type Entity = bank_sync::bank_account::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("bank_sync".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = bank_sync::bank_transaction::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("bank_sync".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = bank_sync::bank_statement::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("bank_sync".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = bank_sync::bank_reconciliation::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("bank_sync".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    // Asset entities
    {
        type Entity = asset::asset::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("asset".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = asset::asset_category::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("asset".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = asset::asset_depreciation::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("asset".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = asset::asset_transaction::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("asset".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    // Budget entities
    {
        type Entity = budget::budget::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("budget".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = budget::budget_period::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("budget".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = budget::budget_line_item::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("budget".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = budget::budget_version::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("budget".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = budget::budget_actual::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("budget".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    // EDI entities
    {
        type Entity = edi::edi_document::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("edi".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = edi::edi_format::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("edi".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = edi::edi_mapping::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("edi".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = edi::edi_acknowledgment::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("edi".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    // Financial Reports entities
    {
        type Entity = financial_reports::financial_report::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("financial_reports".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = financial_reports::report_template::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("financial_reports".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = financial_reports::report_schedule::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("financial_reports".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    {
        type Entity = financial_reports::report_data::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service.entry("financial_reports".to_string()).or_insert_with(Vec::new).push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }
    
    // Write SQL files grouped by service
    let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
    
    for (service, tables) in sql_by_service {
        let service_dir = output_dir.join(&service);
        if !service_dir.exists() {
            fs::create_dir_all(&service_dir)?;
        }
        
        let output_file = service_dir.join(format!("{}_generated_from_entities.sql", timestamp));
        
        let mut sql_content = String::new();
        sql_content.push_str("-- Migration: Generated from Lifeguard entities\n");
        sql_content.push_str(&format!("-- Service: {}\n", service));
        sql_content.push_str(&format!("-- Version: {}\n", timestamp));
        sql_content.push_str(&format!("-- Generated: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        sql_content.push_str("-- This migration was automatically generated from entity definitions.\n");
        sql_content.push_str("-- DO NOT EDIT MANUALLY - regenerate from entities instead.\n\n");
        
        for (table_name, sql) in tables {
            sql_content.push_str(&format!("-- Table: {}\n", table_name));
            sql_content.push_str(&sql);
            sql_content.push_str("\n\n");
        }
        
        fs::write(&output_file, sql_content)?;
        println!("✅ Generated SQL migration for {}: {}", service, output_file.display());
    }
    
    println!("\n✅ Success - All migrations generated!");
    
    Ok(())
}
