# MPL-2.0 inventory

MPL-2.0 is file-level copyleft: if Tapestry modifies an MPL-2.0 file, that file
stays MPL-2.0, the change must be recorded, and its source must be available.
Record such a change in `MODIFICATIONS.md` with an `(MPL-2.0)` marker on its
line; `scripts/check-modifications.sh` enforces this for every path listed in
the section "In-tree MPL-2.0 files" below.

## In-tree MPL-2.0 files

Checked at baseline `2bdd1d6a`: no package in `third_party/`, no per-crate
`license` field in `Cargo.toml`, and no file header under `crates/` or
`third_party/` is MPL-2.0. The list is empty. Add one line per file, in the
form ``- `path/to/file` ``, if MPL-2.0 code is ever vendored or copied in.

## MPL-2.0 dependencies (not in tree)

These are pulled from crates.io. Their sources are not in this repository, so
unmodified use needs only the attribution already in `THIRD-PARTY-NOTICES`
(the MPL notice and licence text are in its Part II). If any is ever patched
or vendored, move it to the section above first.

| Package | Version | Source |
|---|---|---|
| cssparser | 0.31.2, 0.34.0 | https://github.com/servo/rust-cssparser |
| cssparser-macros | 0.6.1 | https://github.com/servo/rust-cssparser |
| dtoa-short | 0.3.5 | https://github.com/upsuper/dtoa-short |
| nucleo | 0.5.0 | https://github.com/helix-editor/nucleo |
| nucleo-matcher | 0.3.1 | https://github.com/helix-editor/nucleo |
| option-ext | 0.2.0 | https://github.com/soc/option-ext |
| selectors | 0.25.0 | https://github.com/servo/servo |
| selectors | 0.26.0 | https://github.com/servo/stylo |

Count: 8 distinct packages, 9 package-version entries.
