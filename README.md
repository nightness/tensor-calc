# 🌌 World's First Rust-based Einstein Field Equation Solver 🌌

[![Crates.io](https://img.shields.io/crates/v/tensor-calc.svg)](https://crates.io/crates/tensor-calc)
[![Documentation](https://docs.rs/tensor-calc/badge.svg)](https://docs.rs/tensor-calc)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/nightness/tensor-calc#license)
[![GitHub](https://img.shields.io/badge/github-nightness/tensor--calc-blue?logo=github)](https://github.com/nightness/tensor-calc)

## A Rust-based Einstein Field Equation Solver

The first Rust-based Einstein field equation solver, bringing the memory safety and performance of Rust to spacetime physics with symbolic tensor calculus, automatic solution generation, and efficient computation.

### Key Features:
- First Rust-based Einstein equation solver (joining Python libraries like EinsteinPy and SageMath Manifolds)
- Memory-safe systems programming approach to tensor calculus
- Standalone Rust executable for general relativity computations  
- Production-ready CLI with Rust performance for spacetime physics
- Comprehensive Rust crate for integrated Einstein field equation solving

While Python libraries like EinsteinPy and SageMath Manifolds provide excellent GR capabilities, this is the first to bring Rust's performance and safety to Einstein equation solving.

## Features

### Einstein Field Equation Solving
- Schwarzschild black holes - Complete spherically symmetric solutions with event horizons
- Reissner-Nordström - Charged black hole spacetimes with electromagnetic fields
- FLRW cosmology - Expanding universe models with dark energy and matter
- de Sitter space - Exponential cosmic inflation and vacuum energy dominance  
- Kerr solutions - Rotating black hole spacetimes with angular momentum
- Automatic solving - Input coordinates and symmetry, get exact solutions
- Solution verification - Mathematically verify metrics satisfy Einstein equations
- Custom spacetimes - Build and verify your own exotic spacetime geometries

### Complete Tensor Calculus Operations
- Symbolic computation - Handles complex expressions like `sin(theta)^2`, `exp(H*t)`, `M/r^2`
- Christoffel symbols - Connection coefficients revealing spacetime curvature
- Riemann curvature tensor - Full 4D spacetime curvature at every point
- Ricci tensor & scalar - Matter-energy coupling and scalar curvature measures
- Einstein tensor - Direct computation of spacetime-matter relationship
- Symbolic derivatives - Automatic differentiation of tensor components
- Tensor contractions - Index raising/lowering and Einstein summation

### Production-Ready CLI Interface
- JSON I/O interface - Clean data exchange with any programming language
- Command line interface - Simple, powerful CLI for all operations
- Mathematical verification - Rigorous proof that solutions satisfy field equations  
- Equation system builder - Construct custom field equations from stress-energy
- Rust performance - Memory-safe, zero-cost abstractions, parallel processing
- Cross-platform - Runs on Linux, macOS, Windows

## Benefits for Different Users

### For Researchers & Physicists:
- Direct access to exact Einstein equation solutions
- No licensing costs or proprietary software dependencies
- Fast calculations that scale to complex problems
- Custom spacetimes - design and test your own exotic geometries
- Production ready - integrate into larger physics simulations

### For Developers & Engineers:
- CLI integration - Easy integration into any application via command line
- JSON API - Standard JSON input/output for universal compatibility
- Memory safe - Rust's safety guarantees for mission-critical applications  
- Open source - Modify and extend for your specific needs
- Future-proof - Built on modern, fast-growing Rust ecosystem

### For the Physics Community:
This provides an alternative to expensive, proprietary software with:
- Free and open general relativity computing
- High performance with Rust's zero-cost abstractions
- Universal integration with any system via JSON CLI interface
- Educational access - Students worldwide can explore spacetime physics
- Research acceleration - Focus on physics, not software limitations

---

## Installation

### From Crates.io
```bash
cargo install tensor-calc
```

### From Source
```bash
git clone https://github.com/nightness/tensor-calc.git
cd tensor-calc
cargo build --release
```

### Prebuilt Binaries
Download prebuilt binaries from the [GitHub Releases](https://github.com/nightness/tensor-calc/releases) page.

---

## Usage

```bash
# Build the project
cargo build --release

# 🌌 SOLVE EINSTEIN FIELD EQUATIONS

# Solve vacuum Einstein equations for spherically symmetric spacetime
./target/release/tensor-calc solve-vacuum \
  --coords '["t", "r", "theta", "phi"]' \
  --symmetry "spherical"

# Solve for cosmological spacetimes (FLRW, de Sitter)
./target/release/tensor-calc solve-vacuum \
  --coords '["t", "r", "theta", "phi"]' \
  --symmetry "cosmological"

# Solve for axisymmetric spacetimes (Kerr black holes)
./target/release/tensor-calc solve-vacuum \
  --coords '["t", "r", "theta", "phi"]' \
  --symmetry "axisymmetric"

# Verify that Schwarzschild metric solves Einstein equations
./target/release/tensor-calc verify-solution \
  --metric '[["-(1-2*M/r)", "0", "0", "0"], ["0", "1/(1-2*M/r)", "0", "0"], ["0", "0", "r^2", "0"], ["0", "0", "0", "r^2*sin(theta)^2"]]' \
  --coords '["t", "r", "theta", "phi"]'

# 🧮 TENSOR CALCULUS OPERATIONS

# Calculate Christoffel symbols
./target/release/tensor-calc christoffel \
  --metric '[["1", "0"], ["0", "r^2"]]' \
  --coords '["r", "theta"]'

# Calculate Riemann curvature tensor  
./target/release/tensor-calc riemann \
  --metric '[["1", "0"], ["0", "r^2"]]' \
  --coords '["r", "theta"]'

# Calculate Einstein tensor
./target/release/tensor-calc einstein \
  --metric '[["1", "0"], ["0", "r^2"]]' \
  --coords '["r", "theta"]'
```

## Example Solutions

### Schwarzschild Black Hole Solution:
```json
{
  "result_type": "vacuum_solutions",
  "data": [
    {
      "solution_type": "exact",
      "metric_tensor": [
        ["-(1 - 2*M/r)", "0", "0", "0"],
        ["0", "1/(1 - 2*M/r)", "0", "0"], 
        ["0", "0", "r^2", "0"],
        ["0", "0", "0", "r^2*sin(theta)^2"]
      ],
      "physical_parameters": {
        "M": "Mass of the black hole"
      },
      "solution_domain": "r > 2M (outside event horizon)",
      "constraints_satisfied": true
    }
  ]
}
```

### FLRW Expanding Universe:
```json
{
  "solution_type": "exact",
  "metric_tensor": [
    ["-1", "0", "0", "0"],
    ["0", "a(t)^2", "0", "0"],
    ["0", "0", "a(t)^2 * r^2", "0"], 
    ["0", "0", "0", "a(t)^2 * r^2 * sin(theta)^2"]
  ],
  "physical_parameters": {
    "a(t)": "Scale factor - universe expansion",
    "H": "Hubble parameter",
    "Omega_m": "Matter density parameter",
    "Omega_Lambda": "Dark energy density parameter"
  },
  "solution_domain": "Spatially homogeneous universe"
}
```

### Reissner-Nordström Charged Black Hole:
```json
{
  "metric_tensor": [
    ["-(1 - 2*M/r + Q^2/r^2)", "0", "0", "0"],
    ["0", "1/(1 - 2*M/r + Q^2/r^2)", "0", "0"],
    ["0", "0", "r^2", "0"],
    ["0", "0", "0", "r^2*sin(theta)^2"]
  ],
  "physical_parameters": {
    "M": "Mass",
    "Q": "Electric charge"
  },
  "solution_domain": "r > M + sqrt(M^2 - Q^2)"
}
```

## Integration with Other Systems

This standalone Rust binary can be integrated into any system through its JSON CLI interface:
- Command line - Direct execution from any shell or script
- JSON I/O - Structured input/output for programmatic access  
- Universal compatibility - Works with Python, JavaScript, Go, C++, etc.
- High performance - Optimized Rust binary for production workloads

## Mathematical Foundation

The implementation uses exact symbolic computation of Einstein's field equations:

### Einstein Field Equations:
```
G_μν + Λg_μν = 8πT_μν
```
Where:
- G_μν = Einstein tensor (spacetime curvature)  
- Λ = Cosmological constant (dark energy)
- T_μν = Stress-energy tensor (matter/energy)

### Core Tensor Calculus Formulas:
- Christoffel symbols: `Γ^μ_αβ = (1/2) * g^μν * (∂g_νβ/∂x^α + ∂g_να/∂x^β - ∂g_αβ/∂x^ν)`
- Riemann tensor: `R^ρ_σμν = ∂Γ^ρ_σν/∂x^μ - ∂Γ^ρ_σμ/∂x^ν + Γ^ρ_λμ*Γ^λ_σν - Γ^ρ_λν*Γ^λ_σμ`
- Einstein tensor: `G_μν = R_μν - (1/2) * g_μν * R`

## Technology Stack

- Rust - Memory-safe systems programming with zero-cost abstractions
- Serde - JSON serialization for universal data exchange  
- nalgebra - High-performance linear algebra with SIMD optimization
- Clap - Modern CLI framework with auto-generated help and validation
- Regex - Pattern matching for symbolic expression parsing
- PetGraph - Graph algorithms for dependency analysis
- num-complex - Complex number arithmetic for advanced physics

---

## Conclusion

This project represents a significant step forward in computational physics accessibility:

- From proprietary to open source  
- From expensive to free  
- From complex to simple  
- From isolated to integrated  
- From slower interpreted languages to fast compiled performance  

Einstein's equations are now accessible to everyone through a modern, safe, and efficient implementation.

---

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---
*Built with ❤️  and a passion for making the universe computable.*