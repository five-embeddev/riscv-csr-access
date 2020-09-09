SRC=test_csr.c
SRC_BASE=test_csr
CROSS_COMPILE=riscv-none-embed-
GCC=${CROSS_COMPILE}gcc
GXX=${CROSS_COMPILE}g++
OBJDUMP=${CROSS_COMPILE}objdump

SPEC_TARGETS=\
     rv32imac \
     rv64imac

MABI_rv32imac=ilp32
MABI_rv64imac=lp64

ALL=\
	$(SPEC_TARGETS:%=test_csr.c.%.disasm) \
	$(SPEC_TARGETS:%=test_csr.cpp.%.disasm) 

all : ${ALL}

${SRC_BASE}.c.%.disasm : ${SRC_BASE}.c 
	@echo "[TARGET $*]"
	${GCC} \
		-nostartfiles \
		-Werror \
		-Wl,--defsym,_start=0x1000 \
		-I ../include \
		-march=${*} \
		-mabi=${MABI_${*}} \
		-Os \
		-g \
		$< -o $(basename $@).elf
	${OBJDUMP} -S $(basename $@).elf > $@

${SRC_BASE}.cpp.%.disasm : ${SRC_BASE}.cpp
	@echo "[TARGET $*]"
	${GXX} \
		-nostartfiles \
		-Werror \
		-Wl,--defsym,_start=0x1000 \
		-I ../include \
		-march=${*} \
		-mabi=${MABI_${*}} \
		-Os \
		-g \
		-std=c++17 \
		$< -o $(basename $@).elf
	${OBJDUMP} -S $(basename $@).elf > $@

test_csr.c.rv32imac.disasm : ../include/riscv-csr.h

test_csr.c.rv64imac.disasm : ../include/riscv-csr.h

test_csr.cpp.rv32imac.disasm : ../include/riscv-csr.hpp

test_csr.cpp.rv64imac.disasm : ../include/riscv-csr.hpp

clean :
	rm -f ${ALL}
