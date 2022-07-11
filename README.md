Reed-Solomon Error Correction


### Installation/setup
#### For development/modifications
- Install Rust (Follow instructions at https://www.rust-lang.org/tools/install)
- Install cbind (cargo install --force cbindgen)

- To update header files run:
  - cbindgen --config cbindgen.toml --crate reed_solomon_error_correction --output reed_solomon_error_correction.h --lang c


### To use in a project
Simply include the header file and call the appropriate functions