//! Generate SQL migrations from accounting entities
//!
//! This binary generates SQL CREATE TABLE statements from all Lifeguard entities
//! in the accounting domain.

// Import modules, not structs, so we can access Entity nested structs
use accounting_entities::accounting::{
    accounts_payable, accounts_receivable, asset, bank_sync, budget, edi, financial_reports,
    general_ledger, invoice,
};

use lifeguard_migrate::{sql_generator, dependency_ordering};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

/// Extract foreign key dependencies from SQL by parsing REFERENCES clauses
fn extract_dependencies_from_sql(sql: &str) -> Vec<String> {
    let mut dependencies = Vec::new();
    
    // Look for REFERENCES clauses in the SQL
    // Pattern: REFERENCES table_name(column) or REFERENCES schema.table_name(column)
    // We'll use simple string parsing since we control the SQL format
    let mut search_pos = 0;
    while let Some(ref_pos) = sql[search_pos..].find("REFERENCES") {
        let start = search_pos + ref_pos + "REFERENCES".len();
        let remaining = &sql[start..];
        
        // Skip whitespace
        let remaining = remaining.trim_start();
        
        // Extract table name (may be schema.table or just table)
        let table_end = remaining
            .find('(')
            .or_else(|| remaining.find(' '))
            .or_else(|| remaining.find('\n'))
            .unwrap_or(remaining.len());
        
        let table_ref = &remaining[..table_end].trim();
        
        // Handle schema.table format
        let table_name = if let Some(dot_pos) = table_ref.rfind('.') {
            &table_ref[dot_pos + 1..]
        } else {
            table_ref
        };
        
        if !table_name.is_empty() && !dependencies.contains(&table_name.to_string()) {
            dependencies.push(table_name.to_string());
        }
        
        search_pos = start + table_end;
        if search_pos >= sql.len() {
            break;
        }
    }
    
    dependencies
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = PathBuf::from("../../migrations/generated");

    // Create output directory if it doesn't exist
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
        println!("📁 Created output directory: {}", output_dir.display());
    }

    println!("🔨 Generating SQL migrations from entities...\n");

    // Group entities by service
    let mut sql_by_service: std::collections::HashMap<String, Vec<(String, String)>> =
        std::collections::HashMap::new();

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
                sql_by_service
                    .entry("general_ledger".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("general_ledger".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("general_ledger".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("general_ledger".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("general_ledger".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("invoice".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("invoice".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("accounts_receivable".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("accounts_receivable".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("accounts_receivable".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("accounts_receivable".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("accounts_payable".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("accounts_payable".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("accounts_payable".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("accounts_payable".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }

    // Bank Sync entities
    // Note: Order matters - banks must come before bank_accounts, which must come before
    // bank_transactions, bank_statements, and bank_reconciliations
    {
        type Entity = bank_sync::bank::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service
                    .entry("bank_sync".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
                println!("   ✓ Generated SQL for {}", table_name);
            }
            Err(e) => eprintln!("   ✗ Failed to generate SQL for {}: {}", table_name, e),
        }
    }

    {
        type Entity = bank_sync::bank_account::Entity;
        let entity = Entity::default();
        let table_name = entity.table_name();
        let table_def = Entity::table_definition();
        match sql_generator::generate_create_table_sql::<Entity>(table_def) {
            Ok(sql) => {
                sql_by_service
                    .entry("bank_sync".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("bank_sync".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("bank_sync".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("bank_sync".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("asset".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("asset".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("asset".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("asset".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("budget".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("budget".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("budget".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("budget".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("budget".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("edi".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("edi".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("edi".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("edi".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("financial_reports".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("financial_reports".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("financial_reports".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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
                sql_by_service
                    .entry("financial_reports".to_string())
                    .or_insert_with(Vec::new)
                    .push((table_name.to_string(), sql));
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

        // Build TableInfo structures for dependency analysis
        let mut table_infos: Vec<dependency_ordering::TableInfo> = Vec::new();
        let mut table_sql_map: HashMap<String, String> = HashMap::new();
        
        for (table_name, sql) in &tables {
            table_sql_map.insert(table_name.clone(), sql.clone());
            
            // Extract foreign key dependencies from the SQL
            // We parse the SQL to find REFERENCES clauses
            let dependencies = extract_dependencies_from_sql(sql);
            
            table_infos.push(dependency_ordering::TableInfo {
                name: table_name.clone(),
                sql: sql.clone(),
                dependencies,
            });
        }
        
        // Validate foreign key references
        if let Err(e) = dependency_ordering::validate_foreign_key_references(&table_infos) {
            eprintln!("⚠️  Warning for {} service: {}", service, e);
            eprintln!("   This migration may fail when applied due to missing foreign key references.");
        }
        
        // Topologically sort tables by dependencies
        let sorted_table_names = match dependency_ordering::topological_sort(&table_infos) {
            Ok(sorted) => sorted,
            Err(e) => {
                eprintln!("⚠️  Warning for {} service: {}", service, e);
                eprintln!("   Tables will be written in original order.");
                // Fall back to original order
                tables.iter().map(|(name, _)| name.clone()).collect()
            }
        };

        let mut sql_content = String::new();
        sql_content.push_str("-- Migration: Generated from Lifeguard entities\n");
        sql_content.push_str(&format!("-- Service: {}\n", service));
        sql_content.push_str(&format!("-- Version: {}\n", timestamp));
        sql_content.push_str(&format!(
            "-- Generated: {}\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));
        sql_content
            .push_str("-- This migration was automatically generated from entity definitions.\n");
        sql_content.push_str("-- DO NOT EDIT MANUALLY - regenerate from entities instead.\n\n");

        // Write tables in dependency order
        for table_name in sorted_table_names {
            if let Some(sql) = table_sql_map.get(&table_name) {
                sql_content.push_str(&format!("-- Table: {}\n", table_name));
                sql_content.push_str(sql);
                sql_content.push_str("\n\n");
            }
        }

        fs::write(&output_file, sql_content)?;
        println!(
            "✅ Generated SQL migration for {}: {}",
            service,
            output_file.display()
        );
    }

    println!("\n✅ Success - All migrations generated!");

    Ok(())
}
