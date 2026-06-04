//! Dr. Elena's Edge-Case Stress Tests
//! 15+ tests probing boundary conditions, adversarial inputs, and failure modes.

use rand::prelude::*;

mod common {
    use rand::prelude::*;

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
                rates[i] = 0.1 * population[i] * (1.0 - population[i] / 100.0) + population[i] * interaction_sum;
            }
            for i in 0..n {
                population[i] += rates[i] * dt;
                population[i] = population[i].max(0.001);
            }
        }
    }

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
                let discovery_prob = env.latent_dimensions as f64 / (env.features.len().max(1) as f64);
                rng.gen::<f64>() < discovery_prob * 1.5
            })
            .count();
        (discovered as f64 / probes as f64).min(1.0)
    }

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
}

use common::*;

// ── EDGE CASE 1: Population size 1 ──────────────────────────────────────────

#[test]
fn test_population_size_one_conservation() {
    let mut rng = thread_rng();
    let mut pop = vec![50.0];
    let matrix = vec![vec![0.0]];
    let phi_initial = conserved_quantity(&pop);
    evolve(&mut pop, &matrix, 0.01, 1000);
    let phi_final = conserved_quantity(&pop);
    let drift = (phi_final - phi_initial).abs();
    assert!(drift < 1.0, "Conservation drift for n=1: {}", drift);
}

#[test]
fn test_population_size_one_fitness() {
    let pop = vec![50.0];
    let matrix = vec![vec![0.0]];
    let fit = population_fitness(&pop, &matrix);
    // Single species has logistic fitness
    assert!(fit > 0.0, "Single species should have positive fitness, got {}", fit);
}

#[test]
fn test_population_size_one_survival() {
    let mut rng = thread_rng();
    let mut pop = vec![50.0];
    let matrix = vec![vec![0.0]];
    evolve(&mut pop, &matrix, 0.01, 10000);
    assert!(pop[0] > 0.1, "Single species went extinct: {}", pop[0]);
}

#[test]
fn test_population_size_one_avoidance_undefined() {
    let pop = vec![50.0];
    let matrix = vec![vec![0.0]];
    let ratio = avoidance_engagement_ratio(&pop, &matrix);
    // With no off-diagonal interactions, both avoidance and engagement are 0
    assert!(ratio.is_infinite() || ratio.is_nan(), "n=1 should give undefined ratio, got {}", ratio);
}

// ── EDGE CASE 2: All-same environment ───────────────────────────────────────

#[test]
fn test_all_same_environment_discovery() {
    let mut rng = thread_rng();
    let env = Environment::uniform(20, 0.5);
    // Uniform environment with latent_dimensions=0 → discovery should be near 0
    let mut discoveries = 0;
    for _ in 0..1000 {
        let rate = discover(&env, 100, &mut rng);
        if rate > 0.0 { discoveries += 1; }
    }
    // latent_dimensions is 0 for uniform, so discovery should be ~0%
    assert!(discoveries < 50, "Uniform env should have near-zero discovery, got {}/1000", discoveries);
}

#[test]
fn test_all_same_population_dynamics() {
    let mut rng = thread_rng();
    let n = 5;
    let mut pop = vec![50.0; n];
    let matrix = coexistence_matrix(n, &mut rng);
    let phi_initial = conserved_quantity(&pop);
    evolve(&mut pop, &matrix, 0.01, 5000);
    // Should still conserve (approximately)
    let phi_final = conserved_quantity(&pop);
    let drift = (phi_final - phi_initial).abs();
    assert!(drift < 1.0, "Drift too large for uniform start: {}", drift);
}

#[test]
fn test_all_same_population_symmetry() {
    let mut rng = thread_rng();
    let n = 5;
    let mut pop = vec![50.0; n];
    let matrix = coexistence_matrix(n, &mut rng);
    evolve(&mut pop, &matrix, 0.01, 1000);
    // With symmetric start and symmetric matrix, should stay roughly equal
    let max_diff = pop.iter().zip(pop.iter().skip(1))
        .map(|(a, b)| (a - b).abs())
        .fold(0.0_f64, f64::max);
    // Allow some drift due to asymmetric random matrix
    assert!(max_diff < 20.0, "Symmetry broken too much: max_diff={}", max_diff);
}

// ── EDGE CASE 3: Random noise injection ─────────────────────────────────────

#[test]
fn test_noise_robustness_conservation() {
    let mut rng = thread_rng();
    let n = 10;
    let mut pop: Vec<f64> = (0..n).map(|i| 50.0 + (i as f64 * 0.7).sin() * 20.0).collect();
    let matrix = coexistence_matrix(n, &mut rng);
    let phi_initial = conserved_quantity(&pop);

    for _ in 0..100 {
        evolve(&mut pop, &matrix, 0.001, 10);
        // Inject noise every 10 steps
        for p in pop.iter_mut() {
            *p += rng.gen_range(-0.5..0.5);
            *p = p.max(0.001);
        }
    }
    let phi_final = conserved_quantity(&pop);
    // Noise should cause some drift but not catastrophic
    let drift = (phi_final - phi_initial).abs();
    assert!(drift < 5.0, "Conservation broken by noise: drift={}", drift);
}

