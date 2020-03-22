# RacHL7

### a Rusty HL7 parser

## Maintainers

Run `pip install ".[dev,deploy]"` 

`setup.py` cannot run on its own before this step. `pyproject.toml` fixes
that by installing the prerequisite `setuptools-rust`.


## Beginner rust guide

1. uninstall rustc from apt
2. install rustup from rustup.rs with curl
3. (restart the shell if necessary to use the updated PATH)
4. `rustup install nightly`
5. `rustup default nightly`
6. `cargo build`