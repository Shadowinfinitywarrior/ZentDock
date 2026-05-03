use clap::{App, Arg, SubCommand};
use std::path::PathBuf;
use zentdock::{
    config::{AppConfig},
    core::{Vector3D},
    DockingConfig, DockingAlgorithm, ScoringFunction,
    docking::DockingEngine,
    parser::MoleculeParser,
    utils::{JobManager, StructurePreparer},
    tools::{FileConverter, MoleculeInfo, ReceptorPreparer, ScoringTool},
    batch::{BatchProcessor, MultiTargetProcessor},
    storage::PoseStorage,
    monitoring::ActivityMonitor,
};

fn parse_center(s: &str) -> Result<Vector3D, String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 3 {
        return Err("Center must be in format x,y,z".to_string());
    }
    
    let x = parts[0].parse::<f64>().map_err(|_| "Invalid x coordinate")?;
    let y = parts[1].parse::<f64>().map_err(|_| "Invalid y coordinate")?;
    let z = parts[2].parse::<f64>().map_err(|_| "Invalid z coordinate")?;
    
    Ok(Vector3D::new(x, y, z))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("zentdock")
        .version("1.0.0")
        .author("Mr. Nithish Kathiravan <nithishkathiravan123@gmail.com>")
        .about("Comprehensive molecular docking software")
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Enable verbose output")
            .global(true))
        .subcommand(SubCommand::with_name("dock")
            .about("Run docking simulation")
            .arg(Arg::with_name("receptor")
                .short("r")
                .long("receptor")
                .value_name("FILE")
                .help("Receptor PDB file")
                .required(true))
            .arg(Arg::with_name("ligand")
                .short("l")
                .long("ligand")
                .value_name("FILE")
                .help("Ligand file")
                .required(true))
            .arg(Arg::with_name("num_poses")
                .long("num-poses")
                .value_name("N")
                .help("Number of poses to generate")
                .default_value("10"))
            .arg(Arg::with_name("algorithm")
                .long("algorithm")
                .value_name("ALGO")
                .help("Docking algorithm")
                .possible_values(&["genetic", "lamarckian", "pso", "sa", "multistage"])
                .default_value("genetic"))
            .arg(Arg::with_name("center")
                .long("center")
                .value_name("X,Y,Z")
                .help("Binding site center"))
            .arg(Arg::with_name("radius")
                .long("radius")
                .value_name("R")
                .help("Search space radius in Angstroms")
                .default_value("10.0"))
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("DIR")
                .help("Output directory")
                .default_value("."))
            .arg(Arg::with_name("max_iterations")
                .long("max-iterations")
                .value_name("N")
                .help("Maximum iterations")
                .default_value("25000"))
            .arg(Arg::with_name("population_size")
                .long("population-size")
                .value_name("N")
                .help("Population size")
                .default_value("150"))
            .arg(Arg::with_name("scoring")
                .long("scoring")
                .value_name("METHOD")
                .help("Scoring function")
                .possible_values(&["autodock4", "vina", "chemscore"])
                .default_value("vina"))
            .arg(Arg::with_name("monitor")
                .long("monitor")
                .help("Enable real-time monitoring"))
            .arg(Arg::with_name("stream")
                .long("stream")
                .help("Enable streaming output"))
            .arg(Arg::with_name("checkpoint")
                .long("checkpoint")
                .value_name("N")
                .help("Checkpoint interval (0 = disabled)")
                .default_value("0"))
            .arg(Arg::with_name("cluster")
                .long("cluster")
                .help("Enable RMSD clustering")))
        .subcommand(SubCommand::with_name("batch")
            .about("Batch processing of multiple ligands")
            .arg(Arg::with_name("receptor")
                .short("r")
                .long("receptor")
                .value_name("FILE")
                .help("Receptor file")
                .required(true))
            .arg(Arg::with_name("ligands")
                .short("l")
                .long("ligands")
                .value_name("PATH")
                .help("Ligands directory or file")
                .required(true))
            .arg(Arg::with_name("jobs")
                .long("jobs")
                .value_name("N")
                .help("Number of parallel jobs")
                .default_value("1"))
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("DIR")
                .help("Output directory")
                .default_value("."))
            .arg(Arg::with_name("continue_on_failure")
                .long("continue-on-failure")
                .help("Continue processing on failure"))
            .arg(Arg::with_name("algorithm")
                .long("algorithm")
                .value_name("ALGO")
                .help("Docking algorithm")
                .possible_values(&["genetic", "lamarckian", "pso", "sa", "multistage"])
                .default_value("genetic"))
            .arg(Arg::with_name("num_poses")
                .long("num-poses")
                .value_name("N")
                .help("Number of poses per ligand")
                .default_value("10"))
            .arg(Arg::with_name("max_iterations")
                .long("max-iterations")
                .value_name("N")
                .help("Maximum iterations")
                .default_value("25000")))
        .subcommand(SubCommand::with_name("prepare")
            .about("Prepare molecules for docking")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Input file")
                .required(true))
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Output file")
                .required(true))
            .arg(Arg::with_name("remove_waters")
                .long("remove-waters")
                .help("Remove water molecules"))
            .arg(Arg::with_name("add_hydrogens")
                .long("add-hydrogens")
                .help("Add hydrogen atoms"))
            .arg(Arg::with_name("fix_sidechains")
                .long("fix-sidechains")
                .help("Fix sidechains"))
            .arg(Arg::with_name("protonate")
                .long("protonate")
                .help("Protonate molecule")))
        .subcommand(SubCommand::with_name("score")
            .about("Score a pose")
            .arg(Arg::with_name("receptor")
                .short("r")
                .long("receptor")
                .value_name("FILE")
                .help("Receptor file")
                .required(true))
            .arg(Arg::with_name("ligand")
                .short("l")
                .long("ligand")
                .value_name("FILE")
                .help("Ligand file")
                .required(true))
            .arg(Arg::with_name("method")
                .long("method")
                .value_name("METHOD")
                .help("Scoring method")
                .possible_values(&["autodock4", "vina", "chemscore"])
                .default_value("vina")))
        .subcommand(SubCommand::with_name("convert")
            .about("Convert file formats")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Input file")
                .required(true))
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Output file")
                .required(true))
            .arg(Arg::with_name("format")
                .long("format")
                .value_name("FMT")
                .help("Output format")
                .possible_values(&["pdb", "mol2", "sdf"])
                .default_value("pdb")))
        .subcommand(SubCommand::with_name("info")
            .about("Show molecule information")
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Molecule file")
                .required(true)))
        .subcommand(SubCommand::with_name("config")
            .about("Configuration management")
            .arg(Arg::with_name("show")
                .long("show")
                .help("Show configuration"))
            .arg(Arg::with_name("path")
                .long("path")
                .value_name("FILE")
                .help("Config file path"))
            .arg(Arg::with_name("set")
                .long("set")
                .value_name("KEY=VALUE")
                .help("Set config value")))
        .get_matches();

    let verbose = matches.is_present("verbose");

    match matches.subcommand() {
        ("dock", Some(dock_matches)) => {
            println!("=== ZentDock: Docking Simulation ===");
            
            // Load molecules
            let receptor_path = dock_matches.value_of("receptor").unwrap();
            let ligand_path = dock_matches.value_of("ligand").unwrap();
            
            let receptor_content = std::fs::read_to_string(receptor_path)?;
            let ligand_content = std::fs::read_to_string(ligand_path)?;
            
            let receptor = MoleculeParser::parse_pdb(&receptor_content);
            let ligand = MoleculeParser::parse_pdb(&ligand_content);
            
            // Configure docking
            let algorithm_str = dock_matches.value_of("algorithm").unwrap();
            let docking_algorithm = match algorithm_str {
                "genetic" => DockingAlgorithm::Genetic,
                "lamarckian" => DockingAlgorithm::Lamarckian,
                "pso" => DockingAlgorithm::PSO,
                "sa" => DockingAlgorithm::SA,
                "multistage" => DockingAlgorithm::MultiStage,
                _ => return Err("Invalid algorithm".into()),
            };
            
            let scoring_str = dock_matches.value_of("scoring").unwrap();
            let scoring_function = match scoring_str {
                "autodock4" => ScoringFunction::AutoDock4,
                "vina" => ScoringFunction::Vina,
                "chemscore" => ScoringFunction::ChemScore,
                _ => return Err("Invalid scoring function".into()),
            };
            
            let num_poses: usize = dock_matches.value_of("num_poses").unwrap().parse()?;
            let max_iterations: usize = dock_matches.value_of("max_iterations").unwrap().parse()?;
            let population_size: usize = dock_matches.value_of("population_size").unwrap().parse()?;
            let radius: f64 = dock_matches.value_of("radius").unwrap().parse()?;
            let output_dir = dock_matches.value_of("output").unwrap();
            
            let center = if let Some(center_str) = dock_matches.value_of("center") {
                Some(parse_center(center_str)?)
            } else {
                None
            };
            
            let config = DockingConfig {
                algorithm: docking_algorithm,
                scoring_function,
                num_poses,
                max_iterations,
                population_size,
                search_box_size: radius,
                real_time_monitoring: dock_matches.is_present("monitor"),
                streaming_output: dock_matches.is_present("stream"),
                checkpoint_interval: dock_matches.value_of("checkpoint").unwrap().parse()?,
                rmsd_clustering: dock_matches.is_present("cluster"),
                ..Default::default()
            };
            
            // Set search center
            if let Some(center_coords) = center {
                println!("  Using specified center: ({:.2}, {:.2}, {:.2})", 
                    center_coords.x, center_coords.y, center_coords.z);
            } else {
                println!("  Using ligand center of mass as search center");
            }
            
            // Run docking
            let results = JobManager::submit_docking(receptor, ligand, config);
            
            // Save results
            std::fs::create_dir_all(output_dir)?;
            
            // Save best pose
            if let Some(best_result) = results.first() {
                let complex_pdb = format!("{}/best_complex.pdb", output_dir);
                let complex_content = MoleculeParser::write_complex(
                    &MoleculeParser::parse_pdb(&receptor_content),
                    &MoleculeParser::parse_pdb(&ligand_content)
                );
                std::fs::write(&complex_pdb, complex_content)?;
                println!("  Best pose saved to: {}", complex_pdb);
            }
            
            // Save all poses
            for (i, result) in results.iter().enumerate() {
                let pose_file = format!("{}/pose_{}_E{:.4}.pdb", output_dir, i + 1, result.energy);
                let transformed_ligand = MoleculeParser::parse_pdb(&ligand_content)
                    .shifted(&result.transform);
                let pose_content = MoleculeParser::write_pdb(&transformed_ligand, 
                    Some(&format!("Pose {} - Energy: {:.4}", i + 1, result.energy)));
                std::fs::write(&pose_file, pose_content)?;
            }
            
            println!("  All poses saved to: {}", output_dir);
        },
        
        ("batch", Some(batch_matches)) => {
            println!("=== ZentDock: Batch Processing ===");
            
            let receptor_path = batch_matches.value_of("receptor").unwrap();
            let ligands_path = batch_matches.value_of("ligands").unwrap();
            let jobs: usize = batch_matches.value_of("jobs").unwrap().parse()?;
            let output_dir = batch_matches.value_of("output").unwrap();
            let continue_on_failure = batch_matches.is_present("continue_on_failure");
            let algorithm = batch_matches.value_of("algorithm").unwrap();
            let num_poses: usize = batch_matches.value_of("num_poses").unwrap().parse()?;
            let max_iterations: usize = batch_matches.value_of("max_iterations").unwrap().parse()?;
            
            // Check if ligands is a file or directory
            let ligands_path_buf = PathBuf::from(ligands_path);
            if ligands_path_buf.is_file() {
                println!("  Processing ligands from file: {}", ligands_path);
            } else if ligands_path_buf.is_dir() {
                println!("  Processing ligands from directory: {}", ligands_path);
                
                // Get all PDB files
                let mut ligand_files = Vec::new();
                for entry in std::fs::read_dir(&ligands_path_buf)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            if ext == "pdb" || ext == "mol2" || ext == "sdf" {
                                ligand_files.push(path);
                            }
                        }
                    }
                }
                
                println!("  Found {} ligand files", ligand_files.len());
                
                // Process each ligand
                let receptor_content = std::fs::read_to_string(receptor_path)?;
                let receptor = MoleculeParser::parse_pdb(&receptor_content);
                
                std::fs::create_dir_all(output_dir)?;
                let start_time = std::time::Instant::now();
                
                for (i, ligand_file) in ligand_files.iter().enumerate() {
                    println!("\n  [{}/{}] Processing {} | Elapsed: {:.1}s", 
                        i + 1, ligand_files.len(), 
                        ligand_file.file_name().unwrap().to_string_lossy(),
                        start_time.elapsed().as_secs_f32());
                    
                    match process_single_ligand(&receptor, ligand_file, output_dir, algorithm, 
                        num_poses, max_iterations, continue_on_failure) {
                        Ok(_) => println!("  ✓ Completed"),
                        Err(e) => {
                            if continue_on_failure {
                                println!("  ✗ Failed: {}", e);
                            } else {
                                return Err(e);
                            }
                        }
                    }
                }
                
                println!("\n=== Batch Processing Complete ===");
                println!("  Processed {} ligands in {:.1}s", 
                    ligand_files.len(), start_time.elapsed().as_secs_f32());
            } else {
                return Err("Ligands path must be a file or directory".into());
            }
        },
        
        ("prepare", Some(prepare_matches)) => {
            let input_file = prepare_matches.value_of("input").unwrap();
            let output_file = prepare_matches.value_of("output").unwrap();
            let remove_waters = prepare_matches.is_present("remove_waters");
            let add_hydrogens = prepare_matches.is_present("add_hydrogens");
            
            ReceptorPreparer::prepare(input_file, output_file, remove_waters, add_hydrogens)?;
        },
        
        ("score", Some(score_matches)) => {
            let receptor_file = score_matches.value_of("receptor").unwrap();
            let ligand_file = score_matches.value_of("ligand").unwrap();
            let method = score_matches.value_of("method").unwrap();
            
            ScoringTool::score_pose(receptor_file, ligand_file, method)?;
        },
        
        ("convert", Some(convert_matches)) => {
            let input_file = convert_matches.value_of("input").unwrap();
            let output_file = convert_matches.value_of("output").unwrap();
            let format = convert_matches.value_of("format").unwrap();
            
            FileConverter::convert(input_file, output_file, format)?;
        },
        
        ("info", Some(info_matches)) => {
            let file = info_matches.value_of("file").unwrap();
            MoleculeInfo::show_info(file)?;
        },
        
        ("config", Some(config_matches)) => {
            if config_matches.is_present("show") {
                let config = AppConfig::default();
                println!("=== ZentDock Configuration ===");
                println!("  Version: {}", config.version);
                println!("  Threads: {}", config.num_threads);
                println!("  Default algorithm: {}", config.default_algorithm);
                println!("  Default scoring: {}", config.default_scoring);
                println!("  Default poses: {}", config.default_num_poses);
                println!("  Default iterations: {}", config.default_max_iterations);
                println!("  Output directory: {}", config.output_dir);
            } else if let Some(set_value) = config_matches.value_of("set") {
                println!("  [PLACEHOLDER] Setting config value: {}", set_value);
                println!("  Config setting not fully implemented");
            } else {
                println!("  Use --show to display configuration or --set key=value to set values");
            }
        },
        
        _ => {
            println!("No subcommand provided. Use --help for available commands.");
        }
    }
    
    Ok(())
}

