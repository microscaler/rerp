# RERP License Options Analysis

## Requirements

1. ✅ **Source code visible** - Others can see and read the source code
2. ❌ **No selling derivations** - Others cannot sell products based on RERP
3. 🔒 **SaaS exclusivity** - Only RERP can offer RERP as a SaaS service
4. 📦 **Future enterprise modules** - Some modules may be closed source later
5. ⏰ **Permanent restrictions** - No time-limited licenses (rules out BSL 1.1)

## License Comparison Table

| License | Type | Source Visible | Commercial Use | Sell Derivations | SaaS Restrictions | Time Limit | Auto-Convert | Enterprise Modules | Legal Precedent | Complexity | Notes |
|---------|------|---------------|----------------|------------------|-------------------|------------|-------------|-------------------|-----------------|------------|-------|
| **Business Source License (BSL) 1.1** | Source-Available | ✅ Yes | ⚠️ Restricted during term | ❌ No (during term) | ✅ Prohibits competing SaaS | ✅ Yes (3-4 years typical) | ✅ Yes (to Apache/GPL) | ✅ Compatible | ✅ High (HashiCorp, CockroachDB, MariaDB) | Medium | ❌ **RULED OUT** - Time limit |
| **Elastic License 2.0 (ELv2)** | Source-Available | ✅ Yes | ✅ Yes (with restrictions) | ⚠️ Allowed (except SaaS) | ✅ Prohibits managed service | ❌ No | ❌ No | ✅ Compatible | ✅ High (Elastic) | Low | ⚠️ Allows selling (not SaaS) |
| **Server Side Public License (SSPL)** | Source-Available | ✅ Yes | ✅ Yes | ⚠️ Allowed (with source disclosure) | ⚠️ Requires source disclosure | ❌ No | ❌ No | ✅ Compatible | ✅ High (MongoDB) | High | ⚠️ Allows SaaS if source disclosed |
| **PolyForm Shield 1.0.0** | Source-Available | ✅ Yes | ✅ Yes (non-competing) | ❌ No (if competing) | ✅ Prohibits competing services | ❌ No | ❌ No | ✅ Compatible | ⚠️ Medium (newer license) | Medium | ⭐ **Permanent** |
| **PolyForm Perimeter 1.0.0** | Source-Available | ✅ Yes | ✅ Yes (non-competing) | ❌ No (if competing with software) | ⚠️ Prohibits competing with software | ❌ No | ❌ No | ✅ Compatible | ⚠️ Medium (newer license) | Medium | ⭐ **Permanent** |
| **AGPL v3.0** | Open Source (Copyleft) | ✅ Yes | ✅ Yes | ✅ Yes (with source sharing) | ⚠️ Requires source disclosure | ❌ No | ❌ No | ⚠️ Complex (copyleft applies) | ✅ High (many projects) | Medium | ❌ Allows selling & SaaS |
| **Commons Clause + Apache 2.0** | Source-Available | ✅ Yes | ⚠️ Restricted | ❌ No selling | ⚠️ No selling SaaS | ❌ No | ❌ No | ✅ Compatible | ⚠️ Medium (Redis Labs) | Low | ⭐ **Permanent** |
| **Custom License** | Proprietary | ✅ Yes | ⚠️ As defined | ⚠️ As defined | ✅ As defined | ⚠️ As defined | ⚠️ As defined | ✅ Compatible | ❌ Low (untested) | High | ⭐ **Permanent** |

## Detailed Analysis

### 1. Business Source License (BSL) 1.1 ❌ **RULED OUT**

**Status**: ❌ **Excluded** - Has time limit before conversion to open source (does not meet permanent restriction requirement).

**Note**: While BSL 1.1 meets most requirements during its restricted term, it automatically converts to open source after 3-4 years, which rules it out for permanent protection needs.

---

### 2. Elastic License 2.0 (ELv2) ⚠️ **PARTIAL MATCH**

**Best For**: Permanent source-available licensing with minimal restrictions.

