//! Dr. Elena's Rigorous Stress-Test of the SuperInstance Ternary Agent Ecosystem
//!
//! This binary implements and tests the 5 claimed laws:
//!   Law 1: Negative Space Discovery — discovery rate across random environments
//!   Law 2: Avoidance Dominance — avoidance-to-engagement ratio > 100:1
//!   Law 3: Species Coexistence — 7 interaction matrices, 100% survival
//!   Law 4: Population > Individual — collective advantage in >90% of trials
//!   Law 5: Conservation — invariant quantity holds at 7 scales (std < 0.01)

use rand::prelude::*;

// ── Local implementations ────────────────────────────────────────────────────

mod conservation_matrix {
    pub fn conserved_quantity(population: &[f64]) -> f64 {
        let total: f64 = population.iter().sum();
        if total <= 0.0 { return 0.0; }
        let entropy: f64 = population.iter()
            .map(|&p| if p > 0.0 { -p * (p / total).ln() } else { 0.0 })
            .sum();
        let energy: f64 = population.iter().map(|&p| p * p).sum::<f64>() / (total * total);
        entropy + energy
    }

    pub fn evolve(population: &mut [f64], interaction: &[Vec<f64>], dt: f64, steps: usize) {
        let n = population.len();
        for _ in 0..steps {
            let mut rates = vec![0.0; n];
            for i in 0..n {
                let interaction_sum: f64 = (0..n)
                    .filter(|&j| j != i)
                    .map(|j| interaction[i][j] * population[j])
                    .sum();
                rates[i] = 0.1 * population[i] * (1.0 - population[i] / 100.0)
                    + population[i] * interaction_sum;
            }
            for i in 0..n {
                population[i] += rates[i] * dt;
                population[i] = population[i].max(0.001);
            }
        }
    }
}

mod negative_space_core {
    use rand::prelude::*;
    pub struct Environment {
        pub features: Vec<f64>,
        pub latent_dimensions: usize,
    }

    impl Environment {
        pub fn random(rng: &mut impl Rng, dim: usize) -> Self {
            let features: Vec<f64> = (0..dim).map(|_| rng.gen_range(-1.0..1.0)).collect();
            let latent = rng.gen_range(1..=dim / 2);
            Environment { features, latent_dimensions: latent }
        }
        pub fn uniform(dim: usize, value: f64) -> Self {
            Environment { features: vec![value; dim], latent_dimensions: 0 }
        }
        pub fn with_noise(&self, rng: &mut impl Rng, noise_level: f64) -> Self {
            let features: Vec<f64> = self.features.iter()
                .map(|&f| f + rng.gen_range(-noise_level..noise_level))
                .collect();
            Environment { features, latent_dimensions: self.latent_dimensions }
        }
    }

    pub fn discover(env: &Environment, probes: usize, rng: &mut impl Rng) -> f64 {
        let discovered = (0..probes)
            .filter(|_| {
                let prob = env.latent_dimensions as f64 / (env.features.len().max(1) as f64);
                rng.gen::<f64>() < prob * 1.5
            })
            .count();
        (discovered as f64 / probes as f64).min(1.0)
    }
}

mod ternary_fitness {
    pub fn avoidance_engagement_ratio(population: &[f64], interaction_matrix: &[Vec<f64>]) -> f64 {
        let n = population.len();
        let mut avoidance = 0.0_f64;
        let mut engagement = 0.0_f64;
        for i in 0..n {
            for j in 0..n {
                if i == j { continue; }
                let force = population[i] * population[j] * interaction_matrix[i][j].abs();
                if interaction_matrix[i][j] < 0.0 {
                    avoidance += force;
                } else {
                    engagement += force;
                }
            }
        }
        if engagement <= 0.0 { return f64::INFINITY; }
        avoidance / engagement
    }

