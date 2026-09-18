use chrono::Utc;
use clap::{Parser, Subcommand, ValueEnum};
use lifeguard_migrate::generated_migration_diff::{combined_old_section, normalize_table_sql_blob};
use lifeguard_migrate::migration_writer::{
    accumulate_per_table_baselines_from_dir, find_existing_per_table_files,
    write_per_table_migration_file, EmissionOutcome, MigrationHeader,
};
use lifeguard_migrate::sql_dependency_order::{
    order_migrations_by_foreign_key_sql, write_apply_order_file, write_seed_order_file,
};
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

type SqlRows = Vec<(String, String)>;
type GenerateSql = fn() -> Result<SqlRows, String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Suite {
    Accounting,
    Documents,
}

impl Suite {
    fn directory_name(self) -> &'static str {
        match self {
            Self::Accounting => "accounting",
            Self::Documents => "documents",
        }
    }
}

#[derive(Debug, Parser)]
#[command(
    name = "rerp_migrator",
    about = "Generate one explicitly selected RERP suite's migrations",
    long_about = "Generate one explicitly selected RERP suite's migrations. Provider crates are compiled with Cargo features; for example: cargo run -p rerp_migrator --features accounting -- generate --suite accounting"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Validate providers, table ownership, and foreign-key ordering without writing files.
    Validate {
        /// Suite whose providers may be loaded.
        #[arg(long, value_enum)]
        suite: Suite,
        /// Also prove that delivered migration history can be evolved safely.
        #[arg(long)]
        migration_history: bool,
    },
    /// Generate migration and seed ordering files for one suite.
    Generate {
        /// Suite whose providers and output directory may be used.
        #[arg(long, value_enum)]
        suite: Suite,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ProviderId {
    suite: Suite,
    service: &'static str,
}

impl ProviderId {
    fn display(self) -> String {
        format!("{}/{}", self.suite.directory_name(), self.service)
    }
}

#[derive(Clone, Debug)]
struct Provider {
    id: ProviderId,
    generate_sql: GenerateSql,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    owner: ProviderId,
    table: String,
    sql: String,
}

#[derive(Debug, PartialEq, Eq)]
struct SuiteLayout {
    suite_root: PathBuf,
    migrations_root: PathBuf,
    seed_order_path: PathBuf,
}

fn suite_layout(manifest_dir: &Path, suite: Suite) -> Result<SuiteLayout, String> {
    let microservices_root = manifest_dir.parent().ok_or_else(|| {
        format!(
            "migrator manifest directory has no microservices parent: {}",
            manifest_dir.display()
        )
    })?;
    let suite_root = microservices_root.join(suite.directory_name());
    Ok(SuiteLayout {
        migrations_root: suite_root.join("migrations"),
        seed_order_path: suite_root.join("seed_order.txt"),
        suite_root,
    })
}

fn discover_seed_files(suite_root: &Path) -> Result<Vec<PathBuf>, String> {
    fn visit(directory: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
        if !directory.is_dir() {
            return Ok(());
        }
        let entries = std::fs::read_dir(directory)
            .map_err(|error| format!("could not read {}: {error}", directory.display()))?;
        for entry in entries {
            let path = entry
                .map_err(|error| format!("could not read {} entry: {error}", directory.display()))?
                .path();
            if path.is_dir() {
                visit(&path, out)?;
                continue;
            }
            let is_seed = path.extension().and_then(|value| value.to_str()) == Some("sql")
                && path
                    .parent()
                    .and_then(Path::file_name)
                    .and_then(|value| value.to_str())
                    == Some("seeds")
                && path
                    .parent()
                    .and_then(Path::parent)
                    .and_then(Path::file_name)
                    .and_then(|value| value.to_str())
                    == Some("impl");
            let is_apple_double = path
                .file_name()
                .and_then(|value| value.to_str())
                .is_some_and(|value| value.starts_with("._"));
            if is_seed && !is_apple_double {
                out.push(path);
            }
        }
        Ok(())
    }

    let mut files = Vec::new();
    visit(suite_root, &mut files)?;
    files.sort();
    Ok(files)
}

#[cfg(feature = "accounting")]
fn accounting_providers() -> Vec<Provider> {
    vec![
        Provider {
            id: ProviderId {
                suite: Suite::Accounting,
                service: "foundation",
            },
            generate_sql: || {
                rerp_entities::generate_sql_for_all().map_err(|error| error.to_string())
            },
        },
        Provider {
            id: ProviderId {
                suite: Suite::Accounting,
                service: "general-ledger",
            },
            generate_sql: || {
                rerp_accounting_general_ledger::models::entity_registry::generate_sql_for_all()
                    .map_err(|error| error.to_string())
            },
        },
        Provider {
            id: ProviderId {
                suite: Suite::Accounting,
                service: "invoice",
            },
            generate_sql: || {
                rerp_accounting_invoice::models::entity_registry::generate_sql_for_all()
                    .map_err(|error| error.to_string())
            },
        },
        Provider {
            id: ProviderId {
                suite: Suite::Accounting,
                service: "accounts-receivable",
            },
            generate_sql: || {
                rerp_accounting_accounts_receivable::models::entity_registry::generate_sql_for_all()
                    .map_err(|error| error.to_string())
            },
        },
        Provider {
            id: ProviderId {
                suite: Suite::Accounting,
                service: "accounts-payable",
            },
            generate_sql: || {
                rerp_accounting_accounts_payable::models::entity_registry::generate_sql_for_all()
                    .map_err(|error| error.to_string())
            },
        },
        Provider {
            id: ProviderId {
                suite: Suite::Accounting,
                service: "bank-sync",
            },
            generate_sql: || {
                rerp_accounting_bank_sync::models::entity_registry::generate_sql_for_all()
                    .map_err(|error| error.to_string())
            },
        },
        Provider {
            id: ProviderId {
                suite: Suite::Accounting,
                service: "asset",
            },
            generate_sql: || {
                rerp_accounting_asset::models::entity_registry::generate_sql_for_all()
                    .map_err(|error| error.to_string())
            },
        },
        Provider {
            id: ProviderId {
                suite: Suite::Accounting,
                service: "budget",
            },
            generate_sql: || {
                rerp_accounting_budget::models::entity_registry::generate_sql_for_all()
                    .map_err(|error| error.to_string())
            },
        },
        Provider {
            id: ProviderId {
                suite: Suite::Accounting,
                service: "edi",
            },
            generate_sql: || {
                rerp_accounting_edi::models::entity_registry::generate_sql_for_all()
                    .map_err(|error| error.to_string())
            },
        },
        Provider {
            id: ProviderId {
                suite: Suite::Accounting,
                service: "financial-reports",
            },
            generate_sql: || {
                rerp_accounting_financial_reports::models::entity_registry::generate_sql_for_all()
                    .map_err(|error| error.to_string())
            },
        },
    ]
}

#[cfg(feature = "documents")]
fn documents_providers() -> Vec<Provider> {
    vec![Provider {
        id: ProviderId {
            suite: Suite::Documents,
            service: "render",
        },
        generate_sql: || {
            rerp_documents_render::models::entity_registry::generate_sql_for_all()
                .map_err(|error| error.to_string())
        },
    }]
}

fn providers_for_suite(suite: Suite) -> Result<Vec<Provider>, String> {
    let providers = match suite {
        Suite::Accounting => {
            #[cfg(feature = "accounting")]
            {
                accounting_providers()
            }
            #[cfg(not(feature = "accounting"))]
            {
                return Err(
                    "the accounting provider is not compiled; rerun with --features accounting"
                        .to_string(),
                );
            }
        }
        Suite::Documents => {
            #[cfg(feature = "documents")]
            {
                documents_providers()
            }
            #[cfg(not(feature = "documents"))]
            {
                return Err(
                    "the documents provider is not compiled; rerun with --features documents"
                        .to_string(),
                );
            }
        }
    };
    for provider in &providers {
        if provider.id.suite != suite {
            return Err(format!(
                "provider '{}' was registered for the wrong selected suite '{}'",
                provider.id.display(),
                suite.directory_name()
            ));
        }
    }
    Ok(providers)
}

fn load_rows(providers: &[Provider]) -> Result<Vec<Row>, String> {
    let mut rows = Vec::new();
    for provider in providers {
        let sql_rows = (provider.generate_sql)().map_err(|error| {
            format!(
                "migration SQL generation failed for service '{}': {error}",
                provider.id.display()
            )
        })?;
        rows.extend(sql_rows.into_iter().map(|(table, sql)| Row {
            owner: provider.id,
            table,
            sql,
        }));
    }
    Ok(rows)
}

fn reject_duplicate_table_owners(rows: &[Row]) -> Result<(), String> {
    let mut owners: BTreeMap<&str, Vec<ProviderId>> = BTreeMap::new();
    for row in rows {
        owners
            .entry(row.table.as_str())
            .or_default()
            .push(row.owner);
    }
    let duplicates: Vec<String> = owners
        .into_iter()
        .filter_map(|(table, mut providers)| {
            if providers.len() < 2 {
                return None;
            }
            providers.sort_unstable();
            let definition_count = providers.len();
            providers.dedup();
            let names: Vec<String> = providers.into_iter().map(ProviderId::display).collect();
            let suffix = if names.len() == 1 {
                format!(" ({} definitions)", definition_count)
            } else {
                String::new()
            };
            Some(format!("{table}: {}{suffix}", names.join(", ")))
        })
        .collect();
    if duplicates.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "duplicate table ownership detected:\n  {}",
            duplicates.join("\n  ")
        ))
    }
}

