/*
   Register access functions for RISC-V system registers.
   SPDX-License-Identifier: Unlicense

   https://five-embeddev.com/

*/
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// error: `#[panic_handler]` function required, but not found
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    loop {}
}

use riscv_csr_macros::csr_read_mcause;
use riscv_csr_macros::csr_read_mip;
use riscv_csr_macros::csr_read_mepc;
use riscv_csr_macros::csr_write_mscratch;
use riscv_csr_macros::csr_read_clr_bits_mtvec;
use riscv_csr_macros::csr_set_bits_mie;
use riscv_csr_macros::csr_set_bits_imm_mie;
use riscv_csr_macros::MCAUSE_INTERRUPT_BIT_MASK;
use riscv_csr_macros::MIE_MTI_BIT_MASK;
use riscv_csr_macros::MTVEC_BASE_ALL_SET_MASK;
use riscv_csr_macros::MTVEC_MODE_ALL_SET_MASK;
use riscv_csr_macros::UintXlen;


#[no_mangle]
pub fn test_csr() {
    let mut _saved_mtvec: UintXlen = 0;
    let cause = csr_read_mcause!();
    if (cause & MCAUSE_INTERRUPT_BIT_MASK) != 0 {
        // Interrupt
        let _mip = csr_read_mip!();
    } else {
        // Exception
        // Read the source of the exception
        let mepc =  csr_read_mepc!();
        // Pointless save to mscratch
        csr_write_mscratch!(mepc);
    }
    // Enable MIE.MSI    
    csr_set_bits_imm_mie!(MIE_MSI_BIT_MASK);
    // Enable MIE.MTI
    csr_set_bits_mie!(MIE_MTI_BIT_MASK);
    // Clear mode and read mtvec.
    let mut old_mtvec = csr_read_clr_bits_mtvec!(MTVEC_MODE_ALL_SET_MASK);
    if (old_mtvec & MTVEC_MODE_ALL_SET_MASK) != 0 {
        // Remove mode bits
        old_mtvec = old_mtvec & MTVEC_BASE_ALL_SET_MASK;
    }
    _saved_mtvec = old_mtvec;
}