fn process_single_ligand(
    receptor: &zentdock::core::Molecule,
    ligand_file: &std::path::Path,
    output_dir: &str,
    algorithm: &str,
    num_poses: usize,
    max_iterations: usize,
    continue_on_failure: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load ligand
    let ligand_content = std::fs::read_to_string(ligand_file)?;
    let ligand = MoleculeParser::parse_pdb(&ligand_content);
    
    // Configure docking
    let docking_algorithm = match algorithm {
        "genetic" => DockingAlgorithm::Genetic,
        "lamarckian" => DockingAlgorithm::Lamarckian,
        "pso" => DockingAlgorithm::PSO,
        "sa" => DockingAlgorithm::SA,
        "multistage" => DockingAlgorithm::MultiStage,
        _ => return Err("Invalid algorithm".into()),
    };
    
    let config = DockingConfig {
        algorithm: docking_algorithm,
        scoring_function: ScoringFunction::Vina,
        num_poses,
        max_iterations,
        population_size: 50, // Smaller for batch processing
        search_box_size: 10.0,
        real_time_monitoring: false,
        ..Default::default()
    };
    
    // Run docking
    let results = DockingEngine::dock(receptor, &ligand, &config);
    
    // Save results
    let ligand_name = ligand_file.file_stem()
        .unwrap_or_default()
        .to_string_lossy();
    
    // Create subdirectory for this ligand
    let ligand_output_dir = std::path::Path::new(output_dir).join(&*ligand_name);
    std::fs::create_dir_all(&ligand_output_dir)?;
    
    // Save best pose
    if let Some(best_result) = results.first() {
        let complex_pdb = ligand_output_dir.join("best_complex.pdb");
        let complex_content = MoleculeParser::write_complex(receptor, &ligand);
        std::fs::write(&complex_pdb, complex_content)?;
        
        // Save results summary
        let results_file = ligand_output_dir.join("results.txt");
        let mut results_text = String::new();
        results_text.push_str(&format!("Ligand: {}\n", ligand_name));
        results_text.push_str(&format!("Best energy: {:.4} kcal/mol\n", best_result.energy));
        results_text.push_str("All poses:\n");
        
        for (i, result) in results.iter().enumerate() {
            results_text.push_str(&format!("  Rank {}: {:.4} kcal/mol\n", i + 1, result.energy));
        }
        
        std::fs::write(&results_file, results_text)?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_center() {
        let center = parse_center("1.0,2.0,3.0").unwrap();
        assert_eq!(center.x, 1.0);
        assert_eq!(center.y, 2.0);
        assert_eq!(center.z, 3.0);
    }

    #[test]
    fn test_parse_center_invalid() {
        assert!(parse_center("1.0,2.0").is_err());
        assert!(parse_center("a,b,c").is_err());
    }
}
