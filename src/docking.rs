use rand::Rng;
use std::f64::consts::PI;
use crate::core::{Molecule, Vector3D, Transform, RotationMatrix, DockingConfig, DockingResult, DockingAlgorithm, ScoringFunction};

pub struct DockingEngine;

impl DockingEngine {
    pub fn dock(receptor: &Molecule, ligand: &Molecule, config: &DockingConfig) -> Vec<DockingResult> {
        let search_center = ligand.center_of_mass();
        
        match config.algorithm {
            DockingAlgorithm::Genetic => Self::genetic_algorithm(receptor, ligand, config, search_center),
            DockingAlgorithm::Lamarckian => Self::lamarckian_algorithm(receptor, ligand, config, search_center),
            DockingAlgorithm::PSO => Self::pso_algorithm(receptor, ligand, config, search_center),
            DockingAlgorithm::SA => Self::simulated_annealing(receptor, ligand, config, search_center),
            DockingAlgorithm::MultiStage => Self::multistage_docking(receptor, ligand, config, search_center),
        }
    }

    fn genetic_algorithm(receptor: &Molecule, ligand: &Molecule,
        config: &DockingConfig, search_center: Vector3D) -> Vec<DockingResult> {
        let mut rng = rand::thread_rng();
        let box_size = config.search_box_size;

        // Initialize population
        let mut population: Vec<(Transform, f64)> = (0..config.population_size)
            .map(|_| {
                let tx = rng.gen_range(-box_size, box_size);
                let ty = rng.gen_range(-box_size, box_size);
                let tz = rng.gen_range(-box_size, box_size);
                let rotation = RotationMatrix::random();
                let translation = search_center.add(&Vector3D::new(tx, ty, tz));
                let transform = Transform::new(rotation, translation);
                let energy = Self::score_with_function(receptor, ligand, &transform, &config.scoring_function);
                (transform, energy)
            }).collect();

        // Sort by energy
        population.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        for iteration in 0..config.max_iterations {
            if config.real_time_monitoring && iteration % 1000 == 0 {
                println!("  GA Iteration {}/{} - Best energy: {:.4}", 
                    iteration, config.max_iterations, population[0].1);
            }

            // Selection and reproduction
            let mut new_population = Vec::new();
            
            // Keep best individual (elitism)
            new_population.push(population[0].clone());

            // Generate offspring
            for _ in 1..config.population_size {
                let parent_idx = rng.gen_range(0, config.population_size.min(20));
                let mut transform = population[parent_idx].0.clone();

                // Mutation
                if rng.gen::<f64>() < config.mutation_rate {
                    transform.translation = transform.translation.add(&Vector3D::new(
                        rng.gen_range(-1.0, 1.0),
                        rng.gen_range(-1.0, 1.0),
                        rng.gen_range(-1.0, 1.0)
                    ));
                    transform.rotation = RotationMatrix::random();
                }

                let energy = Self::score_with_function(receptor, ligand, &transform, &config.scoring_function);
                new_population.push((transform, energy));
            }

            population = new_population;
            population.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        }

        // Convert to DockingResult
        population.into_iter().take(config.num_poses).enumerate().map(|(i, (transform, energy))| {
            DockingResult {
                rank: i + 1,
                transform: transform.clone(),
                energy,
                rmsd: 0.0,
                cluster_id: 0,
                cluster_size: 1,
                ligand_atoms: ligand.shifted(&transform).atoms,
            }
        }).collect()
    }

