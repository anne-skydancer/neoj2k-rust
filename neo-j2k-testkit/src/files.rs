// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Inputs and folders: the repository's corpus, fuzz inputs and test data, the input lists of
//! `tests/common/common.cpp` (`expand_inputs`), and the tests' temporary folder.

use std::path::{Path, PathBuf};

/// The repository's root (this crate lives in `neo-j2k-testkit`).
#[must_use]
pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// `corpus/generated`: the generated corpus and the oracle's reference tables.
#[must_use]
pub fn corpus_dir() -> PathBuf {
    repo_root().join("corpus/generated")
}

/// The conformance and non-regression data (`NJ2K_TESTDATA`, from `tools/fetch_testdata.py`);
/// `None` when it is not set or not there, and the tests that need it are skipped, as in CTest.
#[must_use]
pub fn testdata_dir() -> Option<PathBuf> {
    let dir = PathBuf::from(std::env::var_os("NJ2K_TESTDATA")?);
    dir.is_dir().then_some(dir)
}

/// The folder for the tests' temporary files: `NJ2K_TEST_TMP` when set (the repository's
/// Defender-excluded `.fixture-tmp`), else the system's.
#[must_use]
pub fn test_tmp_dir() -> PathBuf {
    std::env::var_os("NJ2K_TEST_TMP").map_or_else(std::env::temp_dir, PathBuf::from)
}

fn is_codestream_name(p: &Path, with_jp2: bool) -> bool {
    let ext = p
        .extension()
        .map(|e| e.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    matches!(ext.as_str(), "j2k" | "j2c" | "jpc") || (with_jp2 && matches!(ext.as_str(), "jp2" | "jph"))
}

/// A list argument, as the C++ harness takes one: a directory (its `.j2k`, `.j2c` and `.jpc`
/// files, and `.jp2`, `.jph` with `with_jp2`, sorted by path), one codestream or JP2 file, or a
/// text file with one path per line (`#` starts a comment; relative paths are taken from the
/// list's folder). Paths keep the C++'s order (byte order of their text).
#[must_use]
pub fn expand_inputs(arg: &Path, with_jp2: bool) -> Vec<PathBuf> {
    if arg.is_dir() {
        let mut out: Vec<PathBuf> = std::fs::read_dir(arg)
            .into_iter()
            .flatten()
            .flatten()
            .filter(|e| e.file_type().is_ok_and(|t| t.is_file()) && is_codestream_name(&e.path(), with_jp2))
            .map(|e| e.path())
            .collect();
        out.sort_by(|a, b| a.as_os_str().as_encoded_bytes().cmp(b.as_os_str().as_encoded_bytes()));
        return out;
    }
    if is_codestream_name(arg, true) {
        return vec![arg.to_path_buf()];
    }
    let base = arg.parent().unwrap_or(Path::new(""));
    std::fs::read_to_string(arg)
        .unwrap_or_default()
        .lines()
        .map(str::trim)
        .filter(|t| !t.is_empty() && !t.starts_with('#'))
        .map(|t| {
            let item = Path::new(t);
            if item.is_relative() {
                base.join(item)
            } else {
                item.to_path_buf()
            }
        })
        .collect()
}

/// A file's name, for messages.
#[must_use]
pub fn base_name(p: &Path) -> String {
    p.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

#[cfg(all(test, not(miri)))]
mod tests {
    use super::*;

    #[test]
    fn the_corpus_lists_sorted() {
        let files = expand_inputs(&corpus_dir(), false);
        assert!(files.len() >= 60, "{} corpus files", files.len());
        let names: Vec<String> = files.iter().map(|p| base_name(p)).collect();
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(names, sorted);
        assert!(names.iter().all(|n| n.ends_with(".j2k")));
    }
}
