use std::collections::HashMap;
use crate::models::{ProjectSpec, EnvBundle, QuickCheck, EnvironmentOutput};

pub fn build_environment_outputs(
    project: &ProjectSpec,
    bundles: HashMap<String, EnvBundle>,
) -> Result<HashMap<String, EnvironmentOutput>, Box<dyn std::error::Error>> {
    let mut outputs = HashMap::new();

    for (env_name, bundle) in bundles {
        let version_prefix = if bundle.core_version.len() >= 8 {
            &bundle.core_version[..8]
        } else {
            &bundle.core_version
        };
        let env_version = format!("{}-{}", env_name, version_prefix);

        let (publish_path, bootstrap_pointer) = if env_name == "all" {
            (
                project.publish.path.trim_end_matches('/').to_string(),
                None,
            )
        } else if let Some(env_spec) = project.environments.get(&env_name) {
            let path = env_spec.overrides.publish.as_ref()
                .map(|x| x.path.clone())
                .unwrap_or_else(|| project.publish.path.clone());
            let pointer = env_spec.overrides.bootstrap.as_ref()
                .map(|x| x.pointer.clone())
                .unwrap_or_else(|| project.bootstrap.pointer.clone());
            (path, Some(pointer))
        } else {
            (project.publish.path.clone(), Some(project.bootstrap.pointer.clone()))
        };

        let quick_check = QuickCheck {
            bundle_uri: format!("{}/{}/core.bundle.json", publish_path, bundle.core_version),
            checksum: bundle.core_version.clone(),
            env_version,
            publish_path: if env_name == "all" { None } else { Some(publish_path) },
            bootstrap_pointer,
        };

        outputs.insert(env_name, EnvironmentOutput { bundle, quick_check });
    }

    Ok(outputs)
}
