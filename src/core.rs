use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    H, He, Li, Be, B, C, N, O, F, Ne, Na, Mg, Al, Si, P, S, Cl, Ar, K, Ca,
    Sc, Ti, V, Cr, Mn, Fe, Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr, Rb, Sr,
    Y, Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn, Sb, Te, I, Xe, Cs, Ba,
    La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu, Hf, Ta,
    W, Re, Os, Ir, Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn, Fr, Ra, Ac, Th,
    Pa, U, Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No, Lr,
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::H => write!(f, "H"),
            Element::He => write!(f, "He"),
            Element::Li => write!(f, "Li"),
            Element::Be => write!(f, "Be"),
            Element::B => write!(f, "B"),
            Element::C => write!(f, "C"),
            Element::N => write!(f, "N"),
            Element::O => write!(f, "O"),
            Element::F => write!(f, "F"),
            Element::Ne => write!(f, "Ne"),
            Element::Na => write!(f, "Na"),
            Element::Mg => write!(f, "Mg"),
            Element::Al => write!(f, "Al"),
            Element::Si => write!(f, "Si"),
            Element::P => write!(f, "P"),
            Element::S => write!(f, "S"),
            Element::Cl => write!(f, "Cl"),
            Element::Ar => write!(f, "Ar"),
            Element::K => write!(f, "K"),
            Element::Ca => write!(f, "Ca"),
            Element::Sc => write!(f, "Sc"),
            Element::Ti => write!(f, "Ti"),
            Element::V => write!(f, "V"),
            Element::Cr => write!(f, "Cr"),
            Element::Mn => write!(f, "Mn"),
            Element::Fe => write!(f, "Fe"),
            Element::Co => write!(f, "Co"),
            Element::Ni => write!(f, "Ni"),
            Element::Cu => write!(f, "Cu"),
            Element::Zn => write!(f, "Zn"),
            Element::Ga => write!(f, "Ga"),
            Element::Ge => write!(f, "Ge"),
            Element::As => write!(f, "As"),
            Element::Se => write!(f, "Se"),
            Element::Br => write!(f, "Br"),
            Element::Kr => write!(f, "Kr"),
            Element::Rb => write!(f, "Rb"),
            Element::Sr => write!(f, "Sr"),
            Element::Y => write!(f, "Y"),
            Element::Zr => write!(f, "Zr"),
            Element::Nb => write!(f, "Nb"),
            Element::Mo => write!(f, "Mo"),
            Element::Tc => write!(f, "Tc"),
            Element::Ru => write!(f, "Ru"),
            Element::Rh => write!(f, "Rh"),
            Element::Pd => write!(f, "Pd"),
            Element::Ag => write!(f, "Ag"),
            Element::Cd => write!(f, "Cd"),
            Element::In => write!(f, "In"),
            Element::Sn => write!(f, "Sn"),
            Element::Sb => write!(f, "Sb"),
            Element::Te => write!(f, "Te"),
            Element::I => write!(f, "I"),
            Element::Xe => write!(f, "Xe"),
            Element::Cs => write!(f, "Cs"),
            Element::Ba => write!(f, "Ba"),
            Element::La => write!(f, "La"),
            Element::Ce => write!(f, "Ce"),
            Element::Pr => write!(f, "Pr"),
            Element::Nd => write!(f, "Nd"),
            Element::Pm => write!(f, "Pm"),
            Element::Sm => write!(f, "Sm"),
            Element::Eu => write!(f, "Eu"),
            Element::Gd => write!(f, "Gd"),
            Element::Tb => write!(f, "Tb"),
            Element::Dy => write!(f, "Dy"),
            Element::Ho => write!(f, "Ho"),
            Element::Er => write!(f, "Er"),
            Element::Tm => write!(f, "Tm"),
            Element::Yb => write!(f, "Yb"),
            Element::Lu => write!(f, "Lu"),
            Element::Hf => write!(f, "Hf"),
            Element::Ta => write!(f, "Ta"),
            Element::W => write!(f, "W"),
            Element::Re => write!(f, "Re"),
            Element::Os => write!(f, "Os"),
            Element::Ir => write!(f, "Ir"),
            Element::Pt => write!(f, "Pt"),
            Element::Au => write!(f, "Au"),
            Element::Hg => write!(f, "Hg"),
            Element::Tl => write!(f, "Tl"),
            Element::Pb => write!(f, "Pb"),
            Element::Bi => write!(f, "Bi"),
            Element::Po => write!(f, "Po"),
            Element::At => write!(f, "At"),
            Element::Rn => write!(f, "Rn"),
            Element::Fr => write!(f, "Fr"),
            Element::Ra => write!(f, "Ra"),
            Element::Ac => write!(f, "Ac"),
            Element::Th => write!(f, "Th"),
            Element::Pa => write!(f, "Pa"),
            Element::U => write!(f, "U"),
            Element::Np => write!(f, "Np"),
            Element::Pu => write!(f, "Pu"),
            Element::Am => write!(f, "Am"),
            Element::Cm => write!(f, "Cm"),
            Element::Bk => write!(f, "Bk"),
            Element::Cf => write!(f, "Cf"),
            Element::Es => write!(f, "Es"),
            Element::Fm => write!(f, "Fm"),
            Element::Md => write!(f, "Md"),
            Element::No => write!(f, "No"),
            Element::Lr => write!(f, "Lr"),
        }
    }
}

