use clap::{Parser, Subcommand};
use serde_json;
use std::collections::HashMap;
use tensor_calc::*;

#[derive(Parser)]
#[command(name = "tensor-calc")]
#[command(about = "A CLI tool for symbolic tensor calculus")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compute Christoffel symbols from a metric tensor
    Christoffel {
        /// Metric tensor in JSON format
        #[arg(long)]
        metric: String,
        /// Coordinate variables in JSON array format
        #[arg(long)]
        coords: String,
    },
    /// Compute Riemann curvature tensor
    Riemann {
        /// Metric tensor in JSON format
        #[arg(long)]
        metric: String,
        /// Coordinate variables in JSON array format
        #[arg(long)]
        coords: String,
    },
    /// Compute Ricci tensor
    Ricci {
        /// Metric tensor in JSON format
        #[arg(long)]
        metric: String,
        /// Coordinate variables in JSON array format
        #[arg(long)]
        coords: String,
    },
    /// Compute Ricci scalar
    RicciScalar {
        /// Metric tensor in JSON format
        #[arg(long)]
        metric: String,
        /// Coordinate variables in JSON array format
        #[arg(long)]
        coords: String,
    },
    /// Compute Einstein tensor
    Einstein {
        /// Metric tensor in JSON format
        #[arg(long)]
        metric: String,
        /// Coordinate variables in JSON array format
        #[arg(long)]
        coords: String,
    },
    /// Solve Einstein field equations for vacuum spacetimes
    SolveVacuum {
        /// Coordinate variables in JSON array format
        #[arg(long)]
        coords: String,
        /// Symmetry ansatz: "spherical", "cosmological", "axisymmetric"
        #[arg(long)]
        symmetry: String,
        /// Boundary conditions in JSON format (optional)
        #[arg(long)]
        boundary_conditions: Option<String>,
    },
    /// Verify that a metric solves Einstein field equations
    VerifySolution {
        /// Metric tensor in JSON format
        #[arg(long)]
        metric: String,
        /// Coordinate variables in JSON array format
        #[arg(long)]
        coords: String,
        /// Stress-energy tensor in JSON format (optional for vacuum)
        #[arg(long)]
        stress_energy: Option<String>,
        /// Cosmological constant (optional)
        #[arg(long)]
        lambda: Option<String>,
    },
    /// Construct Einstein field equation system
    ConstructEquations {
        /// Stress-energy tensor in JSON format
        #[arg(long)]
        stress_energy: String,
        /// Coordinate variables in JSON array format
        #[arg(long)]
        coords: String,
        /// Cosmological constant (optional)
        #[arg(long)]
        lambda: Option<String>,
    },
}


fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Christoffel { metric, coords } => {
            compute_christoffel_symbols(&metric, &coords)
        }
        Commands::Riemann { metric, coords } => {
            compute_riemann_tensor(&metric, &coords)
        }
        Commands::Ricci { metric, coords } => {
            compute_ricci_tensor(&metric, &coords)
        }
        Commands::RicciScalar { metric, coords } => {
            compute_ricci_scalar(&metric, &coords)
        }
        Commands::Einstein { metric, coords } => {
            compute_einstein_tensor(&metric, &coords)
        }
        Commands::SolveVacuum { coords, symmetry, boundary_conditions } => {
            solve_vacuum_equations(&coords, &symmetry, boundary_conditions.as_deref())
        }
        Commands::VerifySolution { metric, coords, stress_energy, lambda } => {
            verify_solution(&metric, &coords, stress_energy.as_deref(), lambda.as_deref())
        }
        Commands::ConstructEquations { stress_energy, coords, lambda } => {
            construct_field_equations(&stress_energy, &coords, lambda.as_deref())
        }
    };

    match result {
        Ok(tensor_result) => {
            println!("{}", serde_json::to_string_pretty(&tensor_result).unwrap());
            std::process::exit(0);
        }
        Err(e) => {
            let error_result = TensorResult {
                result_type: "error".to_string(),
                data: serde_json::Value::Null,
                coordinates: vec![],
                success: false,
                error: Some(e.to_string()),
            };
            println!("{}", serde_json::to_string_pretty(&error_result).unwrap());
            std::process::exit(1);
        }
    }
}

fn compute_christoffel_symbols(metric_json: &str, coords_json: &str) -> Result<TensorResult, TensorError> {
    let metric: Vec<Vec<String>> = serde_json::from_str(metric_json)?;
    let coords: Vec<String> = serde_json::from_str(coords_json)?;
    
    let parsed_metric = parse_metric_tensor(metric, &coords)?;
    let christoffel = calculate_christoffel_symbols(&parsed_metric, &coords)?;
    
    Ok(TensorResult {
        result_type: "christoffel_symbols".to_string(),
        data: serde_json::to_value(christoffel)?,
        coordinates: coords,
        success: true,
        error: None,
    })
}

