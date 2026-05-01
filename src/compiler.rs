use std::collections::HashMap;
use crate::models::{
    BuildInput, ProjectSpec, ParsedEntry, FileMeta, EnvironmentFilter, EnvBundle,
};

pub fn parse_entry(name: &str, content: &str) -> ParsedEntry {
    let mut yaml_val: serde_yaml::Value =
        serde_yaml::from_str(content).unwrap_or(serde_yaml::Value::Null);

    let meta = if let serde_yaml::Value::Mapping(ref mut map) = yaml_val {
        let meta_key = serde_yaml::Value::String("meta".to_string());
        if let Some(meta_val) = map.remove(&meta_key) {
            parse_file_meta(&meta_val)
        } else {
            FileMeta { environments: EnvironmentFilter::All }
        }
    } else {
        FileMeta { environments: EnvironmentFilter::All }
    };

    let content_json = serde_json::to_value(&yaml_val).unwrap_or(serde_json::Value::Null);
    ParsedEntry { name: name.to_string(), meta, content: content_json }
}

fn parse_file_meta(val: &serde_yaml::Value) -> FileMeta {
    let environments = if let serde_yaml::Value::Mapping(map) = val {
        let key = serde_yaml::Value::String("environments".to_string());
        match map.get(&key) {
            Some(serde_yaml::Value::String(s)) if s == "all" => EnvironmentFilter::All,
            Some(serde_yaml::Value::Sequence(seq)) => {
                let list = seq
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                EnvironmentFilter::List(list)
            }
            _ => EnvironmentFilter::All,
        }
    } else {
        EnvironmentFilter::All
    };
    FileMeta { environments }
}

fn matches_env(entry: &ParsedEntry, env: &str) -> bool {
    match &entry.meta.environments {
        EnvironmentFilter::All => true,
        EnvironmentFilter::List(list) => list.iter().any(|e| e == env),
    }
}

fn filter_entries(entries: &[ParsedEntry], env: &str) -> HashMap<String, serde_json::Value> {
    entries
        .iter()
        .filter(|e| matches_env(e, env))
        .map(|e| (e.name.clone(), e.content.clone()))
        .collect()
}

pub fn discover_environments(
    project: &ProjectSpec,
    all_entries: &[&[ParsedEntry]],
) -> Vec<String> {
    let mut envs: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    envs.insert("all".to_string());

    for key in project.environments.keys() {
        envs.insert(key.clone());
    }

    for group in all_entries {
        for entry in *group {
            if let EnvironmentFilter::List(list) = &entry.meta.environments {
                for env in list {
                    envs.insert(env.clone());
                }
            }
        }
    }

    envs.into_iter().collect()
}

pub fn compile_bundles(
    input: &BuildInput,
    project: &ProjectSpec,
    hash_fn: impl Fn(&str) -> String,
) -> (Vec<String>, HashMap<String, EnvBundle>) {
    let contexts: Vec<ParsedEntry> = input.contexts.iter()
        .map(|f| parse_entry(&f.name, &f.content))
        .collect();
    let segments: Vec<ParsedEntry> = input.segments.iter()
        .map(|f| parse_entry(&f.name, &f.content))
        .collect();
    let decisions: Vec<ParsedEntry> = input.decisions.iter()
        .map(|f| parse_entry(&f.name, &f.content))
        .collect();

    let active_envs = discover_environments(
        project,
        &[&contexts, &segments, &decisions],
    );

    let mut bundles = HashMap::new();

    for env in &active_envs {
        let ctx = filter_entries(&contexts, env);
        let seg = filter_entries(&segments, env);
        let dec = filter_entries(&decisions, env);

        let raw = serde_json::json!({
            "contexts": ctx,
            "segments": seg,
            "decisions": dec,
        });
        let raw_str = serde_json::to_string(&raw).unwrap_or_default();
        let core_version = hash_fn(&raw_str);

        bundles.insert(env.clone(), EnvBundle {
            core_version,
            contexts: ctx,
            segments: seg,
            decisions: dec,
        });
    }

    (active_envs, bundles)
}

pub fn parse_globals(globals_str: &str) -> HashMap<String, serde_json::Value> {
    let yaml_val: serde_yaml::Value =
        serde_yaml::from_str(globals_str).unwrap_or_default();
    match yaml_val {
        serde_yaml::Value::Mapping(map) => map
            .into_iter()
            .map(|(k, v)| {
                let key = k.as_str().unwrap_or("").to_string();
                let val = serde_json::to_value(&v).unwrap_or(serde_json::Value::Null);
                (key, val)
            })
            .collect(),
        _ => HashMap::new(),
    }
}
