//! Deterministic, dependency-light PDF rendering for immutable Phase 1 invoices.
//!
//! Rendering deliberately uses accounting snapshots only. No mutable customer or
//! organisation lookup can change a document after it has been issued.

use crate::posting::StoredInvoice;
use rerp_accounting_core::AccountingDocumentType;
use std::fmt::Write as _;

const LINES_PER_PAGE: usize = 48;
const RENDERER: &str = "rerp-basic-pdf";
pub const RENDERER_VERSION: &str = "1";

pub fn renderer_name() -> &'static str {
    RENDERER
}

pub fn render(invoice: &StoredInvoice) -> Vec<u8> {
    let snapshot = &invoice.snapshot;
    let kind = match snapshot.document_type {
        AccountingDocumentType::CustomerInvoice => "CUSTOMER INVOICE",
        AccountingDocumentType::CustomerCreditNote => "CUSTOMER CREDIT NOTE",
    };
    let mut lines = vec![
        "RERP ACCOUNTING".to_string(),
        kind.to_string(),
        format!("Document: {}", snapshot.document_number),
        format!(
            "Invoice date: {}    Due date: {}",
            snapshot.invoice_date, snapshot.due_date
        ),
        format!("Customer: {}", snapshot.customer_id),
        format!("Currency: {}", snapshot.currency.as_str()),
        String::new(),
        "Line  Description                              Qty        Unit       Tax       Total"
            .to_string(),
        "----  ------------------------------------  --------  ----------  --------  ----------"
            .to_string(),
    ];
    for line in &snapshot.lines {
        lines.push(format!(
            "{:>4}  {:<36}  {:>8}  {:>10}  {:>8}  {:>10}",
            line.line_number,
            printable(&line.description, 36),
            line.quantity,
            line.unit_price,
            line.tax_amount,
            line.total_amount,
        ));
    }
    lines.extend([
        String::new(),
        format!(
            "Subtotal: {} {}",
            snapshot.currency.as_str(),
            snapshot.subtotal
        ),
        format!(
            "Discount: {} {}",
            snapshot.currency.as_str(),
            snapshot.discount_amount
        ),
        format!(
            "Tax:      {} {}",
            snapshot.currency.as_str(),
            snapshot.tax_amount
        ),
        format!(
            "TOTAL:    {} {}",
            snapshot.currency.as_str(),
            snapshot.total_amount
        ),
        String::new(),
        format!("Posted: {} UTC", snapshot.posted_at),
        format!(
            "Source: {}/{}/{}",
            snapshot.source.system, snapshot.source.resource_type, snapshot.source.resource_id
        ),
    ]);
    if let Some(original) = snapshot.original_document_id {
        lines.push(format!("Original document: {original}"));
    }
    let pages = lines
        .chunks(LINES_PER_PAGE)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();
    build_pdf(&pages)
}

fn printable(value: &str, max: usize) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_graphic() || character == ' ' {
                character
            } else {
                '?'
            }
        })
        .take(max)
        .collect()
}

fn pdf_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'(' | b')' | b'\\' => {
                escaped.push('\\');
                escaped.push(char::from(byte));
            }
            0x20..=0x7e => escaped.push(char::from(byte)),
            _ => escaped.push('?'),
        }
    }
    escaped
}

fn page_stream(lines: &[String], page: usize, total_pages: usize) -> String {
    let mut stream = String::from("BT\n/F1 10 Tf\n12 TL\n50 795 Td\n");
    for line in lines {
        let _ = writeln!(stream, "({}) Tj", pdf_string(line));
        stream.push_str("T*\n");
    }
    stream.push_str("T*\nT*\n");
    let _ = writeln!(stream, "(Page {page} of {total_pages}) Tj");
    stream.push_str("ET\n");
    stream
}

fn build_pdf(pages: &[Vec<String>]) -> Vec<u8> {
    let page_count = pages.len().max(1);
    let object_count = 3 + page_count * 2;
    let mut objects = vec![String::new(); object_count + 1];
    objects[1] = "<< /Type /Catalog /Pages 2 0 R >>".to_string();
    let kids = (0..page_count)
        .map(|index| format!("{} 0 R", 4 + index * 2))
        .collect::<Vec<_>>()
        .join(" ");
    objects[2] = format!("<< /Type /Pages /Count {page_count} /Kids [{kids}] >>");
    objects[3] =
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>"
            .to_string();
    for index in 0..page_count {
        let page_id = 4 + index * 2;
        let content_id = page_id + 1;
        let stream = page_stream(
            pages.get(index).map(Vec::as_slice).unwrap_or(&[]),
            index + 1,
            page_count,
        );
        objects[page_id] = format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 595 842] /Resources << /Font << /F1 3 0 R >> >> /Contents {content_id} 0 R >>"
        );
        objects[content_id] = format!(
            "<< /Length {} >>\nstream\n{}endstream",
            stream.len(),
            stream
        );
    }

    let mut output = b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n".to_vec();
    let mut offsets = vec![0usize; object_count + 1];
    for id in 1..=object_count {
        offsets[id] = output.len();
        output.extend_from_slice(format!("{id} 0 obj\n{}\nendobj\n", objects[id]).as_bytes());
    }
    let xref = output.len();
    output.extend_from_slice(
        format!("xref\n0 {}\n0000000000 65535 f \n", object_count + 1).as_bytes(),
    );
    for offset in offsets.iter().skip(1) {
        output.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    output.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n",
            object_count + 1
        )
        .as_bytes(),
    );
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_pdf_literal_strings() {
        assert_eq!(pdf_string("(a)\\b"), "\\(a\\)\\\\b");
        assert_eq!(pdf_string("café"), "caf??");
    }

    #[test]
    fn builds_deterministic_validly_framed_pdf() {
        let pages = vec![vec!["Invoice INV-1".to_string()]];
        let first = build_pdf(&pages);
        let second = build_pdf(&pages);
        assert_eq!(first, second);
        assert!(first.starts_with(b"%PDF-1.4"));
        assert!(first.windows(4).any(|window| window == b"xref"));
        assert!(first.ends_with(b"%%EOF\n"));
    }
}