fn compute_riemann_tensor(metric_json: &str, coords_json: &str) -> Result<TensorResult, TensorError> {
    let metric: Vec<Vec<String>> = serde_json::from_str(metric_json)?;
    let coords: Vec<String> = serde_json::from_str(coords_json)?;
    
    let parsed_metric = parse_metric_tensor(metric, &coords)?;
    let riemann = calculate_riemann_tensor(&parsed_metric, &coords)?;
    
    Ok(TensorResult {
        result_type: "riemann_tensor".to_string(),
        data: serde_json::to_value(riemann)?,
        coordinates: coords,
        success: true,
        error: None,
    })
}

fn compute_ricci_tensor(metric_json: &str, coords_json: &str) -> Result<TensorResult, TensorError> {
    let metric: Vec<Vec<String>> = serde_json::from_str(metric_json)?;
    let coords: Vec<String> = serde_json::from_str(coords_json)?;
    
    let parsed_metric = parse_metric_tensor(metric, &coords)?;
    let ricci = calculate_ricci_tensor(&parsed_metric, &coords)?;
    
    Ok(TensorResult {
        result_type: "ricci_tensor".to_string(),
        data: serde_json::to_value(ricci)?,
        coordinates: coords,
        success: true,
        error: None,
    })
}

fn compute_ricci_scalar(metric_json: &str, coords_json: &str) -> Result<TensorResult, TensorError> {
    let metric: Vec<Vec<String>> = serde_json::from_str(metric_json)?;
    let coords: Vec<String> = serde_json::from_str(coords_json)?;
    
    let parsed_metric = parse_metric_tensor(metric, &coords)?;
    let scalar = calculate_ricci_scalar(&parsed_metric, &coords)?;
    
    Ok(TensorResult {
        result_type: "ricci_scalar".to_string(),
        data: serde_json::to_value(scalar)?,
        coordinates: coords,
        success: true,
        error: None,
    })
}

fn compute_einstein_tensor(metric_json: &str, coords_json: &str) -> Result<TensorResult, TensorError> {
    let metric: Vec<Vec<String>> = serde_json::from_str(metric_json)?;
    let coords: Vec<String> = serde_json::from_str(coords_json)?;
    
    let parsed_metric = parse_metric_tensor(metric, &coords)?;
    let einstein = calculate_einstein_tensor(&parsed_metric, &coords)?;
    
    Ok(TensorResult {
        result_type: "einstein_tensor".to_string(),
        data: serde_json::to_value(einstein)?,
        coordinates: coords,
        success: true,
        error: None,
    })
}

fn solve_vacuum_equations(coords_json: &str, symmetry: &str, boundary_conditions_json: Option<&str>) -> Result<TensorResult, TensorError> {
    let coords: Vec<String> = serde_json::from_str(coords_json)?;
    
    let boundary_conditions = if let Some(bc_json) = boundary_conditions_json {
        serde_json::from_str::<Vec<BoundaryCondition>>(bc_json)?
    } else {
        Vec::new()
    };
    
    let solutions = solve_vacuum_einstein_equations(&coords, symmetry, &boundary_conditions)?;
    
    Ok(TensorResult {
        result_type: "vacuum_solutions".to_string(),
        data: serde_json::to_value(solutions)?,
        coordinates: coords,
        success: true,
        error: None,
    })
}

fn verify_solution(metric_json: &str, coords_json: &str, stress_energy_json: Option<&str>, lambda_json: Option<&str>) -> Result<TensorResult, TensorError> {
    let metric: Vec<Vec<String>> = serde_json::from_str(metric_json)?;
    let coords: Vec<String> = serde_json::from_str(coords_json)?;
    
    let parsed_metric = parse_metric_tensor(metric, &coords)?;
    
    let stress_energy = if let Some(se_json) = stress_energy_json {
        Some(serde_json::from_str::<StressEnergyTensor>(se_json)?)
    } else {
        None
    };
    
    let lambda = if let Some(l_json) = lambda_json {
        Some(SymbolicExpr::parse(l_json)?)
    } else {
        None
    };
    
    // Create a solution object to verify
    let solution = EinsteinSolution {
        metric_tensor: parsed_metric,
        coordinates: coords.clone(),
        solution_type: "unknown".to_string(),
        constraints_satisfied: false,
        physical_parameters: HashMap::new(),
        solution_domain: "to be determined".to_string(),
    };
    
    let is_valid = verify_einstein_solution(&solution, stress_energy.as_ref(), lambda)?;
    
    Ok(TensorResult {
        result_type: "solution_verification".to_string(),
        data: serde_json::to_value(is_valid)?,
        coordinates: coords,
        success: true,
        error: None,
    })
}

fn construct_field_equations(stress_energy_json: &str, coords_json: &str, lambda_json: Option<&str>) -> Result<TensorResult, TensorError> {
    let stress_energy: StressEnergyTensor = serde_json::from_str(stress_energy_json)?;
    let coords: Vec<String> = serde_json::from_str(coords_json)?;
    
    let lambda = if let Some(l_json) = lambda_json {
        Some(SymbolicExpr::parse(l_json)?)
    } else {
        None
    };
    
    let equation_system = construct_einstein_field_equations(&stress_energy, &coords, lambda)?;
    
    Ok(TensorResult {
        result_type: "einstein_equations".to_string(),
        data: serde_json::to_value(equation_system)?,
        coordinates: coords,
        success: true,
        error: None,
    })
}
