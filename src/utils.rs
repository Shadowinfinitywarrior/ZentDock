use crate::core::{Molecule, Element};
use std::path::Path;

pub struct JobManager;
pub struct StructurePreparer;
pub struct RMSDCalculator;

impl JobManager {
    pub fn submit_docking(receptor: Molecule, ligand: Molecule, config: crate::DockingConfig) -> Vec<crate::DockingResult> {
        crate::docking::DockingEngine::dock(&receptor, &ligand, &config)
    }

    pub fn create_complex(receptor: &Molecule, ligand: &Molecule, transform: &crate::Transform) -> String {
        crate::parser::MoleculeParser::write_complex_pdbqt(receptor, ligand, transform, true)
    }
}

impl StructurePreparer {
    pub fn prepare(input_file: &str, output_file: &str, remove_waters: bool, add_hydrogens: bool) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(input_file)?;
        let mut molecule = crate::parser::MoleculeParser::parse_pdb(&content);
        
        if remove_waters {
            molecule.atoms.retain(|atom| !is_water_molecule(atom));
        }
        
        if add_hydrogens {
            add_hydrogens_to_molecule(&mut molecule);
        }
        
        let pdb_content = crate::parser::MoleculeParser::write_pdb(&molecule, Some("Prepared Receptor"));
        std::fs::write(output_file, pdb_content)?;
        
        Ok(())
    }
}

impl RMSDCalculator {
    pub fn calculate_rmsd(mol1: &Molecule, mol2: &Molecule) -> f64 {
        if mol1.atoms.len() != mol2.atoms.len() {
            return f64::INFINITY;
        }
        
        let mut sum_sq = 0.0;
        for (atom1, atom2) in mol1.atoms.iter().zip(mol2.atoms.iter()) {
            if atom1.element == atom2.element {
                let diff = atom1.position.distance(&atom2.position);
                sum_sq += diff * diff;
            }
        }
        
        (sum_sq / mol1.atoms.len() as f64).sqrt()
    }
}

fn is_water_molecule(atom: &crate::core::Atom) -> bool {
    atom.residue_name.to_uppercase() == "HOH" || atom.residue_name.to_uppercase() == "WAT"
}

fn add_hydrogens_to_molecule(molecule: &mut Molecule) {
    let mut new_atoms = Vec::new();
    let mut atom_id = molecule.atoms.len();
    
    for atom in &molecule.atoms {
        match atom.element {
            Element::N => {
                // Add one hydrogen to nitrogen (simplified)
                let h_pos = atom.position.add(&crate::core::Vector3D::new(0.0, 0.0, 1.0));
                let h_atom = crate::core::Atom::new(atom_id, "H".to_string(), Element::H, h_pos);
                new_atoms.push(h_atom);
                atom_id += 1;
            }
            Element::O => {
                // Add one hydrogen to oxygen (simplified)
                let h_pos = atom.position.add(&crate::core::Vector3D::new(0.0, 0.0, 1.0));
                let h_atom = crate::core::Atom::new(atom_id, "H".to_string(), Element::H, h_pos);
                new_atoms.push(h_atom);
                atom_id += 1;
            }
            _ => {}
        }
    }
    
    molecule.atoms.extend(new_atoms);
}
