# Frozen credential fixtures

All passwords are synthetic. Do not regenerate expected hashes with the implementation under test.

- `historical-hash-fixtures.json`: 112 hashes generated through sync/async APIs in published `@node-rs/bcrypt` 1.7.3, 1.9.2, 1.10.5, and 1.10.9. The rows record original cost/salt inputs, including old numeric coercion and text salt handling where supported. Generated on macOS arm64, Node 24.20.0.
- `stored-hash-fixtures.json`: the acceptance baseline from published 1.10.9, including long-password suffixes and both accepted and rejected legacy-prefix vectors. Generated through its WASI backend; subsequently checked against native 1.10.9 and the source build.
- `verification-parser-fixtures.json`: fixed `+4` cost cases accepted by the previous verifier. The creation parser must not gate these checks.

The test suite additionally uses the pinned `bcrypt-previous` npm alias (1.10.9) to verify newly created hashes and checks independent known answers with bcryptjs.
