use crate::parser::MoleculeParser;

pub struct FileConverter;

impl FileConverter {
    pub fn convert(input_file: &str, output_file: &str, format: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(input_file)?;
        let molecule = MoleculeParser::parse_pdb(&content);
        
        let output_content = match format.to_lowercase().as_str() {
            "pdb" => MoleculeParser::write_pdb(&molecule, Some("Converted Molecule")),
            "pdbqt" => crate::parser::MoleculeParser::write_complex_pdbqt(&molecule, &molecule, &crate::core::Transform::identity(), false),
            _ => return Err(format!("Unsupported output format: {}", format).into()),
        };
        
        std::fs::write(output_file, output_content)?;
        println!("Successfully converted {} to {} (format: {})", input_file, output_file, format);
        
        Ok(())
    }
}
