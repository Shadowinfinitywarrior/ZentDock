use crate::core::{Molecule, Vector3D, Transform, Element};

pub struct AutoDock4Scoring;

impl AutoDock4Scoring {
    pub fn calculate(receptor: &Molecule, ligand: &Molecule, transform: &Transform) -> f64 {
        let transformed_ligand = ligand.shifted(transform);
        
        let mut vdw_energy = 0.0;
        let mut electrostatic_energy = 0.0;
        let mut hb_energy = 0.0;
        let mut desolvation_energy = 0.0;

        // Van der Waals and Electrostatic interactions
        for rec_atom in &receptor.atoms {
            for lig_atom in &transformed_ligand.atoms {
                let distance = rec_atom.position.distance(&lig_atom.position);
                
                if distance < 12.0 && distance > 0.1 {
                    // Van der Waals (Lennard-Jones 12-6)
                    let sigma = (rec_atom.element.vdw_radius() + lig_atom.element.vdw_radius()) / 1.122;
                    let epsilon = 0.15; // kcal/mol
                    
                    if distance < sigma * 3.0 {
                        let sr6 = (sigma / distance).powi(6);
                        let sr12 = sr6 * sr6;
                        vdw_energy += epsilon * (sr12 - 2.0 * sr6);
                    }
                    
                    // Electrostatic
                    let dielectric = 4.0;
                    electrostatic_energy += 332.0 * rec_atom.charge * lig_atom.charge / (dielectric * distance);
                    
                    // Hydrogen bonding
                    if (rec_atom.element.is_hb_donor() && lig_atom.element.is_hb_acceptor()) ||
                       (rec_atom.element.is_hb_acceptor() && lig_atom.element.is_hb_donor()) {
                        if distance >= 2.0 && distance <= 4.0 {
                            hb_energy -= 1.0; // Simple H-bond energy
                        }
                    }
                }
            }
        }

        // Desolvation (simplified)
        for lig_atom in &transformed_ligand.atoms {
            let mut close_heavy_atoms = 0;
            for rec_atom in &receptor.atoms {
                if !rec_atom.element.is_hydrogen() {
                    let distance = lig_atom.position.distance(&rec_atom.position);
                    if distance < 6.0 {
                        close_heavy_atoms += 1;
                    }
                }
            }
            if lig_atom.element.is_hb_acceptor() || lig_atom.element.is_hb_donor() {
                desolvation_energy += 0.1 * close_heavy_atoms as f64;
            }
        }

        vdw_energy + electrostatic_energy + hb_energy + desolvation_energy
    }
}

pub struct VinaScoring;

impl VinaScoring {
    pub fn calculate(receptor: &Molecule, ligand: &Molecule, transform: &Transform, _flexible_residue: Option<&Molecule>) -> f64 {
        let transformed_ligand = ligand.shifted(transform);
        
        let mut gauss1 = 0.0;
        let mut gauss2 = 0.0;
        let mut repulsion = 0.0;
        let mut hydrophobic = 0.0;
        let mut hydrogen = 0.0;

        // Vina atom type mapping (simplified)
        for rec_atom in &receptor.atoms {
            for lig_atom in &transformed_ligand.atoms {
                let distance = rec_atom.position.distance(&lig_atom.position);
                
                if distance > 0.1 && distance < 8.0 {
                    let rec_type = Self::get_vina_type(&rec_atom.element);
                    let lig_type = Self::get_vina_type(&lig_atom.element);
                    
                    // Gaussian attractive potentials
                    gauss1 += Self::gaussian(distance, 0.0, 0.5, rec_type, lig_type);
                    gauss2 += Self::gaussian(distance, 3.0, 2.0, rec_type, lig_type);
                    
                    // Repulsion
                    if distance < 0.5 {
                        repulsion += (0.5 - distance).powi(12);
                    }
                    
                    // Hydrophobic
                    if Self::is_hydrophobic(&rec_atom.element) && Self::is_hydrophobic(&lig_atom.element) {
                        if distance < 4.0 {
                            hydrophobic += 1.0;
                        }
                    }
                    
                    // Hydrogen bonding
                    if (rec_atom.element.is_hb_donor() && lig_atom.element.is_hb_acceptor()) ||
                       (rec_atom.element.is_hb_acceptor() && lig_atom.element.is_hb_donor()) {
                        if distance >= 2.0 && distance <= 3.5 {
                            hydrogen -= 1.0;
                        }
                    }
                }
            }
        }

        // Vina weights (simplified)
        -0.035579 * gauss1 - 0.005156 * gauss2 + 0.840245 * repulsion + -0.035069 * hydrophobic + -0.587439 * hydrogen
    }

    fn get_vina_type(element: &Element) -> usize {
        match element {
            Element::C => 0,
            Element::N => 1,
            Element::O => 2,
            Element::S => 3,
            Element::P => 4,
            _ => 5,
        }
    }

    fn gaussian(r: f64, offset: f64, width: f64, type1: usize, type2: usize) -> f64 {
        if type1 == type2 {
            (-((r - offset) / width).powi(2)).exp()
        } else {
            0.0
        }
    }

    fn is_hydrophobic(element: &Element) -> bool {
        matches!(element, Element::C | Element::S)
    }
}