fn validated_rows(suite: Suite) -> Result<Vec<Row>, String> {
    let providers = providers_for_suite(suite)?;
    let rows = load_rows(&providers)?;
    if rows.is_empty() {
        return Err(format!(
            "suite '{}' has no registered entity models",
            suite.directory_name()
        ));
    }
    reject_duplicate_table_owners(&rows)?;
    order_migrations_by_foreign_key_sql(
        rows.iter()
            .map(|row| (row.table.clone(), row.sql.clone()))
            .collect(),
    )
    .map_err(|error| format!("foreign-key ordering failed: {error}"))?;
    Ok(rows)
}

fn legacy_aggregate_baseline_is_identical(
    service_dir: &Path,
    service_name: &str,
    table_name: &str,
    sql: &str,
) -> Result<bool, String> {
    let per_table_files =
        find_existing_per_table_files(service_dir, table_name).map_err(|error| {
            format!("could not inspect migration history for {service_name}.{table_name}: {error}")
        })?;
    if !per_table_files.is_empty() {
        return Ok(false);
    }

    let baselines = accumulate_per_table_baselines_from_dir(service_dir).map_err(|error| {
        format!(
            "could not read aggregate migration history for {service_name}.{table_name}: {error}"
        )
    })?;
    let Some(parts) = baselines.get(table_name) else {
        return Ok(false);
    };
    let existing = combined_old_section(parts);
    if normalize_table_sql_blob(&existing) == normalize_table_sql_blob(sql) {
        Ok(true)
    } else {
        Err(format!(
            "{service_name}.{table_name} differs from its legacy aggregate baseline; add an explicit upgrade migration or convert that baseline to per-table history before generation"
        ))
    }
}

