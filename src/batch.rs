use crate::core::{Molecule, DockingConfig};
use std::path::Path;

pub struct BatchProcessor;
pub struct MultiTargetProcessor;

impl BatchProcessor {
    pub fn process(
        receptor: &Molecule,
        ligands_path: &Path,
        output_dir: &str,
        config: &DockingConfig,
        continue_on_failure: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ligand_files = Self::find_ligand_files(ligands_path)?;
        
        for (i, ligand_file) in ligand_files.iter().enumerate() {
            println!("Processing ligand {}/{}: {}", i + 1, ligand_files.len(), ligand_file.display());
            
            match Self::process_single_ligand(receptor, ligand_file, output_dir, config) {
                Ok(_) => println!("  ✓ Completed"),
                Err(e) => {
                    println!("  ✗ Failed: {}", e);
                    if !continue_on_failure {
                        return Err(e);
                    }
                }
            }
        }
        
        Ok(())
    }

    fn find_ligand_files(path: &Path) -> Result<Vec<std::path::PathBuf>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();
        
        if path.is_file() {
            files.push(path.to_path_buf());
        } else if path.is_dir() {
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "pdb" || ext == "mol2" || ext == "sdf" {
                            files.push(path);
                        }
                    }
                }
            }
        }
        
        Ok(files)
    }

    fn process_single_ligand(
        receptor: &Molecule,
        ligand_file: &std::path::Path,
        output_dir: &str,
        config: &DockingConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Load ligand
        let ligand_content = std::fs::read_to_string(ligand_file)?;
        let ligand = crate::parser::MoleculeParser::parse_pdb(&ligand_content);

        // Run docking
        let results = crate::docking::DockingEngine::dock(receptor, &ligand, config);

        // Save results
        let ligand_name = ligand_file.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("ligand");

        for (i, result) in results.iter().take(config.num_poses).enumerate() {
            let pose_file = format!("{}/{}_pose_{}.pdb", output_dir, ligand_name, i + 1);
            let transformed_ligand = ligand.shifted(&result.transform);
            let pose_content = crate::parser::MoleculeParser::write_pdb(&transformed_ligand, 
                Some(&format!("Pose {} - Energy: {:.4}", i + 1, result.energy)));
            std::fs::write(&pose_file, pose_content)?;
        }

        Ok(())
    }
}

impl MultiTargetProcessor {
    pub fn process(
        receptors: &[Molecule],
        ligands: &[Molecule],
        config: &DockingConfig,
    ) -> Vec<(String, Vec<crate::DockingResult>)> {
        let mut all_results = Vec::new();

        for (i, receptor) in receptors.iter().enumerate() {
            for (j, ligand) in ligands.iter().enumerate() {
                let job_id = format!("rec_{}_lig_{}", i, j);
                println!("Processing {}...", job_id);
                
                let results = crate::docking::DockingEngine::dock(receptor, ligand, config);
                all_results.push((job_id, results));
            }
        }

        all_results
    }
}
