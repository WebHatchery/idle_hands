//! Keep every retained test assigned to one protected responsibility.
//!
//! Five cases per responsibility is the hard ceiling in CODE_STANDARDS §11.2;
//! it is not a target or a reason to add coverage.

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

fn assert_source_has_no_test_modules(root: &Path) {
    let source_root = root.join("src");
    for entry in fs::read_dir(&source_root).expect("source files must be readable") {
        let path = entry.unwrap().path();
        if path.is_dir() {
            assert_source_has_no_test_modules_in(root, &path);
        } else {
            assert_source_file_has_no_tests(root, &path);
        }
    }
}

fn assert_source_has_no_test_modules_in(root: &Path, directory: &Path) {
    for entry in fs::read_dir(directory).expect("source files must be readable") {
        let path = entry.unwrap().path();
        if path.is_dir() {
            assert_source_has_no_test_modules_in(root, &path);
        } else {
            assert_source_file_has_no_tests(root, &path);
        }
    }
}

fn assert_source_file_has_no_tests(root: &Path, path: &Path) {
    if path.extension().is_none_or(|extension| extension != "rs") {
        return;
    }
    let source = fs::read_to_string(path).unwrap();
    let relative = path
        .strip_prefix(root)
        .unwrap()
        .to_string_lossy()
        .replace('\\', "/");
    assert!(
        !source.contains("#[cfg(test)]"),
        "test cfg remains in {relative}"
    );
    assert!(
        !source
            .lines()
            .map(str::trim)
            .any(|line| line == "mod tests {" || line == "mod tests;"),
        "test module remains in {relative}"
    );
}

#[test]
fn retained_cases_stay_within_the_hard_responsibility_ceiling() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let ownership: Ownership = serde_json::from_str(
        &fs::read_to_string(root.join("tests/feature_ownership.json")).unwrap(),
    )
    .unwrap();
    assert_source_has_no_test_modules(root);
    let mut cases = Vec::new();
    collect_cases(root, &root.join("tests"), &mut cases);
    let mut features: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (file, case) in &cases {
        let feature = ownership
            .cases
            .get(case)
            .or_else(|| ownership.files.get(file))
            .unwrap_or_else(|| panic!("Assign {case} to its existing protected responsibility"));
        assert!(!feature.trim().is_empty(), "{case} has no responsibility");
        features.entry(feature).or_default().push(case);
    }
    for (feature, tests) in features {
        assert!(
            tests.len() <= 5,
            "{feature} has {} cases (hard ceiling 5):\n{}",
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