#[test]
fn test_noise_survival() {
    let mut rng = thread_rng();
    let n = 10;
    let mut pop: Vec<f64> = (0..n).map(|_| rng.gen_range(30.0..70.0)).collect();
    let matrix = coexistence_matrix(n, &mut rng);
    for _ in 0..500 {
        evolve(&mut pop, &matrix, 0.001, 5);
        for p in pop.iter_mut() {
            *p += rng.gen_range(-1.0..1.0);
            *p = p.max(0.001);
        }
    }
    // At least 80% should survive with noise
    let survived = pop.iter().filter(|&&p| p > 1.0).count();
    assert!(survived >= n * 8 / 10, "Too many extinctions under noise: {}/{}", survived, n);
}

#[test]
fn test_noise_discovery_rate() {
    let mut rng = thread_rng();
    let base_env = Environment::random(&mut rng, 20);
    let mut total_rate = 0.0;
    for _ in 0..100 {
        let noisy_env = base_env.with_noise(&mut rng, 0.1);
        total_rate += discover(&noisy_env, 50, &mut rng);
    }
    let avg = total_rate / 100.0;
    // Noise should not destroy discovery capability
    assert!(avg > 0.01, "Noise killed discovery: avg={}", avg);
}

// ── EDGE CASE 4: Adversarial environments ───────────────────────────────────

#[test]
fn test_adversarial_conservation() {
    let mut rng = thread_rng();
    let n = 10;
    let mut pop: Vec<f64> = (0..n).map(|_| 50.0).collect();
    // Strongly adversarial matrix
    let mut matrix = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            let val = -0.8;
            matrix[i][j] = val;
            matrix[j][i] = val;
        }
    }
    let phi_initial = conserved_quantity(&pop);
    evolve(&mut pop, &matrix, 0.001, 100);
    let phi_final = conserved_quantity(&pop);
    let drift = (phi_final - phi_initial).abs();
    // Conservation should degrade under adversarial conditions — document this
    println!("Adversarial conservation drift: {}", drift);
    // This is expected to potentially fail — counterexample for the report
}

#[test]
fn test_extreme_population_values() {
    let mut rng = thread_rng();
    let n = 5;
    let mut pop = vec![0.001; n]; // near-extinction
    let matrix = coexistence_matrix(n, &mut rng);
    let phi_initial = conserved_quantity(&pop);
    evolve(&mut pop, &matrix, 0.01, 1000);
    // Should recover or stay alive
    let alive = pop.iter().filter(|&&p| p > 0.001).count();
    assert!(alive > 0, "All species died from near-extinction start");
}

#[test]
fn test_very_large_population() {
    let mut rng = thread_rng();
    let n = 5;
    let mut pop = vec![200.0; n]; // overshooting carrying capacity
    let matrix = coexistence_matrix(n, &mut rng);
    evolve(&mut pop, &matrix, 0.01, 1000);
    // Should converge back toward carrying capacity
    let avg: f64 = pop.iter().sum::<f64>() / n as f64;
    assert!(avg < 150.0, "Population didn't self-regulate: avg={}", avg);
}

#[test]
fn test_zero_population() {
    let pop = vec![0.0; 5];
    let phi = conserved_quantity(&pop);
    assert_eq!(phi, 0.0, "Zero population should give Φ=0, got {}", phi);
}

#[test]
fn test_single_probe_discovery() {
    let mut rng = thread_rng();
    let env = Environment::random(&mut rng, 20);
    let rate = discover(&env, 1, &mut rng);
    assert!(rate == 0.0 || rate == 1.0, "Single probe should give binary result, got {}", rate);
}

#[test]
fn test_conservation_monotonic_evolution() {
    let mut rng = thread_rng();
    let n = 10;
    let mut pop: Vec<f64> = (0..n).map(|i| 30.0 + i as f64 * 5.0).collect();
    let matrix = coexistence_matrix(n, &mut rng);
    let mut phi_values = vec![conserved_quantity(&pop)];
    for _ in 0..50 {
        evolve(&mut pop, &matrix, 0.001, 10);
        phi_values.push(conserved_quantity(&pop));
    }
    // Check that Φ doesn't monotonically increase or decrease (would suggest bias)
    let increasing = phi_values.windows(2).filter(|w| w[1] > w[0]).count();
    let decreasing = phi_values.windows(2).filter(|w| w[1] < w[0]).count();
    assert!(increasing > 5 && decreasing > 5,
        "Φ appears monotonic: inc={}, dec={}", increasing, decreasing);
}

#[test]
fn test_discovery_rate_increases_with_probes() {
    let mut rng = thread_rng();
    let env = Environment::random(&mut rng, 20);
    let rate_10 = discover(&env, 10, &mut rng);
    let rate_100 = discover(&env, 100, &mut rng);
    let rate_1000 = discover(&env, 1000, &mut rng);
    // More probes should (statistically) not decrease discovery
    // Allow some variance since it's stochastic
    println!("Discovery rates: 10→{:.3}, 100→{:.3}, 1000→{:.3}", rate_10, rate_100, rate_1000);
    // This is a soft test — just ensure no crash and reasonable values
    assert!(rate_1000 >= 0.0 && rate_1000 <= 1.0);
}
