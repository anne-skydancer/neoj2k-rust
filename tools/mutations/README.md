<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Mutation lists

Each `*.json` file in this folder is a list of mutations that `tools/tier1_mutations.py` applies, one at a time, to a
scratch copy of the tree. A mutation is a small wrong change of the code; the tests it names must then fail. A test
that still passes cannot see that kind of mistake. CI runs the lists in `.github/workflows/tier1-verify.yml`; with no
list in the folder the check passes at once.

## Format

A list is a JSON array of objects:

| Key | Required | Meaning |
|---|---|---|
| `name` | yes | A unique name over all the lists; the table and the shards use it. |
| `file` | yes | The file to change, relative to the repository root. |
| `old` | yes | The text to replace, written with line feeds (a file with CRLF is matched too). It must occur exactly `count` times. |
| `new` | yes | The wrong text. It must compile. |
| `tests` | yes | Filters given to `cargo test`, a substring of the test's path. Each is run on its own, and each must fail. |
| `count` | no | How many times `old` occurs in `file`; default 1. All of them are replaced. |
| `package` | no | The cargo package; default `neo-j2k`. |
| `target` | no | `--lib` (default), or `--test NAME`. |

## Rules

- A list ships with the code it tests, in the same pull request. Run against a branch where the patterns do not occur,
  the check fails, so a list is never added to `main` ahead of its code.
- A pattern is anchored by enough text to occur the stated number of times; `python tools/tier1_mutations.py --list`
  checks that, and builds nothing.
- A mutation that survives is possibly an equivalent mutant (a change that does not alter what the code computes).
  Either a test is missing, or the mutation is dropped or replaced by one that changes the result. The run fails
  until one of the two is done.
- A mutation that does not compile, or a filter that matches no test, is an error of the list (exit 2).
- Before the first mutation the runner runs every named test unmutated; one that does not pass stops the run.
