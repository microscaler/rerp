"""Fix impl controllers to use Decimal instead of f64 literals."""

import re
import sys
from pathlib import Path
from re import Match


def convert_f64_to_decimal(match: Match[str]) -> str:
    """Convert f64 literal to Decimal::new() call."""
    value = match.group(1)

    # Parse the float value
    try:
        float_val = float(value)
    except ValueError:
        return match.group(0)  # Return unchanged if not a valid float

    # Convert to Decimal::new(mantissa, scale)
    # Example: 3.14 -> Decimal::new(314, 2)
    # Example: 5000000.0 -> Decimal::new(50000000, 1)

    # Handle integer values (e.g., 5000000.0)
    if float_val.is_integer():
        int_val = int(float_val)
        # For large integers, use scale 0
        return f"rust_decimal::Decimal::new({int_val}, 0)"

    # Handle decimal values
    # Convert to string to preserve precision
    str_val = str(float_val)
    if "." in str_val:
        # Remove decimal point and count digits after decimal
        parts = str_val.split(".")
        integer_part = parts[0]
        decimal_part = parts[1].rstrip("0")  # Remove trailing zeros
        if not decimal_part:
            # It was something like 5000000.0
            return f"rust_decimal::Decimal::new({int(float_val)}, 0)"

        # Reconstruct as integer with scale
        mantissa_str = integer_part + decimal_part
        scale = len(decimal_part)
        mantissa = int(mantissa_str)

        # Handle negative numbers
        if mantissa < 0:
            return f"rust_decimal::Decimal::new({mantissa}, {scale})"
        return f"rust_decimal::Decimal::new({mantissa}, {scale})"

    return match.group(0)


def fix_impl_controller(file_path: Path) -> tuple[int, bool]:
    """Fix f64 literals in impl controller file.

    Returns (number of fixes, whether file was changed).
    """
    try:
        content = file_path.read_text()
    except (OSError, UnicodeDecodeError) as e:
        print(f"❌ Failed to read {file_path}: {e}")
        return (0, False)

    original_content = content

    # Pattern to match f64 literals in Some() calls
    # Matches: Some(3.14), Some(5000000.0), Some(-5.0), etc.
    # But avoid matching if it's already Decimal::new()
    pattern = r"Some\((-?\d+\.?\d*)\)"

    def replace_func(match: Match[str]) -> str:
        value = match.group(1)
        # Check if this looks like a decimal value (has decimal point or is a large number)
        try:
            float_val = float(value)
            # Only convert if it's a decimal number (has fractional part) or is a money amount
            if "." in value or abs(float_val) >= 1000:
                decimal_expr = convert_f64_to_decimal(match)
                return f"Some({decimal_expr})"
        except ValueError:
            pass
        return match.group(0)

    # Replace f64 literals in Some() calls
    content = re.sub(pattern, replace_func, content)

    # Also handle direct assignments (not in Some())
    # Pattern: field: 3.14, or field: 5000000.0,
    pattern2 = r":\s+(\d+\.\d+),"

    def replace_func2(match: Match[str]) -> str:
        value = match.group(1)
        try:
            float(value)
            if "." in value:
                decimal_expr = convert_f64_to_decimal(match)
                return f": {decimal_expr},"
        except ValueError:
            pass
        return match.group(0)

    content = re.sub(pattern2, replace_func2, content)

    if content != original_content:
        file_path.write_text(content)
        fixes = len(re.findall(r"rust_decimal::Decimal::new", content)) - len(
            re.findall(r"rust_decimal::Decimal::new", original_content)
        )
        return (fixes, True)

    return (0, False)


def main() -> int:
    """Fix all impl controllers in microservices/accounting."""
    project_root = Path(__file__).parent.parent.parent.parent.parent
    accounting_impl = project_root / "microservices" / "accounting"

    if not accounting_impl.exists():
        print(f"❌ Accounting microservices directory not found: {accounting_impl}")
        return 1

    total_fixes = 0
    files_fixed = []

    # Find all impl controller files
    for controller_file in sorted(accounting_impl.rglob("impl/src/controllers/*.rs")):
        fixes, changed = fix_impl_controller(controller_file)
        if changed:
            files_fixed.append((controller_file, fixes))
            total_fixes += fixes

    if files_fixed:
        print(f"✅ Fixed {total_fixes} f64 literals in {len(files_fixed)} file(s):\n")
        for file_path, fixes in files_fixed:
            rel_path = file_path.relative_to(project_root)
            print(f"  {rel_path}: {fixes} fix(es)")
        return 0
    print("✅ No f64 literals found in impl controllers (or already fixed)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