fn preflight_migration_history(migrations_root: &Path, rows: &[Row]) -> Result<(), String> {
    for row in rows {
        legacy_aggregate_baseline_is_identical(
            &migrations_root.join(row.owner.service),
            row.owner.service,
            &row.table,
            &row.sql,
        )?;
    }
    Ok(())
}

fn write_service_migrations(
    migrations_root: &Path,
    service_name: &str,
    sql_results: Vec<(String, String)>,
    run_timestamp: &str,
) -> Result<(), String> {
    let service_dir = migrations_root.join(service_name);
    std::fs::create_dir_all(&service_dir)
        .map_err(|error| format!("could not create {}: {error}", service_dir.display()))?;

    for (table_name, sql) in sql_results {
        // The delivered Accounting foundation predates the per-table writer and
        // uses one aggregate `*_generated_from_entities.sql` baseline. Lifeguard
        // can read that baseline but its per-table writer cannot safely produce
        // an additive delta from it. Skip an identical aggregate baseline and
        // fail closed on a change until it has an explicit upgrade migration.
        if legacy_aggregate_baseline_is_identical(&service_dir, service_name, &table_name, &sql)? {
            println!("skipped identical aggregate baseline: {service_name}.{table_name}");
            continue;
        }

        let header = MigrationHeader {
            migration_name: &table_name,
            generated_timestamp: run_timestamp,
        };
        match write_per_table_migration_file(
            &service_dir,
            &table_name,
            &sql,
            run_timestamp,
            Some(header),
        )
        .map_err(|error| {
            format!("failed to write migration for {service_name}.{table_name}: {error}")
        })? {
            EmissionOutcome::Initial { path } => {
                println!("generated initial SQL migration: {}", path.display());
            }
            EmissionOutcome::Delta { path } => {
                println!("generated additive SQL migration: {}", path.display());
            }
            EmissionOutcome::Skipped => {
                println!("skipped identical schema: {service_name}.{table_name}");
            }
        }
    }
    Ok(())
}