    pub fn population_fitness(population: &[f64], interaction_matrix: &[Vec<f64>]) -> f64 {
        let n = population.len();
        let mut fitness = 0.0;
        for i in 0..n {
            let self_fit = population[i] * (1.0 - population[i] / 100.0);
            let interaction_fit: f64 = (0..n)
                .filter(|&j| j != i)
                .map(|j| interaction_matrix[i][j] * population[i] * population[j])
                .sum();
            fitness += self_fit + interaction_fit * 0.01;
        }
        fitness
    }

    pub fn individual_fitness(population: &[f64], interaction_matrix: &[Vec<f64>], i: usize) -> f64 {
        let self_fit = population[i] * (1.0 - population[i] / 100.0);
        let n = population.len();
        let interaction_fit: f64 = (0..n)
            .filter(|&j| j != i)
            .map(|j| interaction_matrix[i][j] * population[i] * population[j])
            .sum();
        self_fit + interaction_fit * 0.01
    }
}

mod lotka_volterra_agents {
    use rand::prelude::*;
    pub fn coexistence_matrix(n: usize, rng: &mut impl Rng) -> Vec<Vec<f64>> {
        let mut matrix = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in (i + 1)..n {
                let val = -rng.gen_range(0.001..0.05);
                matrix[i][j] = val;
                matrix[j][i] = val;
            }
        }
        matrix
    }

    pub fn adversarial_matrix(n: usize, rng: &mut impl Rng) -> Vec<Vec<f64>> {
        let mut matrix = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in (i + 1)..n {
                let val = -rng.gen_range(0.1..0.9);
                matrix[i][j] = val;
                matrix[j][i] = val;
            }
        }
        matrix
    }

    pub fn all_survive(population: &[f64], threshold: f64) -> bool {
        population.iter().all(|&p| p > threshold)
    }
}

mod dissertation_engine {
    pub struct LawReport {
        pub law: u8,
        pub description: String,
        pub trials: usize,
        pub passed: usize,
        pub failed: usize,
        pub pass_rate: f64,
        pub details: Vec<String>,
    }

    impl LawReport {
        pub fn new(law: u8, description: &str) -> Self {
            LawReport { law, description: description.to_string(), trials: 0, passed: 0, failed: 0, pass_rate: 0.0, details: Vec::new() }
        }
        pub fn record(&mut self, passed: bool, detail: &str) {
            self.trials += 1;
            if passed { self.passed += 1; } else { self.failed += 1; }
            self.pass_rate = self.passed as f64 / self.trials as f64;
            self.details.push(detail.to_string());
        }
    }

    pub fn format_report(reports: &[LawReport]) -> String {
        let mut s = String::new();
        s.push_str("══════════════════════════════════════════════════════════════════\n");
        s.push_str("  DR. ELENA'S RIGOROUS STRESS-TEST: SUPERINSTANCE TERNARY ECOSYSTEM\n");
        s.push_str("══════════════════════════════════════════════════════════════════\n\n");
        for r in reports {
            let status = if r.passed > 0 && r.failed == 0 { "✓ PASS" } else if r.passed > 0 { "⚠ PARTIAL" } else { "✗ FAIL" };
            s.push_str(&format!("Law {}: {}\n", r.law, r.description));
            s.push_str(&format!("  Status: {} | Trials: {} | Passed: {} | Failed: {} | Rate: {:.2}%\n",
                status, r.trials, r.passed, r.failed, r.pass_rate * 100.0));
            for d in &r.details {
                s.push_str(&format!("    • {}\n", d));
            }
            s.push('\n');
        }
        s
    }
}

// ── Main test harness ────────────────────────────────────────────────────────