    fn lamarckian_algorithm(receptor: &Molecule, ligand: &Molecule,
        config: &DockingConfig, search_center: Vector3D) -> Vec<DockingResult> {
        let mut rng = rand::thread_rng();
        let box_size = config.search_box_size;

        // Initialize population
        let mut population: Vec<(Transform, f64)> = (0..config.population_size)
            .map(|_| {
                let tx = rng.gen_range(-box_size, box_size);
                let ty = rng.gen_range(-box_size, box_size);
                let tz = rng.gen_range(-box_size, box_size);
                let rotation = RotationMatrix::identity();
                let translation = search_center.add(&Vector3D::new(tx, ty, tz));
                let transform = Transform::new(rotation, translation);
                let energy = Self::score_with_function(receptor, ligand, &transform, &config.scoring_function);
                (transform, energy)
            }).collect();

        population.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        for iteration in 0..config.max_iterations {
            if config.real_time_monitoring && iteration % 1000 == 0 {
                println!("  LGA Iteration {}/{} - Best energy: {:.4}", 
                    iteration, config.max_iterations, population[0].1);
            }

            let mut new_population = Vec::new();
            new_population.push(population[0].clone());

            for i in 1..config.population_size {
                let parent_idx = rng.gen_range(0, config.population_size);
                let mut transform = population[parent_idx].0.clone();

                // Local optimization for some individuals
                if rng.gen::<f64>() < 0.5 {
                    transform = Self::local_optimization(receptor, ligand, &transform, &config.scoring_function);
                } else if rng.gen::<f64>() < config.mutation_rate {
                    transform.translation = transform.translation.add(&Vector3D::new(
                        rng.gen_range(-1.0, 1.0),
                        rng.gen_range(-1.0, 1.0),
                        rng.gen_range(-1.0, 1.0)
                    ));
                    transform.rotation = RotationMatrix::random();
                }

                let energy = Self::score_with_function(receptor, ligand, &transform, &config.scoring_function);
                new_population.push((transform, energy));
            }

            population = new_population;
            population.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        }

        population.into_iter().take(config.num_poses).enumerate().map(|(i, (transform, energy))| {
            DockingResult {
                rank: i + 1,
                transform: transform.clone(),
                energy,
                rmsd: 0.0,
                cluster_id: 0,
                cluster_size: 1,
                ligand_atoms: ligand.shifted(&transform).atoms,
            }
        }).collect()
    }

    fn pso_algorithm(receptor: &Molecule, ligand: &Molecule,
        config: &DockingConfig, search_center: Vector3D) -> Vec<DockingResult> {
        let mut rng = rand::thread_rng();
        let box_size = config.search_box_size;

        // Initialize particles
        let mut particles: Vec<(Transform, f64, Transform)> = (0..config.population_size)
            .map(|_| {
                let tx = rng.gen_range(-box_size, box_size);
                let ty = rng.gen_range(-box_size, box_size);
                let tz = rng.gen_range(-box_size, box_size);
                let rotation = RotationMatrix::identity();
                let translation = search_center.add(&Vector3D::new(tx, ty, tz));
                let transform = Transform::new(rotation, translation);
                let energy = Self::score_with_function(receptor, ligand, &transform, &config.scoring_function);
                (transform.clone(), energy, transform) // (position, energy, personal_best)
            }).collect();

        // Find global best
        let mut global_best = particles[0].clone();
        for particle in &particles {
            if particle.1 < global_best.1 {
                global_best = particle.clone();
            }
        }

        for iteration in 0..config.max_iterations {
            if config.real_time_monitoring && iteration % 1000 == 0 {
                println!("  PSO Iteration {}/{} - Best energy: {:.4}", 
                    iteration, config.max_iterations, global_best.1);
            }

            for i in 0..particles.len() {
                let (current_pos, current_energy, personal_best) = &mut particles[i];
                
                // Update position (simplified PSO)
                if rng.gen::<f64>() < 0.5 {
                    current_pos.translation = current_pos.translation.add(&Vector3D::new(
                        rng.gen_range(-0.5, 0.5),
                        rng.gen_range(-0.5, 0.5),
                        rng.gen_range(-0.5, 0.5)
                    ));
                }

                let new_energy = Self::score_with_function(receptor, ligand, current_pos, &config.scoring_function);
                
                // Update personal best
                if new_energy < *current_energy {
                    *personal_best = current_pos.clone();
                    *current_energy = new_energy;
                    
                    // Update global best
                    if new_energy < global_best.1 {
                        global_best = particles[i].clone();
                    }
                }
            }
        }

        particles.into_iter().take(config.num_poses).enumerate().map(|(i, (transform, energy, _))| {
            DockingResult {
                rank: i + 1,
                transform: transform.clone(),
                energy,
                rmsd: 0.0,
                cluster_id: 0,
                cluster_size: 1,
                ligand_atoms: ligand.shifted(&transform).atoms,
            }
        }).collect()
    }

