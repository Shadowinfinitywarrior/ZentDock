use crate::{ScoringFunction, DockingAlgorithm, Transform, DockingConfig};
use crate::parser::MoleculeParser;
use crate::docking::DockingEngine;

pub struct ScoringTool;

impl ScoringTool {
    pub fn score_pose(receptor_file: &str, ligand_file: &str, method: &str) -> Result<(), Box<dyn std::error::Error>> {
        let receptor_content = std::fs::read_to_string(receptor_file)?;
        let receptor = MoleculeParser::parse_pdb(&receptor_content);
        
        let ligand_content = std::fs::read_to_string(ligand_file)?;
        let ligand = MoleculeParser::parse_pdb(&ligand_content);
        
        let scoring_function = match method.to_lowercase().as_str() {
            "autodock4" => ScoringFunction::AutoDock4,
            "vina" => ScoringFunction::Vina,
            "chemscore" => ScoringFunction::ChemScore,
            _ => ScoringFunction::Vina,
        };
        
        let mut config = DockingConfig::default();
        config.scoring_function = scoring_function;
        
        let transform = Transform::identity();
        let energy = DockingEngine::score_with_function(&receptor, &ligand, &transform, &config.scoring_function);
        
        println!("Scoring results for method: {}", method);
        println!("  Energy: {:.4} kcal/mol", energy);
        
        Ok(())
    }
}
