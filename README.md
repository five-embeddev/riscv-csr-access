# RISC-V CSR Access Routines

RISC-V System Register Access Routines in various languages.

For more information see <https://five-embeddev.com/quickref/csrs-access.html>

The register acccess funtions and classes are generated from templates
based on a yaml definition file.

Files:

- `include/riscv-csr.h`: C using macros.
- `include/riscv-csr.hpp`: C++ using an a class interface.
- `rs/riscv_csr_macros/src/riscv_csr_macros.rs`: Rust using macros.

Generators:

- `templates/*` : Jinja2 template files used to generate `include/*`
- `templates/riscv_csr_filters.py` : Helper functions for templates.
- `extern/development-utils/generators/yaml_jinja.py` : Program used to generate source code.  See <https://github.com/nakane1chome/development-utils>.
- `extern/riscv-isa-data/csr.yaml` : CSR definitions. See <https://five-embeddev.com/quickref/csrs.html>.

Examples:

- `examples/test_csr.c` : Example of using `riscv-csr.h`
- `examples/test_csr.cpp` : Example of using `riscv-csr.hpp`
- `rs/riscv_csr_macros/examples/test_csr.rs` : Example of using `riscv_csr_macros.rs`