**Key Features**:
- ✅ Source code always visible
- ✅ Allows commercial use, modification, distribution
- ✅ Only 3 restrictions:
  1. Cannot offer as managed/hosted service
  2. Cannot circumvent license keys
  3. Cannot remove copyright notices
- ✅ No time limit (permanent)
- ✅ Simple, easy to understand

**Restrictions**:
- ❌ Cannot provide software as a managed/hosted service to third parties
- ✅ Can use internally
- ✅ Can sell products that include RERP
- ✅ Can modify and distribute

**Pros**:
- Very simple and clear
- Strong legal precedent (Elastic)
- Allows most commercial uses except SaaS
- No expiration of restrictions

**Cons**:
- **❌ Allows selling derivations** (if not SaaS) - **Does not meet requirement #2**
- Not OSI-approved
- May allow competitors to build competing products (just not as SaaS)

**Verdict**: ⚠️ **Partial match** - Prevents SaaS but allows selling derivations (does not fully meet requirement #2).

**Example Usage**: Elasticsearch, Kibana (dual licensed with SSPL)

---

### 3. Server Side Public License (SSPL) ⚠️ **PARTIAL MATCH**

**Best For**: Strong copyleft with SaaS source disclosure requirements.

**Key Features**:
- ✅ Source code always visible
- ✅ Based on GPL/AGPL
- ✅ Requires source disclosure if offering as a service
- ✅ Strong copyleft provisions

**Restrictions**:
- ⚠️ If offering as a service, must release all source code (including management layers)
- ✅ Can use, modify, distribute otherwise
- ✅ Can sell products

**Pros**:
- Strong protection for SaaS offerings
- Based on well-tested GPL framework
- Requires competitors to open source their service infrastructure

**Cons**:
- Very complex (requires legal review)
- Not OSI-approved
- May be too restrictive for some users
- **⚠️ Competitors can still offer SaaS** if they open source everything (does not fully meet requirement #3)
- **⚠️ Allows selling derivations** (does not fully meet requirement #2)

**Verdict**: ⚠️ **Partial match** - Requires source disclosure but doesn't prevent SaaS or selling.

**Example Usage**: MongoDB

---

### 4. PolyForm Shield 1.0.0 ⭐ **STRONG CANDIDATE**

**Best For**: Permanent protection against competition with your business ecosystem.

**Key Features**:
- ✅ Source code always visible
- ✅ Allows commercial and non-commercial use
- ✅ **Prohibits use that competes with licensor or affiliates** (broad protection)
- ✅ Clear, readable license text
- ✅ **No time limit** (permanent restrictions)

**Restrictions**:
- ❌ Cannot use in ways that compete with RERP or RERP's affiliates
- ❌ **Prevents selling competing products** (should cover requirement #2)
- ❌ **Prevents competing SaaS** (should cover requirement #3)
- ✅ Can use for non-competing purposes
- ✅ Can modify and distribute

**Pros**:
- ✅ **Permanent protection** (no time limit)
- ✅ **Prevents competition** (should cover both selling and SaaS)
- Clear, readable license
- Protects your entire business ecosystem
- Allows most uses except direct competition

**Cons**:
- ⚠️ Newer license (less legal precedent than BSL/ELv2)
- "Competition" definition may be ambiguous (requires clear definition)
- May allow indirect competition (depends on interpretation)

**Verdict**: ⭐ **Strong candidate** - Permanent restrictions that should prevent both selling derivations and competing SaaS.

**Example Usage**: Various startups and companies

---

### 5. PolyForm Perimeter 1.0.0 ⭐ **CANDIDATE**

**Best For**: Permanent protection against competition with the specific software (narrower than Shield).

**Key Features**:
- ✅ Source code always visible
- ✅ Allows commercial and non-commercial use
- ✅ **Prohibits use that competes with the software itself**
- ✅ **No time limit** (permanent restrictions)
- ⚠️ Narrower protection than Shield (only protects the software, not your business)

**Restrictions**:
- ❌ Cannot use in ways that compete with RERP software
- ❌ **Prevents selling competing products** (should cover requirement #2)
- ❌ **Prevents competing SaaS** (should cover requirement #3)
- ✅ Can use for non-competing purposes
- ✅ Can modify and distribute

**Pros**:
- ✅ **Permanent protection** (no time limit)
- ✅ **Prevents competition with software** (should cover both selling and SaaS)
- Clear, readable license
- Narrower scope (less restrictive than Shield)

**Cons**:
- ⚠️ Newer license (less legal precedent)
- ⚠️ Narrower than Shield (only protects software, not your business ecosystem)
- May allow indirect competition
- Less protection than Shield

**Verdict**: ⭐ **Candidate** - Permanent restrictions, but narrower scope than Shield.

---

### 6. AGPL v3.0

**Best For**: Maximum source code sharing (true open source).

**Key Features**:
- ✅ True open source (OSI-approved)
- ✅ Source code always visible
- ✅ Requires source disclosure for network services
- ✅ Strong copyleft

**Restrictions**:
- ⚠️ Must share source code if offering as a network service
- ✅ Can use, modify, distribute
- ✅ Can sell products

**Pros**:
- True open source license
- Strong community acceptance
- Well-tested legally

**Cons**:
- **Does NOT prevent selling derivations**
- **Does NOT prevent SaaS** (only requires source disclosure)
- May not meet your requirements

**Verdict**: ❌ **Does not meet requirements** - allows selling and SaaS

---

### 7. Commons Clause + Apache 2.0 ⭐ **CANDIDATE**

**Best For**: Adding "no selling" restriction to Apache 2.0 (permanent).

**Key Features**:
- ✅ Source code always visible
- ✅ Apache 2.0 base (well-understood)
- ✅ **Adds "no selling" restriction**
- ✅ **No time limit** (permanent restrictions)

**Restrictions**:
- ❌ **Cannot sell the software or services** (should cover requirement #2)
- ❌ **Cannot sell SaaS** (should cover requirement #3)
- ✅ Can use, modify, view source
- ✅ Can distribute (but not sell)

**Pros**:
- ✅ **Permanent "no selling" restriction**
- Simple addition to Apache 2.0
- Clear "no selling" restriction
- Well-understood base license

**Cons**:
- ⚠️ Less legal precedent (Redis Labs moved away from it)
- "Selling" definition may be ambiguous (requires clear definition)
- May not clearly prevent all forms of SaaS (depends on interpretation)

**Verdict**: ⭐ **Candidate** - Permanent "no selling" restriction, but less tested legally.

**Example Usage**: Redis Labs (later moved to different licenses)

---

### 8. Custom License ⭐ **CANDIDATE**

**Best For**: Maximum control and specificity with permanent restrictions.

**Key Features**:
- ✅ **Complete control over terms**
- ✅ **Can specify exact restrictions** (perfect for your requirements)
- ✅ **Can tailor to your needs**
- ✅ **Permanent restrictions** (no time limit)

**Restrictions**:
- ✅ **As you define** - Can explicitly prohibit:
  - Selling derivations
  - Offering as SaaS
  - Any other restrictions you need

**Pros**:
- ✅ **Perfect fit for your requirements**
- ✅ **Can be very specific** (no ambiguity)
- ✅ **Permanent protection**
- Complete control

**Cons**:
- ❌ No legal precedent (untested)
- ⚠️ Requires legal review (higher costs)
- ⚠️ May deter contributors/users (less familiar)
- ⚠️ Higher legal costs

**Verdict**: ⭐ **Candidate** - Maximum control but requires legal expertise and has no precedent.

---

## Recommendation Matrix (Permanent Licenses Only)

### If you want **permanent protection against competition** (selling + SaaS):
→ **PolyForm Shield 1.0.0** ⭐⭐⭐ **TOP RECOMMENDATION**
- Prevents competition with your business ecosystem
- Should cover both selling derivations and SaaS
- Clear, readable license
- Permanent restrictions

### If you want **permanent "no selling" restriction** (simple):
→ **Commons Clause + Apache 2.0** ⭐⭐
- Simple addition to well-understood Apache 2.0
- Clear "no selling" restriction
- Permanent restrictions
- Less legal precedent

### If you want **maximum control** (custom terms):
→ **Custom License** ⭐
- Perfect fit for exact requirements
- Can specify all restrictions explicitly
- Requires legal review
- No legal precedent

### ⚠️ **Partial Matches** (do not fully meet requirements):
- **Elastic License 2.0 (ELv2)**: Prevents SaaS but allows selling derivations
- **SSPL**: Requires source disclosure but doesn't prevent SaaS or selling
- **PolyForm Perimeter**: Similar to Shield but narrower scope

---

## Key Considerations for RERP

### Your Requirements:
1. ✅ Source visible - **All options meet this**
2. ❌ No selling derivations - **PolyForm Shield/Perimeter, Commons Clause, Custom**
3. 🔒 SaaS exclusivity - **PolyForm Shield/Perimeter, Commons Clause, Custom**
4. 📦 Enterprise modules - **All compatible** (separate licensing)
5. ⏰ Permanent restrictions - **All remaining options meet this** (BSL ruled out)

### Recommended Approach (Permanent Licenses):

**Option A: PolyForm Shield 1.0.0** ⭐⭐⭐ **TOP RECOMMENDATION**
- ✅ **Permanent restrictions** (no time limit)
- ✅ **Prevents competition** with your business ecosystem
- ✅ Should cover both selling derivations and SaaS
- ✅ Clear, readable license text
- ⚠️ Newer license (less legal precedent than BSL/ELv2)
- ⚠️ "Competition" definition may need clarification

**Option B: Commons Clause + Apache 2.0** ⭐⭐
- ✅ **Permanent "no selling" restriction**
- ✅ Simple addition to well-understood Apache 2.0
- ✅ Should prevent both selling and SaaS
- ⚠️ Less legal precedent (Redis Labs moved away)
- ⚠️ "Selling" definition may need clarification

**Option C: Custom License** ⭐
- ✅ **Perfect fit** for exact requirements
- ✅ Can explicitly define all restrictions
- ✅ No ambiguity
- ❌ Requires legal review (higher costs)
- ❌ No legal precedent
- ❌ May deter contributors/users

---

## Implementation Notes

### For Enterprise Modules:
- Use **proprietary/commercial license** for enterprise modules
- Keep in separate repository or organization
- Can be dual-licensed (open core model)

### For Community Edition:
- Use chosen source-available license (BSL, ELv2, etc.)
- Keep in main repository
- Source visible, but with commercial restrictions

### Dual Licensing Strategy:
- **Community Edition**: PolyForm Shield/Commons Clause (source-available, permanent restrictions)
- **Enterprise Edition**: Proprietary (closed source, commercial license)
- **SaaS**: Only you can offer (protected by license)

---

## Next Steps

1. **Review legal counsel** - Have a lawyer review your chosen license (especially for PolyForm or Custom)
2. **Clarify definitions** - Define "competition" (PolyForm) or "selling" (Commons Clause) clearly
3. **Update LICENSE file** - Replace Apache 2.0 with chosen license
4. **Add license headers** - Update file headers if needed
5. **Document license choice** - Add to README explaining the license and restrictions

---

## References

- [Business Source License 1.1](https://mariadb.com/bsl11/)
- [Elastic License 2.0](https://www.elastic.co/licensing/elastic-license)
- [PolyForm Project](https://polyformproject.org/)
- [Server Side Public License](https://www.mongodb.com/licensing/server-side-public-license)
- [Commons Clause](https://commonsclause.com/)

---

**Last Updated**: 2026-01-23  
**Status**: Analysis Complete - BSL 1.1 Ruled Out (Time Limit)  
**Top Recommendation**: PolyForm Shield 1.0.0 (Permanent Protection)