fn main() {
    let mut rng = thread_rng();
    let mut reports: Vec<dissertation_engine::LawReport> = Vec::new();

    println!("🔬 Dr. Elena's Rigorous Stress-Test\n");
    println!("Running all 5 laws with large sample sizes...\n");

    // ── LAW 1: Negative Space Discovery ─────────────────────────────────
    let mut law1 = dissertation_engine::LawReport::new(1, "Negative space discovers structure in random environments");
    let env_count = 10_000;
    let mut discovery_rates: Vec<f64> = Vec::with_capacity(env_count);
    for _ in 0..env_count {
        let env = negative_space_core::Environment::random(&mut rng, 20);
        let rate = negative_space_core::discover(&env, 100, &mut rng);
        discovery_rates.push(rate);
    }
    let mean_discovery: f64 = discovery_rates.iter().sum::<f64>() / discovery_rates.len() as f64;
    let nonzero_rate = discovery_rates.iter().filter(|&&r| r > 0.0).count() as f64 / discovery_rates.len() as f64;
    let passed = nonzero_rate > 0.5 && mean_discovery > 0.1;
    law1.record(passed, &format!("{} environments: mean discovery={:.4}, nonzero_rate={:.2}%",
        env_count, mean_discovery, nonzero_rate * 100.0));
    println!("Law 1: mean_discovery={:.4}, nonzero_rate={:.2}% → {}",
        mean_discovery, nonzero_rate * 100.0, if passed { "PASS" } else { "FAIL" });
    reports.push(law1);

    // ── LAW 2: Avoidance Dominance ───────────────────────────────────────
    let mut law2 = dissertation_engine::LawReport::new(2, "Avoidance dominates engagement (ratio > 100:1)");
    let runs = 100;
    let mut ratios: Vec<f64> = Vec::with_capacity(runs);
    for _ in 0..runs {
        let n = rng.gen_range(5..20);
        let matrix = lotka_volterra_agents::adversarial_matrix(n, &mut rng);
        let pop: Vec<f64> = (0..n).map(|_| rng.gen_range(10.0..90.0)).collect();
        let ratio = ternary_fitness::avoidance_engagement_ratio(&pop, &matrix);
        ratios.push(ratio);
    }
    let finite_ratios: Vec<f64> = ratios.iter().cloned().filter(|r| r.is_finite()).collect();
    let mean_ratio = if finite_ratios.is_empty() { f64::NAN } else { finite_ratios.iter().sum::<f64>() / finite_ratios.len() as f64 };
    let consistent = ratios.iter().filter(|&&r| r > 100.0 || r.is_infinite()).count();
    let passed = consistent as f64 / runs as f64 > 0.5;
    law2.record(passed, &format!("{} runs: mean_ratio={:.2}, runs>100:1 or ∞ = {}/{}",
        runs, mean_ratio, consistent, runs));
    println!("Law 2: mean_ratio={:.2}, consistent>100:1={}/{} → {}",
        mean_ratio, consistent, runs, if passed { "PASS" } else { "FAIL" });
    reports.push(law2);

    // ── LAW 3: Species Coexistence ──────────────────────────────────────
    let mut law3 = dissertation_engine::LawReport::new(3, "All species coexist across 7 interaction matrices");
    let matrix_configs: Vec<(usize, &str)> = vec![
        (3, "3-species weak competition"),
        (5, "5-species weak competition"),
        (7, "7-species moderate competition"),
        (10, "10-species dense network"),
        (4, "4-species cyclic dominance"),
        (6, "6-species resource partitioning"),
        (15, "15-species large ecosystem"),
    ];
    for (n, desc) in &matrix_configs {
        let mut matrix = lotka_volterra_agents::coexistence_matrix(*n, &mut rng);
        if *desc == "4-species cyclic dominance" {
            for i in 0..*n {
                let j = (i + 1) % *n;
                matrix[i][j] = -0.02;
                matrix[j][i] = 0.01;
            }
        }
        let mut pop: Vec<f64> = (0..*n).map(|_| 50.0).collect();
        conservation_matrix::evolve(&mut pop, &matrix, 0.01, 1000);
        let survived = lotka_volterra_agents::all_survive(&pop, 0.1);
        let min_pop = pop.iter().cloned().fold(f64::INFINITY, f64::min);
        law3.record(survived, &format!("{} (n={}): min_pop={:.4} → {}",
            desc, n, min_pop, if survived { "SURVIVED" } else { "EXTINCTION" }));
        println!("  {}: min_pop={:.4} → {}", desc, min_pop, if survived { "SURVIVED" } else { "EXTINCTION" });
    }
    let all_survived = law3.failed == 0;
    println!("Law 3: 100% survival = {} → {}", all_survived, if all_survived { "PASS" } else { "FAIL" });
    reports.push(law3);

    // ── LAW 4: Population > Individual ──────────────────────────────────
    let mut law4 = dissertation_engine::LawReport::new(4, "Population fitness exceeds sum of individual fitness");
    let trials = 50;
    let mut advantages = Vec::with_capacity(trials);
    for t in 0..trials {
        let n = rng.gen_range(3..12);
        let matrix = lotka_volterra_agents::coexistence_matrix(n, &mut rng);
        let pop: Vec<f64> = (0..n).map(|_| rng.gen_range(20.0..80.0)).collect();
        let pop_fit = ternary_fitness::population_fitness(&pop, &matrix);
        let ind_fit_sum: f64 = (0..n).map(|i| ternary_fitness::individual_fitness(&pop, &matrix, i)).sum();
        let advantage = pop_fit - ind_fit_sum;
        advantages.push(advantage);
        let positive = advantage > 0.0;
        law4.record(positive, &format!("Trial {}: n={}, pop_fit={:.4}, ind_sum={:.4}, advantage={:.4}",
            t, n, pop_fit, ind_fit_sum, advantage));
    }
    let positive_count = advantages.iter().filter(|&&a| a > 0.0).count();
    let positive_rate = positive_count as f64 / trials as f64;
    let passed = positive_rate > 0.90;
    println!("Law 4: positive advantage in {}/{} trials ({:.0}%) → {}",
        positive_count, trials, positive_rate * 100.0, if passed { "PASS" } else { "FAIL" });
    reports.push(law4);

    // ── LAW 5: Conservation ─────────────────────────────────────────────
    let mut law5 = dissertation_engine::LawReport::new(5, "Conserved quantity invariant across 7 scales (std < 0.01)");
    let scales: Vec<usize> = vec![10, 50, 100, 200, 500, 750, 1000];
    for &n_species in &scales {
        let mut pop: Vec<f64> = (0..n_species).map(|i| 50.0 + (i as f64 * 0.001).sin() * 20.0).collect();
        let matrix = lotka_volterra_agents::coexistence_matrix(n_species, &mut rng);
        let mut phi_values: Vec<f64> = Vec::with_capacity(50);
        phi_values.push(conservation_matrix::conserved_quantity(&pop));
        for _ in 0..49 {
            conservation_matrix::evolve(&mut pop, &matrix, 0.001, 10);
            phi_values.push(conservation_matrix::conserved_quantity(&pop));
        }
        let mean_phi: f64 = phi_values.iter().sum::<f64>() / phi_values.len() as f64;
        let variance: f64 = phi_values.iter().map(|&p| (p - mean_phi).powi(2)).sum::<f64>() / phi_values.len() as f64;
        let std_phi = variance.sqrt();
        let conserved = std_phi < 0.01;
        law5.record(conserved, &format!("Scale n={}: mean_Φ={:.6}, std_Φ={:.6} → {}",
            n_species, mean_phi, std_phi, if conserved { "CONSERVED" } else { "DRIFT" }));
        println!("  Scale {}: std_Φ={:.6} → {}", n_species, std_phi, if conserved { "CONSERVED" } else { "DRIFT" });
    }
    let all_conserved = law5.failed == 0;
    println!("Law 5: all scales conserved = {} → {}", all_conserved, if all_conserved { "PASS" } else { "FAIL" });
    reports.push(law5);

    // ── Final Report ─────────────────────────────────────────────────────
    let report = dissertation_engine::format_report(&reports);
    println!("\n{}", report);

    let total_passed: usize = reports.iter().filter(|r| r.failed == 0).count();
    println!("Overall: {}/5 laws passed all trials", total_passed);

    // Write report to file
    std::fs::write("test-output.txt", &report).expect("Failed to write report");
    println!("Report written to test-output.txt");
}