impl Element {
    pub fn atomic_number(&self) -> u8 {
        match self {
            Element::H => 1, Element::He => 2, Element::Li => 3, Element::Be => 4, Element::B => 5,
            Element::C => 6, Element::N => 7, Element::O => 8, Element::F => 9, Element::Ne => 10,
            Element::Na => 11, Element::Mg => 12, Element::Al => 13, Element::Si => 14, Element::P => 15,
            Element::S => 16, Element::Cl => 17, Element::Ar => 18, Element::K => 19, Element::Ca => 20,
            Element::Fe => 26, Element::Cu => 29, Element::Zn => 30, Element::Br => 35, Element::I => 53,
            _ => 6, // Default to Carbon for others
        }
    }

    pub fn vdw_radius(&self) -> f64 {
        match self {
            Element::H => 1.20, Element::He => 1.40, Element::Li => 1.82, Element::Be => 1.53, Element::B => 1.92,
            Element::C => 1.70, Element::N => 1.55, Element::O => 1.52, Element::F => 1.47, Element::Ne => 1.54,
            Element::Na => 2.27, Element::Mg => 1.73, Element::Al => 1.84, Element::Si => 2.10, Element::P => 1.80,
            Element::S => 1.80, Element::Cl => 1.75, Element::Ar => 1.88, Element::K => 2.75, Element::Ca => 2.31,
            _ => 1.70,
        }
    }

    pub fn mass(&self) -> f64 {
        match self {
            Element::H => 1.008, Element::He => 4.003, Element::Li => 6.941, Element::Be => 9.012, Element::B => 10.811,
            Element::C => 12.011, Element::N => 14.007, Element::O => 15.999, Element::F => 18.998, Element::Ne => 20.180,
            Element::Na => 22.990, Element::Mg => 24.305, Element::Al => 26.982, Element::Si => 28.086, Element::P => 30.974,
            Element::S => 32.065, Element::Cl => 35.453, Element::Ar => 39.948, Element::K => 39.098, Element::Ca => 40.078,
            _ => 12.011,
        }
    }

    pub fn is_hydrogen(&self) -> bool {
        matches!(self, Element::H)
    }

    pub fn is_hb_acceptor(&self) -> bool {
        matches!(self, Element::N | Element::O | Element::S | Element::F)
    }

    pub fn is_hb_donor(&self) -> bool {
        matches!(self, Element::N | Element::O)
    }

