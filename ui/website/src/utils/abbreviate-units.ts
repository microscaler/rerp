/**
 * Abbreviates units in text for brochure sites.
 * - "month" → "mth"
 * - "minutes" → "mins"
 * - "hours" → "hrs"
 */
export function abbreviateUnits(text: string): string {
  return text
    .replace(/\bmonth\b/gi, 'mth')
    .replace(/\bminutes\b/gi, 'mins')
    .replace(/\bminute\b/gi, 'min')
    .replace(/\bhours\b/gi, 'hrs')
    .replace(/\bhour\b/gi, 'hr');
}