fn generate_suite(suite: Suite, manifest_dir: &Path) -> Result<(), String> {
    // Complete every provider and ownership check before the first filesystem write.
    let rows = validated_rows(suite)?;

    let table_to_owner: HashMap<String, ProviderId> = rows
        .iter()
        .map(|row| (row.table.clone(), row.owner))
        .collect();
    let ordered = order_migrations_by_foreign_key_sql(
        rows.iter()
            .map(|row| (row.table.clone(), row.sql.clone()))
            .collect(),
    )
    .map_err(|error| format!("foreign-key ordering failed: {error}"))?;

    let layout = suite_layout(manifest_dir, suite)?;
    // Detect every unsafe legacy-baseline transition before the first write so
    // one service cannot be partially generated before a later provider fails.
    preflight_migration_history(&layout.migrations_root, &rows)?;
    let run_timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    for (table, sql) in ordered {
        let owner = table_to_owner
            .get(&table)
            .ok_or_else(|| format!("ordered table has no owner: {table}"))?;
        write_service_migrations(
            &layout.migrations_root,
            owner.service,
            vec![(table, sql)],
            &run_timestamp,
        )?;
    }
    write_apply_order_file(&layout.migrations_root).map_err(|error| {
        format!(
            "could not write {}: {error}",
            layout.migrations_root.join("apply_order.txt").display()
        )
    })?;

    let seed_files = discover_seed_files(&layout.suite_root)?;
    if !seed_files.is_empty() {
        write_seed_order_file(
            &layout.migrations_root,
            &layout.suite_root,
            &seed_files,
            &layout.seed_order_path,
        )
        .map_err(|error| {
            format!(
                "could not write {}: {error}",
                layout.seed_order_path.display()
            )
        })?;
    }

    println!(
        "generated '{}' suite migrations under {}",
        suite.directory_name(),
        layout.migrations_root.display()
    );
    Ok(())
}

