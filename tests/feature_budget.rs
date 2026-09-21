//! Explicit ownership keeps a feature's strong five-case target intact across files.
//!
//! The map is reviewed with changes: a new filename is not a new feature.
//! Integration suites are the only test source; disabled or ignored cases still
//! spend a slot. Any feature above five cases must carry a written exception.

use serde::Deserialize;
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Deserialize)]
struct Ownership {
    files: BTreeMap<String, String>,
    cases: BTreeMap<String, String>,
    exceptions: BTreeMap<String, Exception>,
}

#[derive(Deserialize)]
struct Exception {
    max_cases: usize,
    reason: String,
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
fn cohesive_features_have_no_more_than_five_test_cases() {
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
            .unwrap_or_else(|| {
                panic!("Assign {case} to its existing cohesive feature before adding a test")
            });
        assert!(!feature.trim().is_empty(), "{case} has no feature name");
        features.entry(feature).or_default().push(case);
    }
    for (feature, tests) in features {
        let limit = ownership
            .exceptions
            .get(feature)
            .map(|exception| {
                assert!(
                    exception.max_cases > 5,
                    "{feature} exception must exceed five"
                );
                assert!(
                    !exception.reason.trim().is_empty(),
                    "{feature} exception needs a reason"
                );
                exception.max_cases
            })
            .unwrap_or(5);
        assert!(
            tests.len() <= limit,
            "{feature} has {} cases (target {limit}):\n{}",
            tests.len(),
            tests.join("\n")
        );
        if tests.len() > 5 {
            assert!(
                ownership.exceptions.contains_key(feature),
                "{feature} exceeds five cases; document its distinct coverage in exceptions"
            );
        }
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
