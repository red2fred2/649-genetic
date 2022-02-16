extern crate random_choice;
use self::random_choice::random_choice;

use rand::{Rng, prelude::ThreadRng};

const EPSILON: f64 = 0.01;
const POPULATION: usize = 100;
const GENERATIONS: usize = 1000;
const SURVIVORS: usize = 75;
const TOP_RESULTS: usize = 5;
const CROSSOVER: bool = true;

fn main() {
	let mut rng = rand::thread_rng();

	// Generate individuals
	let mut population: Vec<f64> = Vec::new();
	for _ in 0..POPULATION {
		population.push(rng.gen_range(0.0..1.0));
	}

	// Run through all the generations
	for _ in 0..GENERATIONS {
		// 'natural' selection
		population = roulette(population, SURVIVORS);

		// Mutation
		for individual in &mut population {
			*individual = mutate(*individual, &mut rng);
		}

		// Generate new ones
		if CROSSOVER {
			// Fill in missing population with crossover between random ones
			while population.len() < POPULATION - 1 {
				// Determine which survivors to pick
				let xi = rng.gen_range(0..population.len());
				let yi = rng.gen_range(0..population.len());

				// Get that survivor
				let x = &population[xi];
				let y = &population[yi];

				// Breed the survivors
				let new = crossover(x, y, &mut rng);

				// Add that to the population
				population.push(new);
			}
		} else {
			// Just choose at random
			while population.len() < POPULATION - 1 {
				// Choose a random number between 0 and 1
				population.push(rng.gen_range(0.0..1.0));
			}
		}
	}

	// Sort results
	population.sort_by(|a, b| {
		let fa = fitness(a);
		let fb = fitness(b);

		fa.partial_cmp(&fb).unwrap()
	});

	// Display results
	for _ in 0..TOP_RESULTS {
		let f = fitness(&population.pop().unwrap());
		println!("{:.3}", f);
	}

}

// Fitness function
fn fitness(x: &f64) -> f64 {
	4. + 2. * x + 2. * (20. * x).sin() - 4. * x.powi(2)
}

fn mutate(x: f64, rng: &mut ThreadRng) -> f64 {
	// Choose random number between 0 and 1
	let mutation_num = rng.gen_range(0.0..1.0);

	// Choose how to mutate based on that number
	if mutation_num < 0.3 {
		// Move left
		return x - EPSILON
	} else if mutation_num > 0.7 {
		// Move right
		return x + EPSILON
	} else {
		// Don't move
		return x
	}
}

fn roulette(population: Vec<f64>, draws: usize) -> Vec<f64> {
	let total_fitness: f64 = population.iter().sum();

	// Determine weighting
	let mut weights: Vec<f64> = Vec::new();
	for individual in &population {
		let weight = fitness(&individual) / total_fitness;
		weights.push(weight);
	}

	// Do the roulette choice
	let choices = random_choice().random_choice_f64(&population, &weights, draws);

	// Deal with rust being annoying
	let mut choices_d = Vec::new();
	for choice in choices {
		choices_d.push(*choice);
	}

	// Return new population
	choices_d
}

fn crossover(x: &f64, y: &f64, rng: &mut ThreadRng) -> f64 {
	// Get random weight
	let weight = rng.gen_range(0.0..1.0);

	// Return crossover value
	weight *  x + (1. - weight) * y
}