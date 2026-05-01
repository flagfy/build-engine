use wasm_bindgen::prelude::*;
use std::collections::HashMap;

mod models;
mod compiler;
mod manifests;
mod hashing;

use models::{BuildInput, ProjectSpec, BuildOutput};

#[wasm_bindgen]
pub fn build_state(input_json: &str) -> String {
    match build_state_inner(input_json) {
        Ok(output) => output,
        Err(e) => {
            let error = serde_json::json!({ "error": e.to_string() });
            serde_json::to_string_pretty(&error)
                .unwrap_or_else(|_| String::from("{\"error\": \"serialization failed\"}"))
        }
    }
}

fn build_state_inner(input_json: &str) -> Result<String, Box<dyn std::error::Error>> {
    let input: BuildInput = serde_json::from_str(input_json)
        .map_err(|e| format!("Invalid input JSON: {}", e))?;

    let project: ProjectSpec = serde_yaml::from_str(&input.project)
        .map_err(|e| format!("Invalid project YAML: {}", e))?;

    let globals = compiler::parse_globals(&input.globals);

    let (active_environments, bundles) =
        compiler::compile_bundles(&input, &project, hashing::sha256_of);

    let env_outputs = manifests::build_environment_outputs(&project, bundles)?;

    let generated_at = {
        use chrono::Utc;
        Utc::now().to_rfc3339()
    };

    let mut files: HashMap<String, serde_json::Value> = HashMap::new();

    files.insert(
        "meta.json".to_string(),
        serde_json::json!({
            "spec": project.spec,
            "generated_at": generated_at,
            "globals": globals,
            "active_environments": active_environments,
        }),
    );

    for (env_name, env_output) in env_outputs {
        files.insert(
            format!("bundles/{}.json", env_name),
            serde_json::to_value(&env_output.bundle)
                .map_err(|e| format!("Failed to serialize bundle for {}: {}", env_name, e))?,
        );
        files.insert(
            format!("quick_checks/{}.json", env_name),
            serde_json::to_value(&env_output.quick_check)
                .map_err(|e| format!("Failed to serialize quick_check for {}: {}", env_name, e))?,
        );
    }

    let output = BuildOutput {
        spec: project.spec.clone(),
        generated_at,
        globals,
        active_environments,
        files,
    };

    Ok(serde_json::to_string_pretty(&output)
        .map_err(|e| format!("Failed to serialize output: {}", e))?)
}
