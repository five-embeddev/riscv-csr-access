GENERATOR_PATH=extern/development-utils/generators/
ISA_DATA_PATH=extern/riscv-isa-data/

all : \
	include/riscv-csr.hpp \
	include/riscv-csr.h

# Generate generic riscv CSR definitions
include/% : templates/%
	${GENERATOR_PATH}/yaml_jinja.py \
		--filter templates/riscv_csr_filters.py \
		${ISA_DATA_PATH}/csr.yaml \
	    $< \
		$@

