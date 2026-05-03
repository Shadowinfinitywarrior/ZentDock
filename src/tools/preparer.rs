use crate::core::{Molecule, Element, Atom, Vector3D};
use crate::parser::MoleculeParser;

pub struct ReceptorPreparer;

impl ReceptorPreparer {
    pub fn prepare(input_file: &str, output_file: &str, remove_waters: bool, add_hydrogens: bool) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(input_file)?;
        let mut molecule = MoleculeParser::parse_pdb(&content);
        
        if remove_waters {
            molecule.atoms.retain(|atom| !is_water_molecule(atom));
        }
        
        if add_hydrogens {
            add_hydrogens_to_molecule(&mut molecule);
        }
        
        let pdb_content = MoleculeParser::write_pdb(&molecule, Some("Prepared Receptor"));
        std::fs::write(output_file, pdb_content)?;
        
        Ok(())
    }
}

fn is_water_molecule(atom: &Atom) -> bool {
    atom.residue_name.to_uppercase() == "HOH" || atom.residue_name.to_uppercase() == "WAT"
}

fn add_hydrogens_to_molecule(molecule: &mut Molecule) {
    let mut new_atoms = Vec::new();
    let mut atom_id = molecule.atoms.len();
    
    for atom in &molecule.atoms {
        match atom.element {
            Element::N => {
                let h_pos = atom.position.add(&Vector3D::new(0.0, 0.0, 1.0));
                let h_atom = Atom::new(atom_id, "H".to_string(), Element::H, h_pos);
                new_atoms.push(h_atom);
                atom_id += 1;
            }
            Element::O => {
                let h_pos = atom.position.add(&Vector3D::new(0.0, 0.0, 1.0));
                let h_atom = Atom::new(atom_id, "H".to_string(), Element::H, h_pos);
                new_atoms.push(h_atom);
                atom_id += 1;
            }
            _ => {}
        }
    }
    
    molecule.atoms.extend(new_atoms);
}