    pub fn max_bonds(&self) -> u8 {
        match self {
            Element::H => 1,
            Element::C => 4,
            Element::N => 3,
            Element::O => 2,
            Element::S => 6,
            Element::P => 5,
            _ => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn distance(&self, other: &Vector3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn scale(&self, factor: f64) -> Vector3D {
        Vector3D::new(self.x * factor, self.y * factor, self.z * factor)
    }

    pub fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn subtract(&self, other: &Vector3D) -> Vector3D {
        Vector3D::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn normalize(&self) -> Vector3D {
        let len = self.length();
        if len > 0.0 {
            Vector3D::new(self.x / len, self.y / len, self.z / len)
        } else {
            Vector3D::new(0.0, 0.0, 0.0)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Atom {
    pub id: usize,
    pub name: String,
    pub element: Element,
    pub position: Vector3D,
    pub charge: f64,
    pub mass: f64,
    pub radius: f64,
    pub partial_charge: f64,
    pub residue_name: String,
    pub residue_id: usize,
    pub chain_id: String,
    pub is_het: bool,
}

impl Atom {
    pub fn new(id: usize, name: String, element: Element, position: Vector3D) -> Self {
        Self {
            id,
            name,
            element,
            position,
            charge: 0.0,
            mass: element.mass(),
            radius: element.vdw_radius(),
            partial_charge: 0.0,
            residue_name: "UNK".to_string(),
            residue_id: 0,
            chain_id: "A".to_string(),
            is_het: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BondType {
    Single,
    Double,
    Triple,
    Aromatic,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bond {
    pub atom1: usize,
    pub atom2: usize,
    pub bond_type: BondType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MoleculeType {
    Protein,
    Ligand,
    NucleicAcid,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Molecule {
    pub id: String,
    pub name: String,
    pub molecule_type: MoleculeType,
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
}

impl Molecule {
    pub fn new(id: String, name: String, molecule_type: MoleculeType) -> Self {
        Self {
            id,
            name,
            molecule_type,
            atoms: Vec::new(),
            bonds: Vec::new(),
        }
    }

    pub fn add_atom(&mut self, atom: Atom) {
        self.atoms.push(atom);
    }

    pub fn num_atoms(&self) -> usize {
        self.atoms.len()
    }

    pub fn heavy_atom_count(&self) -> usize {
        self.atoms.iter().filter(|atom| !atom.element.is_hydrogen()).count()
    }

    pub fn center_of_mass(&self) -> Vector3D {
        if self.atoms.is_empty() {
            return Vector3D::new(0.0, 0.0, 0.0);
        }

        let mut total_mass = 0.0;
        let mut com = Vector3D::new(0.0, 0.0, 0.0);

        for atom in &self.atoms {
            com = com.add(&atom.position.scale(atom.mass));
            total_mass += atom.mass;
        }

        com.scale(1.0 / total_mass)
    }

    pub fn bounding_box(&self) -> (Vector3D, Vector3D) {
        if self.atoms.is_empty() {
            return (Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 0.0));
        }

        let mut min = self.atoms[0].position;
        let mut max = self.atoms[0].position;

        for atom in &self.atoms {
            if atom.position.x < min.x { min.x = atom.position.x; }
            if atom.position.y < min.y { min.y = atom.position.y; }
            if atom.position.z < min.z { min.z = atom.position.z; }
            if atom.position.x > max.x { max.x = atom.position.x; }
            if atom.position.y > max.y { max.y = atom.position.y; }
            if atom.position.z > max.z { max.z = atom.position.z; }
        }

        (min, max)
    }

    pub fn shifted(&self, transform: &crate::Transform) -> Molecule {
        let mut shifted = self.clone();
        for atom in &mut shifted.atoms {
            atom.position = transform.apply(&atom.position);
        }
        shifted
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DockingAlgorithm {
    Genetic,
    Lamarckian,
    PSO,
    SA,
    MultiStage,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScoringFunction {
    AutoDock4,
    Vina,
    ChemScore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockingConfig {
    pub algorithm: DockingAlgorithm,
    pub scoring_function: ScoringFunction,
    pub num_poses: usize,
    pub max_iterations: usize,
    pub population_size: usize,
    pub search_box_size: f64,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub random_seed: Option<u64>,
    pub real_time_monitoring: bool,
    pub streaming_output: bool,
    pub checkpoint_interval: usize,
    pub rmsd_clustering: bool,
    pub convergence_threshold: f64,
}

impl Default for DockingConfig {
    fn default() -> Self {
        Self {
            algorithm: DockingAlgorithm::Genetic,
            scoring_function: ScoringFunction::Vina,
            num_poses: 10,
            max_iterations: 25000,
            population_size: 150,
            search_box_size: 10.0,
            mutation_rate: 0.1,
            crossover_rate: 0.8,
            random_seed: None,
            real_time_monitoring: false,
            streaming_output: false,
            checkpoint_interval: 0,
            rmsd_clustering: false,
            convergence_threshold: 0.01,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub rotation: RotationMatrix,
    pub translation: Vector3D,
}

impl Transform {
    pub fn new(rotation: RotationMatrix, translation: Vector3D) -> Self {
        Self { rotation, translation }
    }

    pub fn identity() -> Self {
        Self {
            rotation: RotationMatrix::identity(),
            translation: Vector3D::new(0.0, 0.0, 0.0),
        }
    }

    pub fn apply(&self, vector: &Vector3D) -> Vector3D {
        let rotated = self.rotation.apply(vector);
        rotated.add(&self.translation)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RotationMatrix {
    matrix: [[f64; 3]; 3],
}

impl RotationMatrix {
    pub fn identity() -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn from_euler(roll: f64, pitch: f64, yaw: f64) -> Self {
        let cr = roll.cos();
        let sr = roll.sin();
        let cp = pitch.cos();
        let sp = pitch.sin();
        let cy = yaw.cos();
        let sy = yaw.sin();

        Self {
            matrix: [
                [cy * cp, cy * sp * sr - sy * cr, cy * sp * cr + sy * sr],
                [sy * cp, sy * sp * sr + cy * cr, sy * sp * cr - cy * sr],
                [-sp, cp * sr, cp * cr],
            ],
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
        let pitch = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
        let yaw = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
        Self::from_euler(roll, pitch, yaw)
    }

    pub fn apply(&self, vector: &Vector3D) -> Vector3D {
        Vector3D::new(
            self.matrix[0][0] * vector.x + self.matrix[0][1] * vector.y + self.matrix[0][2] * vector.z,
            self.matrix[1][0] * vector.x + self.matrix[1][1] * vector.y + self.matrix[1][2] * vector.z,
            self.matrix[2][0] * vector.x + self.matrix[2][1] * vector.y + self.matrix[2][2] * vector.z,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockingResult {
    pub rank: usize,
    pub energy: f64,
    pub transform: Transform,
    pub rmsd: f64,
    pub cluster_id: usize,
    pub cluster_size: usize,
    pub ligand_atoms: Vec<Atom>,
}
