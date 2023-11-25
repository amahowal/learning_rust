To run individual modules do `cargo run control_flow`
The module must have a Cargo.toml file in it and a `main.rs` or otherwise indicate the target

Testing:
`cargo test` runs tests with the compiler in test mode, by default these are parallelized

to run serially use `cargo test -- --test-threads=1`
we can also make sure all the output is printed with `--show-output`, by default only failing tests print the stdout and stderr

another option is running tests by name like so `cargo test greater_than_100`
this is the same as running tests by a regular expression "contains"

if we have tests marked as ignored we can run those specifically by doing `cargo test -- --ignored`

integration tests live in the `tests` directory
three types of tests
1. unit
2. integration
3. doc

we can run a specific integration suite with `cargo test --test integration_test`
