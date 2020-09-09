/*
   Register access functions for RISC-V system registers.
   SPDX-License-Identifier: Unlicense

   https://five-embeddev.com/

*/

#include "riscv-csr.h"

static volatile uint_xlen_t saved_mtvec;

int main(void) {

    uint_xlen_t cause = csr_read_mcause();
    if (cause & MCAUSE_INTERRUPT_BIT_MASK) {
        // Interrupt
        uint_xlen_t csr_read_mip();
    } else {
        // Exception
        // Read the source of the exception
        uint_xlen_t mepc =  csr_read_mepc();
        // Pointless save to mscratch
        csr_write_mscratch(mepc);
    }
    // Enable MIE.MSI    
    CSR_SET_BITS_IMM_MIE(MIE_MSI_BIT_MASK);
    // Enable MIE.MTI
    csr_set_bits_mie(MIE_MTI_BIT_MASK);
    // Clear mode and read mtvec.
    uint_xlen_t old_mtvec = csr_read_clr_bits_mtvec(MTVEC_MODE_ALL_SET_MASK);
    if (old_mtvec & MTVEC_MODE_ALL_SET_MASK) {
        // Remove mode bits
        old_mtvec &= MTVEC_BASE_ALL_SET_MASK;
    }
    saved_mtvec = old_mtvec;

}
