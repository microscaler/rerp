//! The customer schema, applied by the service itself at startup (idempotent).
//!
//! The service's database login acts as the database's owner group (the platform's per-service logins switch to it at
//! login), so it may create its own tables. Two scripts, compiled in so they cannot drift from the source tree:
//! the Sesame RLS contract (vendored for accounting) and crm/migrations. Statements run one by one; every one of them
//! is idempotent, so a crash part-way is finished by the next start.

use lifeguard::LifeExecutor;
use sea_query::Values;

const RLS_CONTRACT: &str = include_str!("../../../../accounting/sql/rls/v1/install.sql");
const MIGRATIONS: &[(&str, &str)] = &[
    (
        "0001_accounts",
        include_str!("../../../migrations/0001_accounts.sql"),
    ),
    (
        "0002_accounts_phone",
        include_str!("../../../migrations/0002_accounts_phone.sql"),
    ),
];

/// Split a script into statements: semicolons end statements except inside dollar-quoted bodies, quotes and comments.
/// Transaction control is dropped (each statement commits on its own).
pub fn statements(script: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut chars = script.chars().peekable();
    let mut dollar: Option<String> = None;
    let mut in_quote = false;
    while let Some(c) = chars.next() {
        if let Some(tag) = &dollar {
            current.push(c);
            if c == '$' && current.ends_with(tag.as_str()) {
                dollar = None;
            }
            continue;
        }
        if in_quote {
            current.push(c);
            if c == '\'' {
                in_quote = false;
            }
            continue;
        }
        match c {
            '-' if chars.peek() == Some(&'-') => {
                for n in chars.by_ref() {
                    if n == '\n' {
                        current.push('\n');
                        break;
                    }
                }
            }
            '\'' => {
                in_quote = true;
                current.push(c);
            }
            '$' => {
                let mut tag = String::from("$");
                while let Some(&n) = chars.peek() {
                    if n.is_ascii_alphanumeric() || n == '_' {
                        tag.push(n);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if chars.peek() == Some(&'$') {
                    chars.next();
                    tag.push('$');
                    current.push_str(&tag);
                    dollar = Some(tag);
                } else {
                    current.push_str(&tag);
                }
            }
            ';' => {
                let stmt = current.trim();
                let upper = stmt.to_ascii_uppercase();
                if !stmt.is_empty() && upper != "BEGIN" && upper != "COMMIT" {
                    out.push(stmt.to_string());
                }
                current.clear();
            }
            _ => current.push(c),
        }
    }
    if !current.trim().is_empty() {
        out.push(current.trim().to_string());
    }
    out
}

/// Apply the RLS contract and every migration. Errors name the script and statement.
pub fn ensure<E: LifeExecutor>(executor: &E) -> Result<usize, String> {
    let mut applied = 0;
    for (name, script) in
        std::iter::once(("sesame-rls-v1", RLS_CONTRACT)).chain(MIGRATIONS.iter().copied())
    {
        for (i, stmt) in statements(script).iter().enumerate() {
            executor
                .execute_values(stmt, &Values(Vec::new()))
                .map_err(|e| format!("{name} statement {}: {e}", i + 1))?;
            applied += 1;
        }
    }
    Ok(applied)
}

/// At startup: apply the schema when a database is configured (DB_HOST set). Failure is logged, not fatal: reads answer
/// 503 and the intake retries until the database is right.
pub fn ensure_at_startup() {
    if std::env::var("DB_HOST")
        .map(|h| h.trim().is_empty())
        .unwrap_or(true)
    {
        println!("[schema] DB_HOST not set; customer schema not applied");
        return;
    }
    match crate::database::db()
        .map_err(|e| e.to_string())
        .and_then(ensure)
    {
        Ok(n) => println!("[schema] customer schema ensured ({n} statements)"),
        Err(e) => eprintln!("[schema][error] customer schema not applied: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_on_semicolons_outside_bodies_quotes_and_comments() {
        let sql = "BEGIN;\n-- a comment; with a semicolon\nCREATE TABLE t (x text DEFAULT 'a;b');\n\
                   CREATE FUNCTION f() RETURNS void LANGUAGE plpgsql AS $$\nBEGIN\n  PERFORM 1;\nEND;\n$$;\n\
                   CREATE FUNCTION g() RETURNS int LANGUAGE sql AS $body$ SELECT 1; $body$;\nCOMMIT;\n";
        let s = statements(sql);
        assert_eq!(s.len(), 3, "{s:#?}");
        assert!(s[0].starts_with("CREATE TABLE") && s[0].contains("'a;b'"));
        assert!(s[1].contains("PERFORM 1;") && s[1].ends_with("$$"));
        assert!(s[2].contains("SELECT 1;"));
    }

    #[test]
    fn the_shipped_scripts_split_cleanly() {
        let rls = statements(RLS_CONTRACT);
        assert!(rls
            .iter()
            .any(|s| s.contains("FUNCTION public.rls_set_session")));
        assert!(rls
            .iter()
            .all(|s| s.matches("$$").count().is_multiple_of(2)));
        let crm = statements(MIGRATIONS[0].1);
        assert!(crm
            .iter()
            .any(|s| s.starts_with("CREATE TABLE IF NOT EXISTS crm_accounts")));
        assert!(crm
            .iter()
            .any(|s| s.starts_with("CREATE POLICY crm_conversions_tenant")));
        assert!(!crm.iter().any(|s| s == "BEGIN" || s == "COMMIT"));
    }
}