fn run() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Command::Validate {
            suite,
            migration_history,
        } => {
            let rows = validated_rows(suite)?;
            if migration_history {
                let layout = suite_layout(Path::new(env!("CARGO_MANIFEST_DIR")), suite)?;
                preflight_migration_history(&layout.migrations_root, &rows)?;
            }
            println!(
                "validated '{}' suite: {} tables with one owner each{}",
                suite.directory_name(),
                rows.len(),
                if migration_history {
                    " and safe migration history"
                } else {
                    ""
                }
            );
            Ok(())
        }
        Command::Generate { suite } => generate_suite(suite, Path::new(env!("CARGO_MANIFEST_DIR"))),
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_directory(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock before epoch")
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "rerp-migrator-{label}-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&directory).expect("create temp directory");
        directory
    }

    #[test]
    fn suite_layout_never_uses_repository_root_migrations() {
        let manifest = Path::new("/repo/microservices/migrator");
        let accounting = suite_layout(manifest, Suite::Accounting).expect("accounting layout");
        assert_eq!(
            accounting.migrations_root,
            Path::new("/repo/microservices/accounting/migrations")
        );
        assert_eq!(
            accounting.seed_order_path,
            Path::new("/repo/microservices/accounting/seed_order.txt")
        );
        assert_ne!(accounting.migrations_root, Path::new("/repo/migrations"));
    }

    #[test]
    fn duplicate_table_owners_are_reported_with_all_services() {
        let rows = vec![
            Row {
                owner: ProviderId {
                    suite: Suite::Accounting,
                    service: "invoice",
                },
                table: "accounting_accounts".to_string(),
                sql: "invoice sql".to_string(),
            },
            Row {
                owner: ProviderId {
                    suite: Suite::Accounting,
                    service: "foundation",
                },
                table: "accounting_accounts".to_string(),
                sql: "foundation sql".to_string(),
            },
        ];
        let error = reject_duplicate_table_owners(&rows).expect_err("must reject duplicates");
        assert!(error.contains("accounting_accounts: accounting/foundation, accounting/invoice"));
    }

    #[test]
    fn duplicate_definitions_inside_one_provider_are_rejected() {
        let owner = ProviderId {
            suite: Suite::Accounting,
            service: "invoice",
        };
        let rows = vec![
            Row {
                owner,
                table: "invoices".to_string(),
                sql: "first".to_string(),
            },
            Row {
                owner,
                table: "invoices".to_string(),
                sql: "second".to_string(),
            },
        ];
        let error = reject_duplicate_table_owners(&rows).expect_err("must reject duplicates");
        assert!(error.contains("invoices: accounting/invoice (2 definitions)"));
    }

    #[test]
    fn provider_errors_are_not_silently_omitted() {
        fn fail_generation() -> Result<SqlRows, String> {
            Err("fixture failure".to_string())
        }
        let provider = Provider {
            id: ProviderId {
                suite: Suite::Accounting,
                service: "invoice",
            },
            generate_sql: fail_generation,
        };
        let error = load_rows(&[provider]).expect_err("provider failure must propagate");
        assert!(error.contains("accounting/invoice"));
        assert!(error.contains("fixture failure"));
    }

    #[test]
    fn identical_legacy_aggregate_baseline_creates_no_drift() {
        let root = temporary_directory("aggregate-skip");
        let service = root.join("invoice");
        std::fs::create_dir_all(&service).expect("create service directory");
        std::fs::write(
            service.join("20260715000000_generated_from_entities.sql"),
            "-- Table: invoices\nCREATE TABLE IF NOT EXISTS invoices (\n    id UUID PRIMARY KEY\n);\n",
        )
        .expect("write aggregate baseline");

        write_service_migrations(
            &root,
            "invoice",
            vec![(
                "invoices".to_string(),
                "CREATE TABLE IF NOT EXISTS invoices (\n    id UUID PRIMARY KEY\n);".to_string(),
            )],
            "20260715000001",
        )
        .expect("identical baseline must skip");

        let sql_files: Vec<PathBuf> = std::fs::read_dir(&service)
            .expect("read service directory")
            .map(|entry| entry.expect("directory entry").path())
            .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("sql"))
            .collect();
        assert_eq!(sql_files.len(), 1);
        std::fs::remove_dir_all(root).expect("remove temp directory");
    }

    #[test]
    fn changed_legacy_aggregate_baseline_fails_without_writing() {
        let root = temporary_directory("aggregate-change");
        let service = root.join("invoice");
        std::fs::create_dir_all(&service).expect("create service directory");
        std::fs::write(
            service.join("20260715000000_generated_from_entities.sql"),
            "-- Table: invoices\nCREATE TABLE IF NOT EXISTS invoices (id UUID PRIMARY KEY);\n",
        )
        .expect("write aggregate baseline");

        let error = write_service_migrations(
            &root,
            "invoice",
            vec![(
                "invoices".to_string(),
                "CREATE TABLE IF NOT EXISTS invoices (id UUID PRIMARY KEY, number TEXT);"
                    .to_string(),
            )],
            "20260715000001",
        )
        .expect_err("changed aggregate baseline must fail closed");
        assert!(error.contains("explicit upgrade migration"));

        let sql_files: Vec<PathBuf> = std::fs::read_dir(&service)
            .expect("read service directory")
            .map(|entry| entry.expect("directory entry").path())
            .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("sql"))
            .collect();
        assert_eq!(sql_files.len(), 1);
        std::fs::remove_dir_all(root).expect("remove temp directory");
    }

    #[test]
    fn seed_discovery_is_recursive_and_suite_scoped() {
        let root = temporary_directory("seeds");
        let expected = root.join("invoice/impl/seeds/001_invoice.sql");
        let nested = root.join("nested/group/bank-sync/impl/seeds/002_bank.sql");
        let ignored = root.join("invoice/gen/seeds/ignored.sql");
        for file in [&expected, &nested, &ignored] {
            std::fs::create_dir_all(file.parent().expect("parent")).expect("create parent");
            std::fs::write(file, "SELECT 1;").expect("write fixture");
        }

        let discovered = discover_seed_files(&root).expect("discover seeds");
        assert_eq!(discovered, vec![expected, nested]);
        std::fs::remove_dir_all(root).expect("remove temp directory");
    }

    #[test]
    fn missing_suite_feature_fails_closed() {
        #[cfg(not(feature = "documents"))]
        assert_eq!(
            providers_for_suite(Suite::Documents).expect_err("documents must be unavailable"),
            "the documents provider is not compiled; rerun with --features documents"
        );
    }

    #[cfg(feature = "accounting")]
    #[test]
    fn accounting_has_one_authoritative_ledger_schema() {
        let rows = load_rows(&accounting_providers()).expect("load Accounting providers");
        let tables: std::collections::BTreeSet<&str> =
            rows.iter().map(|row| row.table.as_str()).collect();

        for required in [
            "accounting_accounts",
            "accounting_journal_entries",
            "accounting_journal_lines",
        ] {
            assert!(
                tables.contains(required),
                "missing foundation table {required}"
            );
        }

        for retired in [
            "accounts",
            "account_balances",
            "chart_of_accounts",
            "journal_entries",
            "journal_entry_lines",
        ] {
            assert!(
                !tables.contains(retired),
                "parallel General Ledger table {retired} must not return"
            );
        }
    }
}
