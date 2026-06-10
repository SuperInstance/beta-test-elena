# beta-test-elena

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Language: Rust](https://img.shields.io/badge/language-Rust-orange.svg)]()
[![SuperInstance](https://img.shields.io/badge/part%20of-SuperInstance-9cf.svg)](https://github.com/SuperInstance)

Dr. Elena's rigorous stress-test of the SuperInstance ternary agent ecosystem's 5 claimed laws. Statistical validation across 10,000+ environments, 7 interaction matrices, and 7 population scales.

## What It Does

This is a **falsification machine**. It takes the 5 laws claimed by the SuperInstance framework and tries to break them with adversarial inputs, extreme scales, and statistical brute force. The results feed into `BETA-REPORT.md` — a law-by-law verdict with specific counterexamples and suggested fixes.

## Results at a Glance

| Law | Claim | Verdict | Pass Rate |
|-----|-------|---------|-----------|
| 1 | Negative space discovers structure | ✅ PASS | 99.99% (10,000 envs) |
| 2 | Avoidance dominates (>100:1) | ✅ PASS* | 100/100 runs |
| 3 | Species coexistence (100% survival) | ❌ FAIL | 14.3% (1/7 matrices) |
| 4 | Population > Individual | ❌ TAUTOLOGY | 0% (identical by construction) |
| 5 | Conservation (std < 0.01) | ❌ FAIL | 0% (massive drift at all scales) |

*Law 2 passes trivially — all-adversarial matrices produce ∞ ratio (zero engagement), not meaningful dominance.

## Installation

```bash
cargo run --bin beta-test-elena
```

Dependencies: `rand` 0.8, `rand_distr` 0.4, `statrs` 0.16.

## Architecture

```
main.rs
├── conservation_matrix
│   ├── conserved_quantity(population) → Φ
│   │     Φ = -Σ pᵢ·ln(pᵢ/K) + Σ pᵢ²/K²
│   └── evolve(population, interaction, dt, steps)
│         Lotka-Volterra with interaction terms
│
├── negative_space_core
│   ├── Environment { features, latent_dimensions }
│   │     random(), uniform(), with_noise()
│   └── discover(env, probes, rng) → f64
│         Stochastic sampling with 1.5× amplification
│
├── ternary_fitness
│   ├── population_fitness(pop, matrix) → f64
│   └── individual_fitness_sum(pop, matrix) → f64
│
├── Law 1 test: 10,000 random environments
├── Law 2 test: 100 adversarial runs
├── Law 3 test: 7 interaction matrices (3–15 species)
├── Law 4 test: 50 random trials (n=3..11)
└── Law 5 test: 7 scales (n=10..10,000)
```

## The Five Laws Tested

### Law 1: Negative Space Discovery

```rust
// From negative_space_core module
pub fn discover(env: &Environment, probes: usize, rng: &mut impl Rng) -> f64 {
    let discovered = (0..probes)
        .filter(|_| {
            let prob = env.latent_dimensions as f64 / (env.features.len().max(1) as f64);
            rng.gen::<f64>() < prob * 1.5
        })
        .count();
    (discovered as f64 / probes as f64).min(1.0)
}
```

10,000 random environments tested. Mean discovery rate = 0.41, nonzero discovery = 99.98%. **Passes.**

### Law 2: Avoidance Dominance

```rust
// Avoidance/engagement ratio with adversarial matrices
fn avoidance_engagement_ratio(pop: &[f64], matrix: &[Vec<f64>]) -> f64 {
    // All-negative matrix → engagement = 0 → ratio = ∞
}
```

100/100 runs produce ratio = ∞. Passes, but trivially — pure adversarial matrices have zero engagement force. Needs mixed-sign tests.

### Law 3: Species Coexistence

7 Lotka-Volterra configurations tested. Only the 4-species cyclic dominance matrix survives. The 3-species weak competition, 5-species, 7-species, 10-species, 6-species partitioning, and 15-species ecosystems all show extinction.

### Law 4: Population > Individual

```rust
// The tautology:
// F_pop = Σᵢ [f_self(i) + 0.01 × Σⱼ≠ᵢ aᵢⱼ pᵢ pⱼ]
// f_ind(i) = f_self(i) + 0.01 × Σⱼ≠ᵢ aᵢⱼ pᵢ pⱼ
// Therefore: F_pop = Σᵢ f_ind(i)  ← true by definition
```

All 50 trials show advantage = 0.0000. The population fitness is the sum of individual fitnesses exactly. No emergent collective advantage exists in the mathematics.

### Law 5: Conservation

```rust
pub fn conserved_quantity(population: &[f64]) -> f64 {
    let total: f64 = population.iter().sum();
    let entropy = population.iter()
        .map(|&p| if p > 0.0 { -p * (p / total).ln() } else { 0.0 })
        .sum();
    let energy = population.iter().map(|&p| p * p).sum::<f64>() / (total * total);
    entropy + energy
}
```

Standard deviation of Φ ranges from 233 (n=10) to 689,704 (n=10,000). Required: < 0.01. The functional Φ = -Σ pᵢ·ln(pᵢ/K) + Σ pᵢ²/K² is not conserved by the Lotka-Volterra dynamics.

## Edge-Case Tests

`tests/edge_cases.rs` contains 17 boundary-condition tests:

- Population size 1 (4 tests) — degenerate case handled
- All-same environment — near-zero discovery confirmed
- Random noise injection — conservation breaks under noise (drift=1011)
- Adversarial matrices — documents expected conservation degradation
- Extreme populations — self-regulation works above/below carrying capacity
- Conservation monotonicity — Φ is not monotonic (no bias, but confirms non-conservation)

## Key Files

```
beta-test-elena/
├── src/main.rs           # All 5 law tests + modules (~400 lines)
├── tests/edge_cases.rs   # 17 boundary-condition tests
├── BETA-REPORT.md        # Full law-by-law analysis with counterexamples
├── test-output.txt       # Raw test run output
└── docs/                 # Integration notes
```

## Running

```bash
# Run the full stress test
cargo run --bin beta-test-elena

# Run edge-case tests
cargo test

# View the report
cat BETA-REPORT.md
```

## Suggested Fixes (from BETA-REPORT.md)

1. **Law 3**: Add diagonal dominance condition `aᵢᵢ > Σⱼ≠ᵢ |aᵢⱼ|` for guaranteed coexistence
2. **Law 4**: Introduce higher-order interaction terms `α × Σᵢ<ⱼ<ₖ aᵢⱼₖ pᵢ pⱼ pₖ`
3. **Law 5**: Replace exact conservation with dissipation bound `|Φ(t) - Φ(0)| ≤ C × t × max(|aᵢⱼ|)`
4. **Law 2**: Require mixed-sign matrices with ≥20% positive entries

## Related Repos

| Repo | Role |
|------|------|
| `conservation-verify` | Multi-scale verification of γ + η = C |
| `negative-space-core` | Core discovery and tracking |
| `beta-test-marcus` | Investor due-diligence perspective |
| `beta-test-priya` | CS student usability testing |
| `superinstance-architecture` | Architecture specification |

---

*"The strength of a theory is not in what it proves, but in what it survives."* — Dr. Elena
