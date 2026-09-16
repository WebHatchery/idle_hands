//! Explicit ownership keeps a feature's five-case budget intact across files.
//!
//! The map is reviewed with changes: a new filename is not a new feature.
//! This gate scans source, so disabled or ignored cases still spend a slot.

use serde::Deserialize;
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Deserialize)]
struct Ownership {
    files: BTreeMap<String, String>,
    cases: BTreeMap<String, String>,
}

fn collect_cases(root: &Path, directory: &Path, cases: &mut Vec<(String, String)>) {
    for entry in fs::read_dir(directory).expect("test sources must be readable") {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_cases(root, &path, cases);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            let source = fs::read_to_string(&path).unwrap();
            let relative = path
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            let mut pending_test = false;
            for line in source.lines().map(str::trim) {
                if line == "#[test]" {
                    assert!(!pending_test, "missing test function in {relative}");
                    pending_test = true;
                } else if pending_test && line.starts_with("fn ") {
                    let name = line[3..].split('(').next().unwrap();
                    cases.push((relative.clone(), format!("{relative}::{name}")));
                    pending_test = false;
                }
            }
            assert!(!pending_test, "unrecognized test declaration in {relative}");
        }
    }
}

#[test]
fn cohesive_features_have_no_more_than_five_test_cases() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let ownership: Ownership = serde_json::from_str(
        &fs::read_to_string(root.join("tests/feature_ownership.json")).unwrap(),
    )
    .unwrap();
    let mut cases = Vec::new();
    for directory in ["src", "tests"] {
        collect_cases(root, &root.join(directory), &mut cases);
    }
    let mut features: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (file, case) in &cases {
        let feature = ownership
            .cases
            .get(case)
            .or_else(|| ownership.files.get(file))
            .unwrap_or_else(|| {
                panic!("Assign {case} to its existing cohesive feature before adding a test")
            });
        assert!(!feature.trim().is_empty(), "{case} has no feature name");
        features.entry(feature).or_default().push(case);
    }
    for (feature, tests) in features {
        assert!(
            tests.len() <= 5,
            "{feature} has {} cases (hard limit 5):\n{}",
            tests.len(),
            tests.join("\n")
        );
    }
    for file in ownership.files.keys() {
        assert!(
            cases.iter().any(|(found, _)| found == file),
            "stale file ownership: {file}"
        );
    }
    for case in ownership.cases.keys() {
        assert!(
            cases.iter().any(|(_, found)| found == case),
            "stale case ownership: {case}"
        );
    }
}
