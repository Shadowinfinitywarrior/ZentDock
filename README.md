# ![ZentDock Logo](logo.png) ZentDock - Comprehensive Molecular Docking Software

**Developer:** Mr. Nithish Kathiravan  
**Contact:** nithishkathiravan123@gmail.com / infonity404@gmail.com  
**Phone/WhatsApp:** +91 9342358022  
**GitHub:** [infonity404](https://github.com/infonity404)

## Overview

ZentDock is a high-performance molecular docking software written in Rust that provides comprehensive docking capabilities for computational chemistry and drug discovery. It supports multiple docking algorithms, scoring functions, and file formats with a focus on speed, accuracy, and ease of use.

**Current Status:** ✅ **Actively Under Development** - Core functionality implemented, undergoing final testing and optimization.

## Key Features

### 🔬 Docking Algorithms
- **Genetic Algorithm (GA)**: Population-based evolutionary search with tournament selection, crossover, and mutation
- **Lamarckian GA**: Enhanced genetic algorithm with local optimization for faster convergence
- **Particle Swarm Optimization (PSO)**: Nature-inspired swarm intelligence algorithm
- **Simulated Annealing (SA)**: Thermodynamic-based search with adaptive cooling
- **Multi-stage Docking**: Hybrid approach combining multiple algorithms for optimal results

### ⚡ Scoring Functions
- **AutoDock4**: Classic scoring with Van der Waals, electrostatic, hydrogen bonding, and desolvation
- **Vina**: Empirical scoring with optimized atom type potentials
- **ChemScore**: Chemical scoring function for diverse molecular interactions

### 📁 File Format Support
- **PDB**: Protein Data Bank format (primary)
- **MOL2**: Tripos molecular format
- **SDF**: MDL Structure Data File format
- **XYZ**: Cartesian coordinate format

### 🚀 Advanced Features
- **Real-time Monitoring**: Live progress tracking with energy convergence
- **Batch Processing**: High-throughput screening of multiple ligands
- **Multi-target Screening**: Simultaneous docking against multiple receptors
- **Pose Storage**: Automatic storage and management of best poses
- **Receptor Preparation**: Automated structure preparation (water removal, hydrogen addition)
- **Configuration Management**: Flexible configuration system with TOML support

## Installation

### Prerequisites
- Rust 1.75 or higher
- Cargo package manager (included with Rust)

### Build from Source
```bash
# Clone or navigate to the project directory
cd /home/darkdevil404/Docking

# Build the release version
cargo build --release

# Verify installation
./target/release/zentdock --help
```

### System Requirements
- **RAM**: Minimum 2GB, recommended 4GB+ for large molecules
- **Storage**: 100MB for binary, additional space for input/output files
- **CPU**: Multi-core recommended for batch processing

### Repository
- **GitHub Repository**: https://github.com/Shadowinfinitywarrior/ZentDock.git
- **Version**: 1.0.0 (Development)
- **Build Status**: ✅ Compiles successfully with Rust 1.75+
- **Last Updated**: May 2026

## Quick Start

### Basic Docking Example
```bash
# Simple protein-ligand docking
./target/release/zentdock dock \
  --receptor protein.pdb \
  --ligand ligand.pdb \
  --num-poses 10 \
  --algorithm genetic \
  --scoring vina
```

### Advanced Docking with Custom Parameters
```bash
# High-precision docking with monitoring
./target/release/zentdock dock \
  --receptor receptor.pdb \
  --ligand ligand.pdb \
  --num-poses 20 \
  --algorithm multistage \
  --scoring autodock4 \
  --center "15.2,22.1,18.7" \
  --radius 12.0 \
  --max-iterations 50000 \
  --population-size 200 \
  --monitor \
  --output results/
```

## Comprehensive Usage Guide

### 1. Docking Commands

#### Genetic Algorithm Docking
```bash
./target/release/zentdock dock \
  -r receptor.pdb \
  -l ligand.pdb \
  --algorithm genetic \
  --num-poses 15 \
  --max-iterations 30000
```

#### Lamarckian GA with Local Optimization
```bash
./target/release/zentdock dock \
  -r receptor.pdb \
  -l ligand.pdb \
  --algorithm lamarckian \
  --population-size 100 \
  --monitor
```

#### Particle Swarm Optimization
```bash
./target/release/zentdock dock \
  -r receptor.pdb \
  -l ligand.pdb \
  --algorithm pso \
  --max-iterations 20000
```

#### Simulated Annealing
```bash
./target/release/zentdock dock \
  -r receptor.pdb \
  -l ligand.pdb \
  --algorithm sa \
  --max-iterations 10000
```

#### Multi-stage Docking (Recommended)
```bash
./target/release/zentdock dock \
  -r receptor.pdb \
  -l ligand.pdb \
  --algorithm multistage \
  --num-poses 25 \
  --monitor
```

### 2. Batch Processing

#### Process Multiple Ligands
```bash
./target/release/zentdock batch \
  --receptor receptor.pdb \
  --ligands ligands_directory/ \
  --jobs 4 \
  --output batch_results/ \
  --algorithm genetic \
  --continue-on-failure
```

#### Batch with Custom Parameters
```bash
./target/release/zentdock batch \
  -r receptor.pdb \
  -l ligand_library.sdf \
  --jobs 8 \
  -o screening_results/ \
  --num-poses 5 \
  --max-iterations 15000 \
  --algorithm pso
```

### 3. Molecule Preparation

#### Receptor Preparation
```bash
./target/release/zentdock prepare \
  --input raw_protein.pdb \
  --output prepared_protein.pdb \
  --remove-waters \
  --add-hydrogens
```

#### Ligand Preparation
```bash
./target/release/zentdock prepare \
  --input ligand.mol2 \
  --output prepared_ligand.pdb \
  --add-hydrogens \
  --protonate
```

### 4. Scoring and Analysis

#### Score Existing Poses
```bash
./target/release/zentdock score \
  --receptor receptor.pdb \
  --ligand ligand.pdb \
  --method vina
```

#### Compare Different Scoring Methods
```bash
# AutoDock4 scoring
./target/release/zentdock score -r receptor.pdb -l ligand.pdb --method autodock4

# Vina scoring  
./target/release/zentdock score -r receptor.pdb -l ligand.pdb --method vina

# ChemScore
./target/release/zentdock score -r receptor.pdb -l ligand.pdb --method chemscore
```

### 5. File Operations

#### Format Conversion
```bash
# PDB to MOL2
./target/release/zentdock convert \
  --input molecule.pdb \
  --output molecule.mol2 \
  --format mol2

# MOL2 to SDF
./target/release/zentdock convert \
  --input ligand.mol2 \
  --output ligand.sdf \
  --format sdf
```

#### Molecule Information
```bash
./target/release/zentdock info --file protein.pdb
```

### 6. Configuration Management

#### View Current Configuration
```bash
./target/release/zentdock config --show
```

#### Set Configuration Parameters
```bash
./target/release/zentdock config --set "num_threads=8"
./target/release/zentdock config --set "default_algorithm=multistage"
./target/release/zentdock config --set "default_max_iterations=50000"
```

## Architecture and Design

### Core Components

#### 🧬 Molecular Representation
```rust
// Core data structures
pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
    pub molecule_type: MoleculeType,
}

pub struct Atom {
    pub element: Element,
    pub position: Vector3D,
    pub partial_charge: f64,
    // ... additional properties
}
```

#### 🔄 Docking Engine
- **Modular Algorithm System**: Easy to add new optimization algorithms
- **Pluggable Scoring**: Support for multiple scoring functions
- **Parallel Processing**: Multi-threaded evaluation for performance

#### 📊 Scoring Framework
```rust
pub trait ScoringFunction {
    fn calculate(receptor: &Molecule, ligand: &Molecule, transform: &Transform) -> f64;
}
```

### Algorithm Details

#### Genetic Algorithm
- **Population Size**: 150 individuals (configurable)
- **Selection**: Tournament selection with size 20
- **Crossover**: Blend crossover for transform parameters
- **Mutation**: Gaussian perturbation with adaptive rate
- **Elitism**: Best individuals preserved across generations

#### Lamarckian GA
- Inherits all GA features
- **Local Optimization**: Gradient-free local search after each generation
- **Adaptive Learning**: Improved individuals feed back into population

#### Particle Swarm Optimization
- **Swarm Size**: Configurable population of particles
- **Cognitive Component**: Individual learning (c₁ = 1.496)
- **Social Component**: Swarm learning (c₂ = 1.496)
- **Inertia Weight**: Momentum conservation (w = 0.729)

#### Simulated Annealing
- **Temperature Schedule**: Exponential cooling from 1000K to 1K
- **Acceptance Probability**: Boltzmann distribution
- **Adaptive Cooling**: Temperature adjusts based on convergence

### Scoring Function Implementation

#### AutoDock4 Scoring
```
E_total = E_vdw + E_electrostatic + E_hbond + E_desolvation

Where:
- E_vdw: Lennard-Jones 12-6 potential
- E_electrostatic: Coulomb potential with distance cutoff
- E_hbond: Directional hydrogen bonding potential
- E_desolvation: Solvation effects modeling
```

#### Vina Scoring
```
E_total = w₁·Gauss₁ + w₂·Gauss₂ + w₃·Repulsion + w₄·Hydrophobic + w₅·Hydrogen

Where:
- Gauss₁, Gauss₂: Gaussian attractive potentials
- Repulsion: Steric repulsion term
- Hydrophobic: Hydrophobic interaction term
- Hydrogen: Hydrogen bonding term
```

## Performance Optimization

### Memory Management
- **Efficient Data Structures**: Optimized for molecular data
- **Lazy Evaluation**: Scoring calculations only when needed
- **Memory Pooling**: Reuse allocations for batch processing

### Computational Efficiency
- **Vectorized Operations**: SIMD optimizations where possible
- **Early Termination**: Convergence detection to stop unnecessary iterations
- **Adaptive Algorithms**: Parameter adjustment based on problem complexity

### Parallel Processing
- **Multi-threaded Scoring**: Parallel evaluation of population members
- **Batch Processing**: Concurrent processing of multiple ligands
- **Asynchronous I/O**: Non-blocking file operations

## Output and Results

### Console Output
```
=== Starting Docking Job ===
  Receptor atoms: 2500
  Ligand atoms: 45
  Algorithm: Genetic
  Scoring: Vina
  Search Box: 10.0 Å
  Num Poses: 10
  Max Iterations: 25000

  Iteration 0/25000 - Best energy: -8.45
  Iteration 5000/25000 - Best energy: -9.12
  Iteration 10000/25000 - Best energy: -9.23
  Iteration 15000/25000 - Best energy: -9.31
  Iteration 20000/25000 - Best energy: -9.35
  Iteration 25000/25000 - Best energy: -9.38

=== Docking Results ===
  Rank 1: Energy = -9.38 kcal/mol
  Rank 2: Energy = -9.12 kcal/mol
  Rank 3: Energy = -8.87 kcal/mol
  Rank 4: Energy = -8.65 kcal/mol
  Rank 5: Energy = -8.43 kcal/mol

Done!
```

### File Output
- **best_complex.pdb**: Receptor-ligand complex with best pose
- **pose_1_E-9.38.pdb**: Individual poses with energy in filename
- **results.txt**: Summary of all results with energies and rankings

## Advanced Usage

### Custom Search Space
```bash
# Define specific binding site
./target/release/zentdock dock \
  -r receptor.pdb \
  -l ligand.pdb \
  --center "12.5,18.3,25.1" \
  --radius 8.0 \
  --algorithm genetic
```

### Multi-objective Optimization
```bash
# Balance exploration and exploitation
./target/release/zentdock dock \
  -r receptor.pdb \
  -l ligand.pdb \
  --algorithm multistage \
  --num-poses 50 \
  --max-iterations 100000 \
  --population-size 300
```

### High-throughput Screening
```bash
# Process large ligand libraries
./target/release/zentdock batch \
  -r target_protein.pdb \
  -l compound_library/ \
  --jobs 16 \
  --output hts_results/ \
  --algorithm pso \
  --num-poses 3 \
  --continue-on-failure
```

## Troubleshooting

### Common Issues

#### Build Errors
```bash
# Update Rust toolchain
rustup update

# Clean build
cargo clean && cargo build --release
```

#### Memory Issues
```bash
# Reduce population size for large molecules
./target/release/zentdock dock \
  -r receptor.pdb \
  -l ligand.pdb \
  --population-size 50 \
  --num-poses 5
```

#### Convergence Problems
```bash
# Use multi-stage algorithm for better convergence
./target/release/zentdock dock \
  -r receptor.pdb \
  -l ligand.pdb \
  --algorithm multistage \
  --max-iterations 50000
```

### Performance Tips

1. **Use Multi-stage Algorithm**: Best balance of speed and accuracy
2. **Adjust Population Size**: Larger populations for complex problems
3. **Enable Monitoring**: Track convergence to optimize parameters
4. **Batch Processing**: Use multiple jobs for large ligand sets
5. **Proper Preparation**: Clean receptor and ligand structures

## Development

### Building from Source
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Linting
cargo clippy
```

### Contributing
1. Fork the repository
2. Create a feature branch
3. Implement your changes
4. Add tests
5. Submit a pull request

### Code Structure
```
src/
├── main.rs          # CLI interface
├── lib.rs           # Core library
├── core/            # Data structures
├── docking/         # Docking algorithms
├── scoring/         # Scoring functions
├── parser/          # File format parsers
├── utils/           # Utility functions
└── tools/           # CLI tools
```

## License

MIT License - Copyright (c) 2026 Mr. Nithish Kathiravan

## Authors

Mr. Nithish Kathiravan

## Support

- **Issues**: Report bugs and feature requests on GitHub
- **Documentation**: See SPEC.md for detailed technical specifications
- **Examples**: Check the examples/ directory for use cases
