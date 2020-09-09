/*
   Register access functions for RISC-V system registers.
   SPDX-License-Identifier: Unlicense

   https://five-embeddev.com/

*/

#include "riscv-csr.hpp"

static volatile riscv::csr::uint_xlen_t saved_mtvec;

int main(void) {

    riscv::csr::all csrs;

    auto cause = csrs.mcause.read();
    if (cause & riscv::csr::mcause_data::interrupt::BIT_MASK) {
        // Interrupt
        auto mip = csrs.mip.read();
    } else {
        // Exception
        // Read the source of the exception
        auto mepc = csrs.mepc.read();
        // Pointless save to mscratch
        csrs.mscratch.write(mepc);
    }
    // Enable MIE.MSI    
    csrs.mie.msi.set();
    // Enable MIE.MTI
    csrs.mie.mti.set();
    // Clear mode and read mtvec.
    auto old_mtvec = csrs.mtvec.read_clr_bits_const<riscv::csr::mtvec_data::mode::ALL_SET_MASK>();
    if (old_mtvec & riscv::csr::mtvec_data::mode::ALL_SET_MASK) {
        // Remove mode bits
        old_mtvec &= riscv::csr::mtvec_data::base::ALL_SET_MASK;
    }
    saved_mtvec = old_mtvec;

    // ----
    // Try writing const values

    // Write via a register value to the MTVEC
    csrs.mtvec.write(0x10);
    // Write an immediate value to the MTVEC
    csrs.mtvec.write_const<0x10>();
    // Write via a register value to the MTVEC
    csrs.mtvec.write_const<0x100000>();

    // ----
    // Try out the operators

    // Read mcycle via default operator
    auto mcycle = csrs.mcycle();

#if 0 // TODO - upgrade to gcc that supports these CSRs
    // Disable all the counters
    csrs.mcountinhibit |= 0xFFFFFFFF;

    // Enable the instruction retired count
    csrs.mcountinhibit.ir.clr();
    // Enable the cycle count
    csrs.mcountinhibit.cy.write(0);
    // Enable the hpm count
    csrs.mcountinhibit.hpm.clr();
#endif
}
