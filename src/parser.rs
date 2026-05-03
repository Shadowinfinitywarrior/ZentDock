use crate::core::{Molecule, Atom, Vector3D, Element, MoleculeType, Transform, RotationMatrix};

pub struct MoleculeParser;

impl MoleculeParser {
    pub fn parse_pdb(content: &str) -> Molecule {
        let mut molecule = Molecule::new("molecule_1".to_string(), "Parsed Molecule".to_string(), MoleculeType::Other);
        let mut atom_id = 0;
        
        for line in content.lines() {
            if line.starts_with("ATOM") || line.starts_with("HETATM") {
                if let Some(atom) = parse_atom_record(line, atom_id) {
                    molecule.add_atom(atom);
                    atom_id += 1;
                }
            }
        }
        
        // Determine molecule type based on content
        molecule.molecule_type = if molecule.atoms.iter().any(|a| a.is_het) {
            if molecule.atoms.iter().any(|a| a.element == Element::C || a.element == Element::N || a.element == Element::O) {
                MoleculeType::Ligand
            } else {
                MoleculeType::Other
            }
        } else if molecule.atoms.len() > 100 {
            MoleculeType::Protein
        } else {
            MoleculeType::Other
        };
        
        molecule
    }

    pub fn write_pdb(molecule: &Molecule, title: Option<&str>) -> String {
        let mut output = String::new();
        
        if let Some(t) = title {
            output.push_str(&format!("TITLE     {}\n", t));
        }
        
        for atom in &molecule.atoms {
            let charge = if atom.partial_charge != 0.0 { 
                format!("{:+.2}", atom.partial_charge) 
            } else { 
                "  ".to_string() 
            };
            
            output.push_str(&format!(
                "{:6}{:5} {:4} {:3} {:1}{:4}    {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}    {:>2}{:+.2}\n",
                if atom.is_het { "HETATM" } else { "ATOM" },
                atom.id + 1,
                atom.name,
                atom.residue_name,
                atom.chain_id,
                atom.residue_id,
                atom.position.x,
                atom.position.y,
                atom.position.z,
                1.0, // occupancy
                0.0, // B-factor
                atom.element.atomic_number() as u8,
                charge,
            ));
        }
        
        output.push_str("END\n");
        output
    }

    pub fn write_complex_pdbqt(receptor: &Molecule, ligand: &Molecule, transform: &Transform, include_partial_charges: bool) -> String {
        let transformed_ligand = ligand.shifted(transform);
        let mut output = String::new();
        output.push_str("TITLE     Receptor-Ligand Complex (PDBQT)\n");
        
        // Write receptor atoms
        for atom in &receptor.atoms {
            let charge = if include_partial_charges { atom.partial_charge } else { 0.0 };
            output.push_str(&format!(
                "{:6}{:5} {:4} {:3} {:1}{:4}    {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}    {:>2}{:+.2}\n",
                "ATOM",
                atom.id + 1,
                atom.name,
                atom.residue_name,
                atom.chain_id,
                atom.residue_id,
                atom.position.x,
                atom.position.y,
                atom.position.z,
                1.0,
                0.0,
                atom.element.atomic_number() as u8,
                charge,
            ));
        }

        // Write ligand atoms with HETATM
        for atom in &transformed_ligand.atoms {
            let charge = if include_partial_charges { atom.partial_charge } else { 0.0 };
            output.push_str(&format!(
                "{:6}{:5} {:4} {:3} {:1}{:4}    {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}    {:>2}{:+.2}\n",
                "HETATM",
                atom.id + receptor.num_atoms() + 1,
                atom.name,
                atom.residue_name,
                atom.chain_id,
                atom.residue_id,
                atom.position.x,
                atom.position.y,
                atom.position.z,
                1.0,
                0.0,
                atom.element.atomic_number() as u8,
                charge,
            ));
        }
        
        output.push_str("END\n");
        output
    }

    pub fn write_complex(receptor: &Molecule, ligand: &Molecule) -> String {
        let mut output = Self::write_pdb(receptor, Some("Receptor"));
        output.push_str("\n");
        output.push_str(&Self::write_pdb(ligand, Some("Ligand")));
        output
    }
}

fn parse_atom_record(line: &str, atom_id: usize) -> Option<Atom> {
    if line.len() < 54 {
        return None;
    }
    
    let is_het = line.starts_with("HETATM");
    let name = line[12..16].trim().to_string();
    let res_name = line[17..20].trim().to_string();
    let chain_id = line[21..22].to_string();
    let res_id = line[22..26].trim().parse::<usize>().unwrap_or(0);
    
    let x = line[30..38].trim().parse::<f64>().unwrap_or(0.0);
    let y = line[38..46].trim().parse::<f64>().unwrap_or(0.0);
    let z = line[46..54].trim().parse::<f64>().unwrap_or(0.0);
    
    let element_str = if line.len() >= 78 {
        line[76..78].trim()
    } else {
        ""
    };
    
    let element = if element_str.is_empty() {
        if name.starts_with('C') { Element::C }
        else if name.starts_with('N') { Element::N }
        else if name.starts_with('O') { Element::O }
        else if name.starts_with('S') { Element::S }
        else if name.starts_with('H') { Element::H }
        else { Element::C }
    } else {
        match element_str.to_uppercase().as_str() {
            "C" => Element::C,
            "N" => Element::N,
            "O" => Element::O,
            "S" => Element::S,
            "H" => Element::H,
            "P" => Element::P,
            "F" => Element::F,
            "CL" => Element::Cl,
            "BR" => Element::Br,
            "I" => Element::I,
            _ => Element::C,
        }
    };
    
    let mut atom = Atom::new(atom_id, name, element, Vector3D::new(x, y, z));
    atom.residue_name = res_name;
    atom.residue_id = res_id;
    atom.chain_id = chain_id;
    atom.is_het = is_het;
    
    Some(atom)
}
