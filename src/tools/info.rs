use crate::parser::MoleculeParser;

pub struct MoleculeInfo;

impl MoleculeInfo {
    pub fn show_info(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;
        let molecule = MoleculeParser::parse_pdb(&content);
        
        println!("Molecule Information for: {}", file_path);
        println!("  Name: {}", molecule.name);
        println!("  Atoms: {}", molecule.atoms.len());
        println!("  Heavy Atoms: {}", molecule.heavy_atom_count());
        println!("  Type: {:?}", molecule.molecule_type);
        
        let center = molecule.center_of_mass();
        println!("  Center of Mass: ({:.3}, {:.3}, {:.3})", center.x, center.y, center.z);
        
        Ok(())
    }
}
