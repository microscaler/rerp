/**
 * Formats a percentage string for brochure sites.
 * Converts large numbers to readable format (e.g., "+49,900%" -> "+49.9K%")
 * Rounds to one decimal place for large numbers.
 */
export function formatPercentageForBrochure(percentage: string): string {
  // Extract the sign and number
  const sign = percentage.startsWith('-') ? '-' : percentage.startsWith('+') ? '+' : '';
  const numberStr = percentage.replace(/[+\-%]/g, '').replace(/,/g, '');
  
  // Parse the number
  const num = parseFloat(numberStr);
  
  // If it's not a valid number, return as-is
  if (isNaN(num)) {
    return percentage;
  }
  
  // For numbers >= 1000, format with K suffix and one decimal place
  if (Math.abs(num) >= 1000) {
    const formatted = (Math.abs(num) / 1000).toFixed(1);
    // Remove trailing zero if it's a whole number
    const cleanFormatted = formatted.endsWith('.0') ? formatted.slice(0, -2) : formatted;
    return `${sign}${cleanFormatted}K%`;
  }
  
  // For numbers < 1000, return as-is (they're already readable)
  return percentage;
}

