# zynko-oracle · `rust-armstrong-numbers-is-armstrong-number-conformance-oracle`

**A deterministic, re-checkable conformance oracle for `armstrong-numbers` (rust).**

## Proven
Measured on the canonical Exercism corpus — **5 input/output pairs, 2 distinct outputs** — produced by *running* the reference in a sealed sandbox, not asserted.

## Scope (declared)
The corpus is the canonical Exercism test data for `armstrong-numbers`. Inputs outside that set are **not covered**; this oracle decides agreement on the published corpus only and makes no claim of general correctness.

## Provenance
Reference: the Exercism reference solution for `armstrong-numbers` (rust; MIT, Exercism), body unchanged. Proven by the exercism testsuite (pin=b9c2e9f15a071892), re-executed by harvest in a sealed sandbox (unshare -rn) before this bundle was generated.

## License
Apache-2.0 for the scaffolding; the reference body retains its upstream MIT (Exercism) license.