    fn simulated_annealing(receptor: &Molecule, ligand: &Molecule,
        config: &DockingConfig, search_center: Vector3D) -> Vec<DockingResult> {
        let mut rng = rand::thread_rng();
        let box_size = config.search_box_size;

        // Temperature schedule
        let temp_start = 1000.0;
        let temp_end = 1.0;
        let cooling_rate = (temp_end / temp_start as f64).powf(1.0 / config.max_iterations as f64);

        // Initial state
        let tx = rng.gen_range(-box_size, box_size);
        let ty = rng.gen_range(-box_size, box_size);
        let tz = rng.gen_range(-box_size, box_size);
        let rotation = RotationMatrix::identity();
        let mut current = Transform::new(rotation, search_center.add(&Vector3D::new(tx, ty, tz)));
        let mut current_energy = Self::score_with_function(receptor, ligand, &current, &config.scoring_function);

        let mut best = current.clone();
        let mut best_energy = current_energy;
        let mut temp = temp_start;

        // Annealing loop
        for iteration in 0..config.max_iterations {
            if config.real_time_monitoring && iteration % 1000 == 0 {
                println!("  SA Iteration {}/{} - Best energy: {:.4} - Temp: {:.2}", 
                    iteration, config.max_iterations, best_energy, temp);
            }

            // Generate new state
            let new_translation = current.translation.add(&Vector3D::new(
                rng.gen_range(-box_size / 10.0, box_size / 10.0),
                rng.gen_range(-box_size / 10.0, box_size / 10.0),
                rng.gen_range(-box_size / 10.0, box_size / 10.0),
            ));
            let new_transform = Transform::new(current.rotation.clone(), new_translation);
            let new_energy = Self::score_with_function(receptor, ligand, &new_transform, &config.scoring_function);

            // Accept or reject
            let delta = new_energy - current_energy;
            if delta < 0.0 || rng.gen::<f64>() < (-delta / temp).exp() {
                current = new_transform;
                current_energy = new_energy;
                
                if current_energy < best_energy {
                    best = current.clone();
                    best_energy = current_energy;
                }
            }

            temp *= cooling_rate;
        }

        vec![DockingResult {
            rank: 1,
            transform: best.clone(),
            energy: best_energy,
            rmsd: 0.0,
            cluster_id: 0,
            cluster_size: 1,
            ligand_atoms: ligand.shifted(&best).atoms,
        }]
    }

    fn multistage_docking(receptor: &Molecule, ligand: &Molecule,
        config: &DockingConfig, search_center: Vector3D) -> Vec<DockingResult> {
        // Stage 1: Fast rigid-body docking
        let mut fast_config = config.clone();
        fast_config.max_iterations = config.max_iterations / 2;
        fast_config.population_size = config.population_size / 2;
        
        let rigid_results = Self::genetic_algorithm(receptor, ligand, &fast_config, search_center);

        // Stage 2: Local optimization of best poses
        println!("=== Stage 2: Local Optimization ===");
        let mut refined_results = Vec::new();
        for (idx, rigid_result) in rigid_results.iter().take(config.num_poses).enumerate() {
            let local_opt = Self::local_optimization(receptor, ligand, &rigid_result.transform, &config.scoring_function);
            let energy = Self::score_with_function(receptor, ligand, &local_opt, &config.scoring_function);
            
            refined_results.push(DockingResult {
                rank: idx + 1,
                energy,
                transform: local_opt.clone(),
                rmsd: 0.0,
                cluster_id: idx,
                cluster_size: 1,
                ligand_atoms: ligand.clone().shifted(&local_opt).atoms,
            });
        }

        refined_results.sort_by(|a, b| a.energy.partial_cmp(&b.energy).unwrap());
        for (i, r) in refined_results.iter_mut().enumerate() {
            r.rank = i + 1;
        }
        refined_results.truncate(config.num_poses);
        refined_results
    }

    pub fn score_with_function(receptor: &Molecule, ligand: &Molecule,
        transform: &Transform, function: &ScoringFunction) -> f64 {
        match function {
            ScoringFunction::AutoDock4 => crate::scoring::AutoDock4Scoring::calculate(receptor, ligand, transform),
            ScoringFunction::Vina => crate::scoring::VinaScoring::calculate(receptor, ligand, transform, None),
            ScoringFunction::ChemScore => crate::scoring::AutoDock4Scoring::calculate(receptor, ligand, transform),
        }
    }

    fn local_optimization(receptor: &Molecule, ligand: &Molecule,
        transform: &Transform, function: &ScoringFunction) -> Transform {
        let mut best = transform.clone();
        let mut best_energy = Self::score_with_function(receptor, ligand, &best, function);
        
        // Simple gradient-free optimization
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let step_size = 0.5;
            
            let new_translation = best.translation.add(&Vector3D::new(
                rng.gen_range(-step_size, step_size),
                rng.gen_range(-step_size, step_size),
                rng.gen_range(-step_size, step_size),
            ));
            
            let new_transform = Transform::new(best.rotation.clone(), new_translation);
            let new_energy = Self::score_with_function(receptor, ligand, &new_transform, function);
            
            if new_energy < best_energy {
                best = new_transform;
                best_energy = new_energy;
            }
        }
        
        best
    }
}
