/*
   Register access functions for RISC-V system registers.
   SPDX-License-Identifier: Unlicense

   https://five-embeddev.com/

*/

#![no_std]

#[cfg(target_pointer_width = "32")]
pub type UintXlen = u32;
#[cfg(target_pointer_width = "32")]
pub type UintCsr64 = u32;

#[cfg(target_pointer_width = "64")]
pub type  UintXlen = u64;
#[cfg(target_pointer_width = "64")]
pub type  UintCsr64 = u64;

pub type  UintCsr32 = u32;


#[allow(non_upper_case_globals)]
#[cfg(target_pointer_width = "32")]
const __riscv_xlen : isize = 32;

#[allow(non_upper_case_globals)]
#[cfg(target_pointer_width = "64")]
const __riscv_xlen : isize = 64;

/*******************************************
 * misa - MRW - Machine ISA 
 */
/* misa: CSR Whole register access */
/* misa: CSR read.
e.g.
    let _v = csr_read_misa!();
 */
#[macro_export]
macro_rules! csr_read_misa {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, misa" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* misa: CSR write 
e.g.
    csr_write_misa!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_misa {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    misa, {0}" , in(reg) $x);
        }
    );
}
/* misa: CSR Read and Write 
e.g.
    let v_ = csr_read_write_misa!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_misa {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, misa, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* misa: CSR Field Modifications - via register */

/*******************************************
 * mvendorid - MRO - Machine Vendor ID 
 */
/* mvendorid: CSR Whole register access */
/* mvendorid: CSR read.
e.g.
    let _v = csr_read_mvendorid!();
 */
#[macro_export]
macro_rules! csr_read_mvendorid {
    ( ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mvendorid" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}

/* mvendorid: CSR Field Modifications - via register */

/*******************************************
 * marchid - MRO - Machine Architecture ID 
 */
/* marchid: CSR Whole register access */
/* marchid: CSR read.
e.g.
    let _v = csr_read_marchid!();
 */
#[macro_export]
macro_rules! csr_read_marchid {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, marchid" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}

/* marchid: CSR Field Modifications - via register */

/*******************************************
 * mimpid - MRO - Machine Implementation ID 
 */
/* mimpid: CSR Whole register access */
/* mimpid: CSR read.
e.g.
    let _v = csr_read_mimpid!();
 */
#[macro_export]
macro_rules! csr_read_mimpid {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mimpid" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}

/* mimpid: CSR Field Modifications - via register */

/*******************************************
 * mhartid - MRO - Hardware Thread ID 
 */
/* mhartid: CSR Whole register access */
/* mhartid: CSR read.
e.g.
    let _v = csr_read_mhartid!();
 */
#[macro_export]
macro_rules! csr_read_mhartid {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mhartid" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}

/* mhartid: CSR Field Modifications - via register */

/*******************************************
 * mstatus - MRW - Machine Status 
 */
/* mstatus: CSR Whole register access */
/* mstatus: CSR read.
e.g.
    let _v = csr_read_mstatus!();
 */
#[macro_export]
macro_rules! csr_read_mstatus {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mstatus" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mstatus: CSR write 
e.g.
    csr_write_mstatus!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mstatus {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mstatus, {0}" , in(reg) $x);
        }
    );
}
/* mstatus: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mstatus!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mstatus {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mstatus, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mstatus: CSR Field Modifications - via register */
/* Register CSR bit set instructions.
e.g.
csr_set_bits_mstatus!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_mstatus {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, mstatus, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_mstatus!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_mstatus {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, mstatus, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_mstatus!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_mstatus {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {0}, mstatus, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_mstatus!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_mstatus {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {0}, mstatus, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* mstatus: CSR Field Modifications - via immediate */
/* mstatus, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_mstatus!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_mstatus {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, mstatus, ", stringify!($value)));
        }
    );
}

/* mstatus, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_mstatus!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_mstatus {
    ( MSTATUS_MIE_BIT_MASK) => { csr_set_bits_imm_mstatus!(0x8)};
    ( MSTATUS_SIE_BIT_MASK) => { csr_set_bits_imm_mstatus!(0x4)};
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, mstatus, ", stringify!($value)));
        }
    );
}
/* mstatus, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_mstatus!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_mstatus {
    ( MSTATUS_MIE_BIT_MASK) => { csr_clr_bits_imm_mstatus!(0x8)};
    ( MSTATUS_SIE_BIT_MASK) => { csr_clr_bits_imm_mstatus!(0x4)};
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, mstatus, ", stringify!($value)));
        }
    );
}

/*******************************************
 * mstatush - MRW - Additional machine status register, RV32 only. 
 */
/* mstatush: CSR Whole register access */
/* mstatush: CSR read.
e.g.
    let _v = csr_read_mstatush!();
 */
#[macro_export]
macro_rules! csr_read_mstatush {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mstatush" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mstatush: CSR write 
e.g.
    csr_write_mstatush!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mstatush {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mstatush, {0}" , in(reg) $x);
        }
    );
}
/* mstatush: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mstatush!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mstatush {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mstatush, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mstatush: CSR Field Modifications - via register */

/*******************************************
 * mtvec - MRW - Machine Trap Vector Base Address 
 */
/* mtvec: CSR Whole register access */
/* mtvec: CSR read.
e.g.
    let _v = csr_read_mtvec!();
 */
#[macro_export]
macro_rules! csr_read_mtvec {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mtvec" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mtvec: CSR write 
e.g.
    csr_write_mtvec!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mtvec {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mtvec, {0}" , in(reg) $x);
        }
    );
}
/* mtvec: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mtvec!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mtvec {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mtvec, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mtvec: CSR Field Modifications - via register */
/* Register CSR bit set instructions.
e.g.
csr_set_bits_mtvec!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_mtvec {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, mtvec, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_mtvec!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_mtvec {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, mtvec, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_mtvec!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_mtvec {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {0}, mtvec, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_mtvec!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_mtvec {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {0}, mtvec, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* mtvec: CSR Field Modifications - via immediate */
/* mtvec, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_mtvec!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_mtvec {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, mtvec, ", stringify!($value)));
        }
    );
}

/* mtvec, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_mtvec!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_mtvec {
    ( MTVEC_MODE_BIT_MASK) => { csr_set_bits_imm_mtvec!(0x3)};
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, mtvec, ", stringify!($value)));
        }
    );
}
/* mtvec, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_mtvec!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_mtvec {
    ( MTVEC_MODE_BIT_MASK) => { csr_clr_bits_imm_mtvec!(0x3)};
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, mtvec, ", stringify!($value)));
        }
    );
}

/*******************************************
 * medeleg - MRW - Machine Exception Delegation 
 */
/* medeleg: CSR Whole register access */
/* medeleg: CSR read.
e.g.
    let _v = csr_read_medeleg!();
 */
#[macro_export]
macro_rules! csr_read_medeleg {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, medeleg" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* medeleg: CSR write 
e.g.
    csr_write_medeleg!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_medeleg {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    medeleg, {0}" , in(reg) $x);
        }
    );
}
/* medeleg: CSR Read and Write 
e.g.
    let v_ = csr_read_write_medeleg!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_medeleg {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, medeleg, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* medeleg: CSR Field Modifications - via register */

/*******************************************
 * mideleg - MRW - Machine Interrupt Delegation 
 */
/* mideleg: CSR Whole register access */
/* mideleg: CSR read.
e.g.
    let _v = csr_read_mideleg!();
 */
#[macro_export]
macro_rules! csr_read_mideleg {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mideleg" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mideleg: CSR write 
e.g.
    csr_write_mideleg!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mideleg {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mideleg, {0}" , in(reg) $x);
        }
    );
}
/* mideleg: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mideleg!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mideleg {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mideleg, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mideleg: CSR Field Modifications - via register */

/*******************************************
 * mip - MRW - Machine Interrupt Pending 
 */
/* mip: CSR Whole register access */
/* mip: CSR read.
e.g.
    let _v = csr_read_mip!();
 */
#[macro_export]
macro_rules! csr_read_mip {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mip" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mip: CSR write 
e.g.
    csr_write_mip!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mip {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mip, {0}" , in(reg) $x);
        }
    );
}
/* mip: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mip!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mip {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mip, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mip: CSR Field Modifications - via register */
/* Register CSR bit set instructions.
e.g.
csr_set_bits_mip!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_mip {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, mip, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_mip!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_mip {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, mip, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_mip!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_mip {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {0}, mip, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_mip!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_mip {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {0}, mip, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* mip: CSR Field Modifications - via immediate */
/* mip, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_mip!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_mip {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, mip, ", stringify!($value)));
        }
    );
}

/* mip, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_mip!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_mip {
    ( MIP_MSI_BIT_MASK) => { csr_set_bits_imm_mip!(0x8)};
    ( MIP_SSI_BIT_MASK) => { csr_set_bits_imm_mip!(0x2)};
    ( MIP_USI_BIT_MASK) => { csr_set_bits_imm_mip!(0x1)};
    ( MIP_UTI_BIT_MASK) => { csr_set_bits_imm_mip!(0x10)};
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, mip, ", stringify!($value)));
        }
    );
}
/* mip, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_mip!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_mip {
    ( MIP_MSI_BIT_MASK) => { csr_clr_bits_imm_mip!(0x8)};
    ( MIP_SSI_BIT_MASK) => { csr_clr_bits_imm_mip!(0x2)};
    ( MIP_USI_BIT_MASK) => { csr_clr_bits_imm_mip!(0x1)};
    ( MIP_UTI_BIT_MASK) => { csr_clr_bits_imm_mip!(0x10)};
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, mip, ", stringify!($value)));
        }
    );
}

/*******************************************
 * mie - MRW - Machine Interrupt Enable 
 */
/* mie: CSR Whole register access */
/* mie: CSR read.
e.g.
    let _v = csr_read_mie!();
 */
#[macro_export]
macro_rules! csr_read_mie {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mie" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mie: CSR write 
e.g.
    csr_write_mie!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mie {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mie, {0}" , in(reg) $x);
        }
    );
}
/* mie: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mie!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mie {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mie, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mie: CSR Field Modifications - via register */
/* Register CSR bit set instructions.
e.g.
csr_set_bits_mie!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_mie {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, mie, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_mie!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_mie {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, mie, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_mie!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_mie {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {0}, mie, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_mie!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_mie {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {0}, mie, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* mie: CSR Field Modifications - via immediate */
/* mie, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_mie!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_mie {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, mie, ", stringify!($value)));
        }
    );
}

/* mie, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_mie!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_mie {
    ( MIE_MSI_BIT_MASK) => { csr_set_bits_imm_mie!(0x8)};
    ( MIE_SSI_BIT_MASK) => { csr_set_bits_imm_mie!(0x2)};
    ( MIE_USI_BIT_MASK) => { csr_set_bits_imm_mie!(0x1)};
    ( MIE_UTI_BIT_MASK) => { csr_set_bits_imm_mie!(0x10)};
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, mie, ", stringify!($value)));
        }
    );
}
/* mie, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_mie!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_mie {
    ( MIE_MSI_BIT_MASK) => { csr_clr_bits_imm_mie!(0x8)};
    ( MIE_SSI_BIT_MASK) => { csr_clr_bits_imm_mie!(0x2)};
    ( MIE_USI_BIT_MASK) => { csr_clr_bits_imm_mie!(0x1)};
    ( MIE_UTI_BIT_MASK) => { csr_clr_bits_imm_mie!(0x10)};
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, mie, ", stringify!($value)));
        }
    );
}

/*******************************************
 * mcountinhibit - MRW - Machine Counter Inhibit 
 */
/* mcountinhibit: CSR Whole register access */
/* mcountinhibit: CSR read.
e.g.
    let _v = csr_read_mcountinhibit!();
 */
#[macro_export]
macro_rules! csr_read_mcountinhibit {
    ( ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mcountinhibit" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mcountinhibit: CSR write 
e.g.
    csr_write_mcountinhibit!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mcountinhibit {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mcountinhibit, {0}" , in(reg) $x);
        }
    );
}
/* mcountinhibit: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mcountinhibit!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mcountinhibit {
    ( $x:expr ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mcountinhibit, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mcountinhibit: CSR Field Modifications - via register */
/* Register CSR bit set instructions.
e.g.
csr_set_bits_mcountinhibit!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_mcountinhibit {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, mcountinhibit, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_mcountinhibit!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_mcountinhibit {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, mcountinhibit, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_mcountinhibit!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_mcountinhibit {
    ( $mask:expr ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {0}, mcountinhibit, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_mcountinhibit!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_mcountinhibit {
    ( $mask:expr ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {0}, mcountinhibit, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* mcountinhibit: CSR Field Modifications - via immediate */
/* mcountinhibit, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_mcountinhibit!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_mcountinhibit {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, mcountinhibit, ", stringify!($value)));
        }
    );
}

/* mcountinhibit, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_mcountinhibit!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_mcountinhibit {
    ( MCOUNTINHIBIT_CY_BIT_MASK) => { csr_set_bits_imm_mcountinhibit!(0x1)};
    ( MCOUNTINHIBIT_IR_BIT_MASK) => { csr_set_bits_imm_mcountinhibit!(0x4)};
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, mcountinhibit, ", stringify!($value)));
        }
    );
}
/* mcountinhibit, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_mcountinhibit!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_mcountinhibit {
    ( MCOUNTINHIBIT_CY_BIT_MASK) => { csr_clr_bits_imm_mcountinhibit!(0x1)};
    ( MCOUNTINHIBIT_IR_BIT_MASK) => { csr_clr_bits_imm_mcountinhibit!(0x4)};
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, mcountinhibit, ", stringify!($value)));
        }
    );
}

/*******************************************
 * mcycle - MRW - Clock Cycles Executed Counter 
 */
/* mcycle: CSR Whole register access */
/* mcycle: CSR read.
e.g.
    let _v = csr_read_mcycle!();
 */
#[macro_export]
macro_rules! csr_read_mcycle {
    ( ) => (
        {
            let tmp_value: UintCsr64;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mcycle" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mcycle: CSR write 
e.g.
    csr_write_mcycle!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mcycle {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mcycle, {0}" , in(reg) $x);
        }
    );
}
/* mcycle: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mcycle!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mcycle {
    ( $x:expr ) => (
        {
            let tmp_value: UintCsr64;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mcycle, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mcycle: CSR Field Modifications - via register */

/*******************************************
 * minstret - MRW - Number of Instructions Retired Counter 
 */
/* minstret: CSR Whole register access */
/* minstret: CSR read.
e.g.
    let _v = csr_read_minstret!();
 */
#[macro_export]
macro_rules! csr_read_minstret {
    ( ) => (
        {
            let tmp_value: UintCsr64;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, minstret" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* minstret: CSR write 
e.g.
    csr_write_minstret!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_minstret {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    minstret, {0}" , in(reg) $x);
        }
    );
}
/* minstret: CSR Read and Write 
e.g.
    let v_ = csr_read_write_minstret!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_minstret {
    ( $x:expr ) => (
        {
            let tmp_value: UintCsr64;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, minstret, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* minstret: CSR Field Modifications - via register */

/*******************************************
 * mhpmcounter3 - MRW - Event Counters 
 */
/* mhpmcounter3: CSR Whole register access */
/* mhpmcounter3: CSR read.
e.g.
    let _v = csr_read_mhpmcounter3!();
 */
#[macro_export]
macro_rules! csr_read_mhpmcounter3 {
    ( ) => (
        {
            let tmp_value: UintCsr64;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mhpmcounter3" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mhpmcounter3: CSR write 
e.g.
    csr_write_mhpmcounter3!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mhpmcounter3 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mhpmcounter3, {0}" , in(reg) $x);
        }
    );
}
/* mhpmcounter3: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mhpmcounter3!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mhpmcounter3 {
    ( $x:expr ) => (
        {
            let tmp_value: UintCsr64;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mhpmcounter3, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mhpmcounter3: CSR Field Modifications - via register */

/*******************************************
 * mhpmevent3 - MRW - Event Counter Event Select 
 */
/* mhpmevent3: CSR Whole register access */
/* mhpmevent3: CSR read.
e.g.
    let _v = csr_read_mhpmevent3!();
 */
#[macro_export]
macro_rules! csr_read_mhpmevent3 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mhpmevent3" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mhpmevent3: CSR write 
e.g.
    csr_write_mhpmevent3!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mhpmevent3 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mhpmevent3, {0}" , in(reg) $x);
        }
    );
}
/* mhpmevent3: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mhpmevent3!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mhpmevent3 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mhpmevent3, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mhpmevent3: CSR Field Modifications - via register */

/*******************************************
 * mcounteren - MRW - Counter Enable 
 */
/* mcounteren: CSR Whole register access */
/* mcounteren: CSR read.
e.g.
    let _v = csr_read_mcounteren!();
 */
#[macro_export]
macro_rules! csr_read_mcounteren {
    ( ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mcounteren" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mcounteren: CSR write 
e.g.
    csr_write_mcounteren!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mcounteren {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mcounteren, {0}" , in(reg) $x);
        }
    );
}
/* mcounteren: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mcounteren!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mcounteren {
    ( $x:expr ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mcounteren, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mcounteren: CSR Field Modifications - via register */
/* Register CSR bit set instructions.
e.g.
csr_set_bits_mcounteren!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_mcounteren {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, mcounteren, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_mcounteren!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_mcounteren {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, mcounteren, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_mcounteren!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_mcounteren {
    ( $mask:expr ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {0}, mcounteren, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_mcounteren!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_mcounteren {
    ( $mask:expr ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {0}, mcounteren, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* mcounteren: CSR Field Modifications - via immediate */
/* mcounteren, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_mcounteren!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_mcounteren {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, mcounteren, ", stringify!($value)));
        }
    );
}

/* mcounteren, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_mcounteren!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_mcounteren {
    ( MCOUNTEREN_CY_BIT_MASK) => { csr_set_bits_imm_mcounteren!(0x1)};
    ( MCOUNTEREN_TM_BIT_MASK) => { csr_set_bits_imm_mcounteren!(0x2)};
    ( MCOUNTEREN_IR_BIT_MASK) => { csr_set_bits_imm_mcounteren!(0x4)};
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, mcounteren, ", stringify!($value)));
        }
    );
}
/* mcounteren, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_mcounteren!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_mcounteren {
    ( MCOUNTEREN_CY_BIT_MASK) => { csr_clr_bits_imm_mcounteren!(0x1)};
    ( MCOUNTEREN_TM_BIT_MASK) => { csr_clr_bits_imm_mcounteren!(0x2)};
    ( MCOUNTEREN_IR_BIT_MASK) => { csr_clr_bits_imm_mcounteren!(0x4)};
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, mcounteren, ", stringify!($value)));
        }
    );
}

/*******************************************
 * scounteren - SRW - Counter Enable 
 */
/* scounteren: CSR Whole register access */
/* scounteren: CSR read.
e.g.
    let _v = csr_read_scounteren!();
 */
#[macro_export]
macro_rules! csr_read_scounteren {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, scounteren" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* scounteren: CSR write 
e.g.
    csr_write_scounteren!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_scounteren {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    scounteren, {0}" , in(reg) $x);
        }
    );
}
/* scounteren: CSR Read and Write 
e.g.
    let v_ = csr_read_write_scounteren!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_scounteren {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, scounteren, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* scounteren: CSR Field Modifications - via register */

/*******************************************
 * mscratch - MRW - Machine Mode Scratch Register 
 */
/* mscratch: CSR Whole register access */
/* mscratch: CSR read.
e.g.
    let _v = csr_read_mscratch!();
 */
#[macro_export]
macro_rules! csr_read_mscratch {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mscratch" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mscratch: CSR write 
e.g.
    csr_write_mscratch!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mscratch {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mscratch, {0}" , in(reg) $x);
        }
    );
}
/* mscratch: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mscratch!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mscratch {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mscratch, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mscratch: CSR Field Modifications - via register */

/*******************************************
 * mepc - MRW - Machine Exception Program Counter 
 */
/* mepc: CSR Whole register access */
/* mepc: CSR read.
e.g.
    let _v = csr_read_mepc!();
 */
#[macro_export]
macro_rules! csr_read_mepc {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mepc" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mepc: CSR write 
e.g.
    csr_write_mepc!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mepc {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mepc, {0}" , in(reg) $x);
        }
    );
}
/* mepc: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mepc!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mepc {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mepc, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mepc: CSR Field Modifications - via register */

/*******************************************
 * mcause - MRW - Machine Exception Cause 
 */
/* mcause: CSR Whole register access */
/* mcause: CSR read.
e.g.
    let _v = csr_read_mcause!();
 */
#[macro_export]
macro_rules! csr_read_mcause {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mcause" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mcause: CSR write 
e.g.
    csr_write_mcause!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mcause {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mcause, {0}" , in(reg) $x);
        }
    );
}
/* mcause: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mcause!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mcause {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mcause, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mcause: CSR Field Modifications - via register */
/* Register CSR bit set instructions.
e.g.
csr_set_bits_mcause!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_mcause {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, mcause, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_mcause!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_mcause {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, mcause, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_mcause!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_mcause {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {0}, mcause, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_mcause!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_mcause {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {0}, mcause, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* mcause: CSR Field Modifications - via immediate */
/* mcause, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_mcause!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_mcause {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, mcause, ", stringify!($value)));
        }
    );
}

/* mcause, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_mcause!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_mcause {
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, mcause, ", stringify!($value)));
        }
    );
}
/* mcause, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_mcause!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_mcause {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, mcause, ", stringify!($value)));
        }
    );
}

/*******************************************
 * mtval - MRW - Machine Trap Value 
 */
/* mtval: CSR Whole register access */
/* mtval: CSR read.
e.g.
    let _v = csr_read_mtval!();
 */
#[macro_export]
macro_rules! csr_read_mtval {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mtval" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mtval: CSR write 
e.g.
    csr_write_mtval!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mtval {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mtval, {0}" , in(reg) $x);
        }
    );
}
/* mtval: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mtval!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mtval {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mtval, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mtval: CSR Field Modifications - via register */

/*******************************************
 * sscratch - SRW - Supervisor Mode Scratch Register 
 */
/* sscratch: CSR Whole register access */
/* sscratch: CSR read.
e.g.
    let _v = csr_read_sscratch!();
 */
#[macro_export]
macro_rules! csr_read_sscratch {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, sscratch" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* sscratch: CSR write 
e.g.
    csr_write_sscratch!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_sscratch {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    sscratch, {0}" , in(reg) $x);
        }
    );
}
/* sscratch: CSR Read and Write 
e.g.
    let v_ = csr_read_write_sscratch!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_sscratch {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, sscratch, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* sscratch: CSR Field Modifications - via register */

/*******************************************
 * sepc - SRW - Supervisor Exception Program Counter 
 */
/* sepc: CSR Whole register access */
/* sepc: CSR read.
e.g.
    let _v = csr_read_sepc!();
 */
#[macro_export]
macro_rules! csr_read_sepc {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, sepc" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* sepc: CSR write 
e.g.
    csr_write_sepc!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_sepc {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    sepc, {0}" , in(reg) $x);
        }
    );
}
/* sepc: CSR Read and Write 
e.g.
    let v_ = csr_read_write_sepc!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_sepc {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, sepc, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* sepc: CSR Field Modifications - via register */

/*******************************************
 * scause - SRW - Supervisor Exception Cause 
 */
/* scause: CSR Whole register access */
/* scause: CSR read.
e.g.
    let _v = csr_read_scause!();
 */
#[macro_export]
macro_rules! csr_read_scause {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, scause" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* scause: CSR write 
e.g.
    csr_write_scause!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_scause {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    scause, {0}" , in(reg) $x);
        }
    );
}
/* scause: CSR Read and Write 
e.g.
    let v_ = csr_read_write_scause!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_scause {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, scause, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* scause: CSR Field Modifications - via register */
/* Register CSR bit set instructions.
e.g.
csr_set_bits_scause!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_scause {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, scause, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_scause!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_scause {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, scause, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_scause!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_scause {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {0}, scause, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_scause!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_scause {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {0}, scause, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* scause: CSR Field Modifications - via immediate */
/* scause, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_scause!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_scause {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, scause, ", stringify!($value)));
        }
    );
}

/* scause, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_scause!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_scause {
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, scause, ", stringify!($value)));
        }
    );
}
/* scause, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_scause!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_scause {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, scause, ", stringify!($value)));
        }
    );
}

/*******************************************
 * sstatus - SRW - Supervisor Status 
 */
/* sstatus: CSR Whole register access */
/* sstatus: CSR read.
e.g.
    let _v = csr_read_sstatus!();
 */
#[macro_export]
macro_rules! csr_read_sstatus {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, sstatus" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* sstatus: CSR write 
e.g.
    csr_write_sstatus!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_sstatus {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    sstatus, {0}" , in(reg) $x);
        }
    );
}
/* sstatus: CSR Read and Write 
e.g.
    let v_ = csr_read_write_sstatus!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_sstatus {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, sstatus, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* sstatus: CSR Field Modifications - via register */
/* Register CSR bit set instructions.
e.g.
csr_set_bits_sstatus!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_sstatus {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, sstatus, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_sstatus!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_sstatus {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, sstatus, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_sstatus!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_sstatus {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {0}, sstatus, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_sstatus!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_sstatus {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {0}, sstatus, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* sstatus: CSR Field Modifications - via immediate */
/* sstatus, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_sstatus!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_sstatus {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, sstatus, ", stringify!($value)));
        }
    );
}

/* sstatus, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_sstatus!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_sstatus {
    ( SSTATUS_SIE_BIT_MASK) => { csr_set_bits_imm_sstatus!(0x4)};
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, sstatus, ", stringify!($value)));
        }
    );
}
/* sstatus, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_sstatus!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_sstatus {
    ( SSTATUS_SIE_BIT_MASK) => { csr_clr_bits_imm_sstatus!(0x4)};
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, sstatus, ", stringify!($value)));
        }
    );
}

/*******************************************
 * stvec - SRW - Supervisor Trap Vector Base Address 
 */
/* stvec: CSR Whole register access */
/* stvec: CSR read.
e.g.
    let _v = csr_read_stvec!();
 */
#[macro_export]
macro_rules! csr_read_stvec {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, stvec" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* stvec: CSR write 
e.g.
    csr_write_stvec!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_stvec {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    stvec, {0}" , in(reg) $x);
        }
    );
}
/* stvec: CSR Read and Write 
e.g.
    let v_ = csr_read_write_stvec!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_stvec {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, stvec, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* stvec: CSR Field Modifications - via register */
/* Register CSR bit set instructions.
e.g.
csr_set_bits_stvec!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_stvec {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, stvec, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_stvec!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_stvec {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, stvec, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_stvec!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_stvec {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {0}, stvec, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_stvec!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_stvec {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {0}, stvec, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* stvec: CSR Field Modifications - via immediate */
/* stvec, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_stvec!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_stvec {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, stvec, ", stringify!($value)));
        }
    );
}

/* stvec, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_stvec!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_stvec {
    ( STVEC_MODE_BIT_MASK) => { csr_set_bits_imm_stvec!(0x3)};
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, stvec, ", stringify!($value)));
        }
    );
}
/* stvec, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_stvec!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_stvec {
    ( STVEC_MODE_BIT_MASK) => { csr_clr_bits_imm_stvec!(0x3)};
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, stvec, ", stringify!($value)));
        }
    );
}

/*******************************************
 * sideleg - SRW - Supervisor Interrupt Delegation 
 */
/* sideleg: CSR Whole register access */
/* sideleg: CSR read.
e.g.
    let _v = csr_read_sideleg!();
 */
#[macro_export]
macro_rules! csr_read_sideleg {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, sideleg" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* sideleg: CSR write 
e.g.
    csr_write_sideleg!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_sideleg {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    sideleg, {0}" , in(reg) $x);
        }
    );
}
/* sideleg: CSR Read and Write 
e.g.
    let v_ = csr_read_write_sideleg!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_sideleg {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, sideleg, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* sideleg: CSR Field Modifications - via register */

/*******************************************
 * sedeleg - SRW - Supervisor Exception Delegation 
 */
/* sedeleg: CSR Whole register access */
/* sedeleg: CSR read.
e.g.
    let _v = csr_read_sedeleg!();
 */
#[macro_export]
macro_rules! csr_read_sedeleg {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, sedeleg" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* sedeleg: CSR write 
e.g.
    csr_write_sedeleg!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_sedeleg {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    sedeleg, {0}" , in(reg) $x);
        }
    );
}
/* sedeleg: CSR Read and Write 
e.g.
    let v_ = csr_read_write_sedeleg!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_sedeleg {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, sedeleg, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* sedeleg: CSR Field Modifications - via register */

/*******************************************
 * sip - SRW - Supervisor Interrupt Pending 
 */
/* sip: CSR Whole register access */
/* sip: CSR read.
e.g.
    let _v = csr_read_sip!();
 */
#[macro_export]
macro_rules! csr_read_sip {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, sip" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* sip: CSR write 
e.g.
    csr_write_sip!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_sip {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    sip, {0}" , in(reg) $x);
        }
    );
}
/* sip: CSR Read and Write 
e.g.
    let v_ = csr_read_write_sip!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_sip {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, sip, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* sip: CSR Field Modifications - via register */
/* Register CSR bit set instructions.
e.g.
csr_set_bits_sip!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_sip {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, sip, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_sip!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_sip {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, sip, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_sip!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_sip {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {0}, sip, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_sip!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_sip {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {0}, sip, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* sip: CSR Field Modifications - via immediate */
/* sip, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_sip!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_sip {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, sip, ", stringify!($value)));
        }
    );
}

/* sip, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_sip!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_sip {
    ( SIP_SSI_BIT_MASK) => { csr_set_bits_imm_sip!(0x2)};
    ( SIP_USI_BIT_MASK) => { csr_set_bits_imm_sip!(0x1)};
    ( SIP_UTI_BIT_MASK) => { csr_set_bits_imm_sip!(0x10)};
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, sip, ", stringify!($value)));
        }
    );
}
/* sip, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_sip!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_sip {
    ( SIP_SSI_BIT_MASK) => { csr_clr_bits_imm_sip!(0x2)};
    ( SIP_USI_BIT_MASK) => { csr_clr_bits_imm_sip!(0x1)};
    ( SIP_UTI_BIT_MASK) => { csr_clr_bits_imm_sip!(0x10)};
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, sip, ", stringify!($value)));
        }
    );
}

/*******************************************
 * sie - SRW - Supervisor Interrupt Enable 
 */
/* sie: CSR Whole register access */
/* sie: CSR read.
e.g.
    let _v = csr_read_sie!();
 */
#[macro_export]
macro_rules! csr_read_sie {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, sie" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* sie: CSR write 
e.g.
    csr_write_sie!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_sie {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    sie, {0}" , in(reg) $x);
        }
    );
}
/* sie: CSR Read and Write 
e.g.
    let v_ = csr_read_write_sie!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_sie {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, sie, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* sie: CSR Field Modifications - via register */
/* Register CSR bit set instructions.
e.g.
csr_set_bits_sie!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_sie {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, sie, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_sie!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_sie {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, sie, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_sie!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_sie {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {0}, sie, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_sie!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_sie {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {0}, sie, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* sie: CSR Field Modifications - via immediate */
/* sie, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_sie!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_sie {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, sie, ", stringify!($value)));
        }
    );
}

/* sie, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_sie!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_sie {
    ( SIE_SSI_BIT_MASK) => { csr_set_bits_imm_sie!(0x2)};
    ( SIE_USI_BIT_MASK) => { csr_set_bits_imm_sie!(0x1)};
    ( SIE_UTI_BIT_MASK) => { csr_set_bits_imm_sie!(0x10)};
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, sie, ", stringify!($value)));
        }
    );
}
/* sie, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_sie!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_sie {
    ( SIE_SSI_BIT_MASK) => { csr_clr_bits_imm_sie!(0x2)};
    ( SIE_USI_BIT_MASK) => { csr_clr_bits_imm_sie!(0x1)};
    ( SIE_UTI_BIT_MASK) => { csr_clr_bits_imm_sie!(0x10)};
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, sie, ", stringify!($value)));
        }
    );
}

/*******************************************
 * ustatus - URW - User mode restricted view of mstatus 
 */
/* ustatus: CSR Whole register access */
/* ustatus: CSR read.
e.g.
    let _v = csr_read_ustatus!();
 */
#[macro_export]
macro_rules! csr_read_ustatus {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, ustatus" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* ustatus: CSR write 
e.g.
    csr_write_ustatus!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_ustatus {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    ustatus, {0}" , in(reg) $x);
        }
    );
}
/* ustatus: CSR Read and Write 
e.g.
    let v_ = csr_read_write_ustatus!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_ustatus {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, ustatus, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* ustatus: CSR Field Modifications - via register */
/* Register CSR bit set instructions.
e.g.
csr_set_bits_ustatus!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_ustatus {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, ustatus, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_ustatus!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_ustatus {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, ustatus, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_ustatus!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_ustatus {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {0}, ustatus, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_ustatus!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_ustatus {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {0}, ustatus, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* ustatus: CSR Field Modifications - via immediate */
/* ustatus, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_ustatus!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_ustatus {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, ustatus, ", stringify!($value)));
        }
    );
}

/* ustatus, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_ustatus!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_ustatus {
    ( USTATUS_UIE_BIT_MASK) => { csr_set_bits_imm_ustatus!(0x2)};
    ( USTATUS_UPIE_BIT_MASK) => { csr_set_bits_imm_ustatus!(0x8)};
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, ustatus, ", stringify!($value)));
        }
    );
}
/* ustatus, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_ustatus!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_ustatus {
    ( USTATUS_UIE_BIT_MASK) => { csr_clr_bits_imm_ustatus!(0x2)};
    ( USTATUS_UPIE_BIT_MASK) => { csr_clr_bits_imm_ustatus!(0x8)};
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, ustatus, ", stringify!($value)));
        }
    );
}

/*******************************************
 * uip - URW - User Interrupt Pending 
 */
/* uip: CSR Whole register access */
/* uip: CSR read.
e.g.
    let _v = csr_read_uip!();
 */
#[macro_export]
macro_rules! csr_read_uip {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, uip" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* uip: CSR write 
e.g.
    csr_write_uip!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_uip {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    uip, {0}" , in(reg) $x);
        }
    );
}
/* uip: CSR Read and Write 
e.g.
    let v_ = csr_read_write_uip!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_uip {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, uip, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* uip: CSR Field Modifications - via register */
/* Register CSR bit set instructions.
e.g.
csr_set_bits_uip!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_uip {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, uip, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_uip!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_uip {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, uip, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_uip!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_uip {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {0}, uip, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_uip!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_uip {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {0}, uip, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* uip: CSR Field Modifications - via immediate */
/* uip, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_uip!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_uip {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, uip, ", stringify!($value)));
        }
    );
}

/* uip, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_uip!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_uip {
    ( UIP_USI_BIT_MASK) => { csr_set_bits_imm_uip!(0x1)};
    ( UIP_UTI_BIT_MASK) => { csr_set_bits_imm_uip!(0x10)};
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, uip, ", stringify!($value)));
        }
    );
}
/* uip, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_uip!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_uip {
    ( UIP_USI_BIT_MASK) => { csr_clr_bits_imm_uip!(0x1)};
    ( UIP_UTI_BIT_MASK) => { csr_clr_bits_imm_uip!(0x10)};
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, uip, ", stringify!($value)));
        }
    );
}

/*******************************************
 * uie - URW - User Interrupt Enable 
 */
/* uie: CSR Whole register access */
/* uie: CSR read.
e.g.
    let _v = csr_read_uie!();
 */
#[macro_export]
macro_rules! csr_read_uie {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, uie" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* uie: CSR write 
e.g.
    csr_write_uie!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_uie {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    uie, {0}" , in(reg) $x);
        }
    );
}
/* uie: CSR Read and Write 
e.g.
    let v_ = csr_read_write_uie!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_uie {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, uie, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* uie: CSR Field Modifications - via register */
/* Register CSR bit set instructions.
e.g.
csr_set_bits_uie!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_uie {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, uie, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_uie!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_uie {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, uie, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_uie!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_uie {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {0}, uie, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_uie!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_uie {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {0}, uie, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* uie: CSR Field Modifications - via immediate */
/* uie, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_uie!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_uie {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, uie, ", stringify!($value)));
        }
    );
}

/* uie, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_uie!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_uie {
    ( UIE_USI_BIT_MASK) => { csr_set_bits_imm_uie!(0x1)};
    ( UIE_UTI_BIT_MASK) => { csr_set_bits_imm_uie!(0x10)};
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, uie, ", stringify!($value)));
        }
    );
}
/* uie, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_uie!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_uie {
    ( UIE_USI_BIT_MASK) => { csr_clr_bits_imm_uie!(0x1)};
    ( UIE_UTI_BIT_MASK) => { csr_clr_bits_imm_uie!(0x10)};
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, uie, ", stringify!($value)));
        }
    );
}

/*******************************************
 * uscratch - URW - User Mode Scratch Register 
 */
/* uscratch: CSR Whole register access */
/* uscratch: CSR read.
e.g.
    let _v = csr_read_uscratch!();
 */
#[macro_export]
macro_rules! csr_read_uscratch {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, uscratch" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* uscratch: CSR write 
e.g.
    csr_write_uscratch!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_uscratch {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    uscratch, {0}" , in(reg) $x);
        }
    );
}
/* uscratch: CSR Read and Write 
e.g.
    let v_ = csr_read_write_uscratch!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_uscratch {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, uscratch, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* uscratch: CSR Field Modifications - via register */

/*******************************************
 * uepc - URW - User Exception Program Counter 
 */
/* uepc: CSR Whole register access */
/* uepc: CSR read.
e.g.
    let _v = csr_read_uepc!();
 */
#[macro_export]
macro_rules! csr_read_uepc {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, uepc" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* uepc: CSR write 
e.g.
    csr_write_uepc!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_uepc {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    uepc, {0}" , in(reg) $x);
        }
    );
}
/* uepc: CSR Read and Write 
e.g.
    let v_ = csr_read_write_uepc!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_uepc {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, uepc, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* uepc: CSR Field Modifications - via register */

/*******************************************
 * ucause - URW - User Exception Cause 
 */
/* ucause: CSR Whole register access */
/* ucause: CSR read.
e.g.
    let _v = csr_read_ucause!();
 */
#[macro_export]
macro_rules! csr_read_ucause {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, ucause" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* ucause: CSR write 
e.g.
    csr_write_ucause!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_ucause {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    ucause, {0}" , in(reg) $x);
        }
    );
}
/* ucause: CSR Read and Write 
e.g.
    let v_ = csr_read_write_ucause!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_ucause {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, ucause, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* ucause: CSR Field Modifications - via register */
/* Register CSR bit set instructions.
e.g.
csr_set_bits_ucause!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_ucause {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, ucause, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_ucause!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_ucause {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, ucause, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_ucause!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_ucause {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {0}, ucause, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_ucause!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_ucause {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {0}, ucause, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* ucause: CSR Field Modifications - via immediate */
/* ucause, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_ucause!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_ucause {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, ucause, ", stringify!($value)));
        }
    );
}

/* ucause, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_ucause!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_ucause {
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, ucause, ", stringify!($value)));
        }
    );
}
/* ucause, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_ucause!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_ucause {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, ucause, ", stringify!($value)));
        }
    );
}

/*******************************************
 * utvec - URW - User Trap Vector Base Address 
 */
/* utvec: CSR Whole register access */
/* utvec: CSR read.
e.g.
    let _v = csr_read_utvec!();
 */
#[macro_export]
macro_rules! csr_read_utvec {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, utvec" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* utvec: CSR write 
e.g.
    csr_write_utvec!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_utvec {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    utvec, {0}" , in(reg) $x);
        }
    );
}
/* utvec: CSR Read and Write 
e.g.
    let v_ = csr_read_write_utvec!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_utvec {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, utvec, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* utvec: CSR Field Modifications - via register */
/* Register CSR bit set instructions.
e.g.
csr_set_bits_utvec!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_utvec {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, utvec, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_utvec!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_utvec {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, utvec, {0}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_utvec!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_utvec {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {0}, utvec, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_utvec!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_utvec {
    ( $mask:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {0}, utvec, {1}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* utvec: CSR Field Modifications - via immediate */
/* utvec, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_utvec!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_utvec {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, utvec, ", stringify!($value)));
        }
    );
}

/* utvec, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_utvec!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_utvec {
    ( UTVEC_MODE_BIT_MASK) => { csr_set_bits_imm_utvec!(0x3)};
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, utvec, ", stringify!($value)));
        }
    );
}
/* utvec, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_utvec!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_utvec {
    ( UTVEC_MODE_BIT_MASK) => { csr_clr_bits_imm_utvec!(0x3)};
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, utvec, ", stringify!($value)));
        }
    );
}

/*******************************************
 * utval - URW - User Trap Value 
 */
/* utval: CSR Whole register access */
/* utval: CSR read.
e.g.
    let _v = csr_read_utval!();
 */
#[macro_export]
macro_rules! csr_read_utval {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, utval" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* utval: CSR write 
e.g.
    csr_write_utval!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_utval {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    utval, {0}" , in(reg) $x);
        }
    );
}
/* utval: CSR Read and Write 
e.g.
    let v_ = csr_read_write_utval!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_utval {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, utval, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* utval: CSR Field Modifications - via register */

/*******************************************
 * fflags - URW - Floating-Point Accrued Exceptions. 
 */
/* fflags: CSR Whole register access */
/* fflags: CSR read.
e.g.
    let _v = csr_read_fflags!();
 */
#[macro_export]
macro_rules! csr_read_fflags {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, fflags" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* fflags: CSR write 
e.g.
    csr_write_fflags!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_fflags {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    fflags, {0}" , in(reg) $x);
        }
    );
}
/* fflags: CSR Read and Write 
e.g.
    let v_ = csr_read_write_fflags!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_fflags {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, fflags, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* fflags: CSR Field Modifications - via register */

/*******************************************
 * frm - URW - Floating-Point Dynamic Rounding Mode. 
 */
/* frm: CSR Whole register access */
/* frm: CSR read.
e.g.
    let _v = csr_read_frm!();
 */
#[macro_export]
macro_rules! csr_read_frm {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, frm" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* frm: CSR write 
e.g.
    csr_write_frm!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_frm {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    frm, {0}" , in(reg) $x);
        }
    );
}
/* frm: CSR Read and Write 
e.g.
    let v_ = csr_read_write_frm!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_frm {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, frm, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* frm: CSR Field Modifications - via register */

/*******************************************
 * fcsr - URW - Floating-Point Control and Status 
 */
/* fcsr: CSR Whole register access */
/* fcsr: CSR read.
e.g.
    let _v = csr_read_fcsr!();
 */
#[macro_export]
macro_rules! csr_read_fcsr {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, fcsr" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* fcsr: CSR write 
e.g.
    csr_write_fcsr!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_fcsr {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    fcsr, {0}" , in(reg) $x);
        }
    );
}
/* fcsr: CSR Read and Write 
e.g.
    let v_ = csr_read_write_fcsr!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_fcsr {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, fcsr, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* fcsr: CSR Field Modifications - via register */

/*******************************************
 * cycle - URO - Cycle counter for RDCYCLE instruction. 
 */
/* cycle: CSR Whole register access */
/* cycle: CSR read.
e.g.
    let _v = csr_read_cycle!();
 */
#[macro_export]
macro_rules! csr_read_cycle {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, cycle" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}

/* cycle: CSR Field Modifications - via register */

/*******************************************
 * time - URO - Timer for RDTIME instruction. 
 */
/* time: CSR Whole register access */
/* time: CSR read.
e.g.
    let _v = csr_read_time!();
 */
#[macro_export]
macro_rules! csr_read_time {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, time" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}

/* time: CSR Field Modifications - via register */

/*******************************************
 * instret - URO - Instructions-retired counter for RDINSTRET instruction. 
 */
/* instret: CSR Whole register access */
/* instret: CSR read.
e.g.
    let _v = csr_read_instret!();
 */
#[macro_export]
macro_rules! csr_read_instret {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, instret" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}

/* instret: CSR Field Modifications - via register */

/*******************************************
 * hpmcounter3 - URO - Performance-monitoring counter. 
 */
/* hpmcounter3: CSR Whole register access */
/* hpmcounter3: CSR read.
e.g.
    let _v = csr_read_hpmcounter3!();
 */
#[macro_export]
macro_rules! csr_read_hpmcounter3 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, hpmcounter3" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}

/* hpmcounter3: CSR Field Modifications - via register */

/*******************************************
 * hpmcounter4 - URO - Performance-monitoring counter. 
 */
/* hpmcounter4: CSR Whole register access */
/* hpmcounter4: CSR read.
e.g.
    let _v = csr_read_hpmcounter4!();
 */
#[macro_export]
macro_rules! csr_read_hpmcounter4 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, hpmcounter4" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}

/* hpmcounter4: CSR Field Modifications - via register */

/*******************************************
 * hpmcounter31 - URO - Performance-monitoring counter. 
 */
/* hpmcounter31: CSR Whole register access */
/* hpmcounter31: CSR read.
e.g.
    let _v = csr_read_hpmcounter31!();
 */
#[macro_export]
macro_rules! csr_read_hpmcounter31 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, hpmcounter31" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}

/* hpmcounter31: CSR Field Modifications - via register */

/*******************************************
 * cycleh - URO - Upper 32 bits of  cycle, RV32I only. 
 */
/* cycleh: CSR Whole register access */
/* cycleh: CSR read.
e.g.
    let _v = csr_read_cycleh!();
 */
#[macro_export]
macro_rules! csr_read_cycleh {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, cycleh" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}

/* cycleh: CSR Field Modifications - via register */

/*******************************************
 * timeh - URO - Upper 32 bits of  time, RV32I only. 
 */
/* timeh: CSR Whole register access */
/* timeh: CSR read.
e.g.
    let _v = csr_read_timeh!();
 */
#[macro_export]
macro_rules! csr_read_timeh {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, timeh" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}

/* timeh: CSR Field Modifications - via register */

/*******************************************
 * instreth - URO - Upper 32 bits of  instret, RV32I only. 
 */
/* instreth: CSR Whole register access */
/* instreth: CSR read.
e.g.
    let _v = csr_read_instreth!();
 */
#[macro_export]
macro_rules! csr_read_instreth {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, instreth" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}

/* instreth: CSR Field Modifications - via register */

/*******************************************
 * hpmcounter3h - URO - Upper 32 bits of  hpmcounter3, RV32I only. 
 */
/* hpmcounter3h: CSR Whole register access */
/* hpmcounter3h: CSR read.
e.g.
    let _v = csr_read_hpmcounter3h!();
 */
#[macro_export]
macro_rules! csr_read_hpmcounter3h {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, hpmcounter3h" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}

/* hpmcounter3h: CSR Field Modifications - via register */

/*******************************************
 * hpmcounter4h - URO - Upper 32 bits of  hpmcounter4, RV32I only. 
 */
/* hpmcounter4h: CSR Whole register access */
/* hpmcounter4h: CSR read.
e.g.
    let _v = csr_read_hpmcounter4h!();
 */
#[macro_export]
macro_rules! csr_read_hpmcounter4h {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, hpmcounter4h" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}

/* hpmcounter4h: CSR Field Modifications - via register */

/*******************************************
 * hpmcounter31h - URO - Upper 32 bits of  hpmcounter31, RV32I only. 
 */
/* hpmcounter31h: CSR Whole register access */
/* hpmcounter31h: CSR read.
e.g.
    let _v = csr_read_hpmcounter31h!();
 */
#[macro_export]
macro_rules! csr_read_hpmcounter31h {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, hpmcounter31h" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}

/* hpmcounter31h: CSR Field Modifications - via register */

/*******************************************
 * stval - SRW - Supervisor bad address or instruction. 
 */
/* stval: CSR Whole register access */
/* stval: CSR read.
e.g.
    let _v = csr_read_stval!();
 */
#[macro_export]
macro_rules! csr_read_stval {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, stval" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* stval: CSR write 
e.g.
    csr_write_stval!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_stval {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    stval, {0}" , in(reg) $x);
        }
    );
}
/* stval: CSR Read and Write 
e.g.
    let v_ = csr_read_write_stval!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_stval {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, stval, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* stval: CSR Field Modifications - via register */

/*******************************************
 * satp - SRW - Supervisor address translation and protection. 
 */
/* satp: CSR Whole register access */
/* satp: CSR read.
e.g.
    let _v = csr_read_satp!();
 */
#[macro_export]
macro_rules! csr_read_satp {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, satp" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* satp: CSR write 
e.g.
    csr_write_satp!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_satp {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    satp, {0}" , in(reg) $x);
        }
    );
}
/* satp: CSR Read and Write 
e.g.
    let v_ = csr_read_write_satp!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_satp {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, satp, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* satp: CSR Field Modifications - via register */

/*******************************************
 * hstatus - HRW - Hypervisor status register. 
 */
/* hstatus: CSR Whole register access */
/* hstatus: CSR read.
e.g.
    let _v = csr_read_hstatus!();
 */
#[macro_export]
macro_rules! csr_read_hstatus {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, hstatus" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* hstatus: CSR write 
e.g.
    csr_write_hstatus!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_hstatus {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    hstatus, {0}" , in(reg) $x);
        }
    );
}
/* hstatus: CSR Read and Write 
e.g.
    let v_ = csr_read_write_hstatus!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_hstatus {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, hstatus, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* hstatus: CSR Field Modifications - via register */

/*******************************************
 * hedeleg - HRW - Hypervisor exception delegation register. 
 */
/* hedeleg: CSR Whole register access */
/* hedeleg: CSR read.
e.g.
    let _v = csr_read_hedeleg!();
 */
#[macro_export]
macro_rules! csr_read_hedeleg {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, hedeleg" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* hedeleg: CSR write 
e.g.
    csr_write_hedeleg!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_hedeleg {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    hedeleg, {0}" , in(reg) $x);
        }
    );
}
/* hedeleg: CSR Read and Write 
e.g.
    let v_ = csr_read_write_hedeleg!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_hedeleg {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, hedeleg, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* hedeleg: CSR Field Modifications - via register */

/*******************************************
 * hideleg - HRW - Hypervisor interrupt delegation register. 
 */
/* hideleg: CSR Whole register access */
/* hideleg: CSR read.
e.g.
    let _v = csr_read_hideleg!();
 */
#[macro_export]
macro_rules! csr_read_hideleg {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, hideleg" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* hideleg: CSR write 
e.g.
    csr_write_hideleg!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_hideleg {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    hideleg, {0}" , in(reg) $x);
        }
    );
}
/* hideleg: CSR Read and Write 
e.g.
    let v_ = csr_read_write_hideleg!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_hideleg {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, hideleg, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* hideleg: CSR Field Modifications - via register */

/*******************************************
 * hcounteren - HRW - Hypervisor counter enable. 
 */
/* hcounteren: CSR Whole register access */
/* hcounteren: CSR read.
e.g.
    let _v = csr_read_hcounteren!();
 */
#[macro_export]
macro_rules! csr_read_hcounteren {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, hcounteren" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* hcounteren: CSR write 
e.g.
    csr_write_hcounteren!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_hcounteren {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    hcounteren, {0}" , in(reg) $x);
        }
    );
}
/* hcounteren: CSR Read and Write 
e.g.
    let v_ = csr_read_write_hcounteren!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_hcounteren {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, hcounteren, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* hcounteren: CSR Field Modifications - via register */

/*******************************************
 * hgatp - HRW - Hypervisor guest address translation and protection. 
 */
/* hgatp: CSR Whole register access */
/* hgatp: CSR read.
e.g.
    let _v = csr_read_hgatp!();
 */
#[macro_export]
macro_rules! csr_read_hgatp {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, hgatp" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* hgatp: CSR write 
e.g.
    csr_write_hgatp!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_hgatp {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    hgatp, {0}" , in(reg) $x);
        }
    );
}
/* hgatp: CSR Read and Write 
e.g.
    let v_ = csr_read_write_hgatp!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_hgatp {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, hgatp, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* hgatp: CSR Field Modifications - via register */

/*******************************************
 * htimedelta - HRW - Delta for VS/VU-mode timer. 
 */
/* htimedelta: CSR Whole register access */
/* htimedelta: CSR read.
e.g.
    let _v = csr_read_htimedelta!();
 */
#[macro_export]
macro_rules! csr_read_htimedelta {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, htimedelta" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* htimedelta: CSR write 
e.g.
    csr_write_htimedelta!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_htimedelta {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    htimedelta, {0}" , in(reg) $x);
        }
    );
}
/* htimedelta: CSR Read and Write 
e.g.
    let v_ = csr_read_write_htimedelta!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_htimedelta {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, htimedelta, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* htimedelta: CSR Field Modifications - via register */

/*******************************************
 * htimedeltah - HRW - Upper 32 bits of  htimedelta, RV32I only. 
 */
/* htimedeltah: CSR Whole register access */
/* htimedeltah: CSR read.
e.g.
    let _v = csr_read_htimedeltah!();
 */
#[macro_export]
macro_rules! csr_read_htimedeltah {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, htimedeltah" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* htimedeltah: CSR write 
e.g.
    csr_write_htimedeltah!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_htimedeltah {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    htimedeltah, {0}" , in(reg) $x);
        }
    );
}
/* htimedeltah: CSR Read and Write 
e.g.
    let v_ = csr_read_write_htimedeltah!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_htimedeltah {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, htimedeltah, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* htimedeltah: CSR Field Modifications - via register */

/*******************************************
 * vsstatus - HRW - Virtual supervisor status register. 
 */
/* vsstatus: CSR Whole register access */
/* vsstatus: CSR read.
e.g.
    let _v = csr_read_vsstatus!();
 */
#[macro_export]
macro_rules! csr_read_vsstatus {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, vsstatus" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* vsstatus: CSR write 
e.g.
    csr_write_vsstatus!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_vsstatus {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    vsstatus, {0}" , in(reg) $x);
        }
    );
}
/* vsstatus: CSR Read and Write 
e.g.
    let v_ = csr_read_write_vsstatus!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_vsstatus {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, vsstatus, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* vsstatus: CSR Field Modifications - via register */

/*******************************************
 * vsie - HRW - Virtual supervisor interrupt-enable register. 
 */
/* vsie: CSR Whole register access */
/* vsie: CSR read.
e.g.
    let _v = csr_read_vsie!();
 */
#[macro_export]
macro_rules! csr_read_vsie {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, vsie" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* vsie: CSR write 
e.g.
    csr_write_vsie!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_vsie {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    vsie, {0}" , in(reg) $x);
        }
    );
}
/* vsie: CSR Read and Write 
e.g.
    let v_ = csr_read_write_vsie!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_vsie {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, vsie, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* vsie: CSR Field Modifications - via register */

/*******************************************
 * vstvec - HRW - Virtual supervisor trap handler base address. 
 */
/* vstvec: CSR Whole register access */
/* vstvec: CSR read.
e.g.
    let _v = csr_read_vstvec!();
 */
#[macro_export]
macro_rules! csr_read_vstvec {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, vstvec" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* vstvec: CSR write 
e.g.
    csr_write_vstvec!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_vstvec {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    vstvec, {0}" , in(reg) $x);
        }
    );
}
/* vstvec: CSR Read and Write 
e.g.
    let v_ = csr_read_write_vstvec!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_vstvec {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, vstvec, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* vstvec: CSR Field Modifications - via register */

/*******************************************
 * vsscratch - HRW - Virtual supervisor scratch register. 
 */
/* vsscratch: CSR Whole register access */
/* vsscratch: CSR read.
e.g.
    let _v = csr_read_vsscratch!();
 */
#[macro_export]
macro_rules! csr_read_vsscratch {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, vsscratch" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* vsscratch: CSR write 
e.g.
    csr_write_vsscratch!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_vsscratch {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    vsscratch, {0}" , in(reg) $x);
        }
    );
}
/* vsscratch: CSR Read and Write 
e.g.
    let v_ = csr_read_write_vsscratch!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_vsscratch {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, vsscratch, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* vsscratch: CSR Field Modifications - via register */

/*******************************************
 * vsepc - HRW - Virtual supervisor exception program counter. 
 */
/* vsepc: CSR Whole register access */
/* vsepc: CSR read.
e.g.
    let _v = csr_read_vsepc!();
 */
#[macro_export]
macro_rules! csr_read_vsepc {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, vsepc" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* vsepc: CSR write 
e.g.
    csr_write_vsepc!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_vsepc {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    vsepc, {0}" , in(reg) $x);
        }
    );
}
/* vsepc: CSR Read and Write 
e.g.
    let v_ = csr_read_write_vsepc!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_vsepc {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, vsepc, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* vsepc: CSR Field Modifications - via register */

/*******************************************
 * vscause - HRW - Virtual supervisor trap cause. 
 */
/* vscause: CSR Whole register access */
/* vscause: CSR read.
e.g.
    let _v = csr_read_vscause!();
 */
#[macro_export]
macro_rules! csr_read_vscause {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, vscause" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* vscause: CSR write 
e.g.
    csr_write_vscause!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_vscause {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    vscause, {0}" , in(reg) $x);
        }
    );
}
/* vscause: CSR Read and Write 
e.g.
    let v_ = csr_read_write_vscause!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_vscause {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, vscause, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* vscause: CSR Field Modifications - via register */

/*******************************************
 * vstval - HRW - Virtual supervisor bad address or instruction. 
 */
/* vstval: CSR Whole register access */
/* vstval: CSR read.
e.g.
    let _v = csr_read_vstval!();
 */
#[macro_export]
macro_rules! csr_read_vstval {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, vstval" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* vstval: CSR write 
e.g.
    csr_write_vstval!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_vstval {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    vstval, {0}" , in(reg) $x);
        }
    );
}
/* vstval: CSR Read and Write 
e.g.
    let v_ = csr_read_write_vstval!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_vstval {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, vstval, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* vstval: CSR Field Modifications - via register */

/*******************************************
 * vsip - HRW - Virtual supervisor interrupt pending. 
 */
/* vsip: CSR Whole register access */
/* vsip: CSR read.
e.g.
    let _v = csr_read_vsip!();
 */
#[macro_export]
macro_rules! csr_read_vsip {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, vsip" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* vsip: CSR write 
e.g.
    csr_write_vsip!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_vsip {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    vsip, {0}" , in(reg) $x);
        }
    );
}
/* vsip: CSR Read and Write 
e.g.
    let v_ = csr_read_write_vsip!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_vsip {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, vsip, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* vsip: CSR Field Modifications - via register */

/*******************************************
 * vsatp - HRW - Virtual supervisor address translation and protection. 
 */
/* vsatp: CSR Whole register access */
/* vsatp: CSR read.
e.g.
    let _v = csr_read_vsatp!();
 */
#[macro_export]
macro_rules! csr_read_vsatp {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, vsatp" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* vsatp: CSR write 
e.g.
    csr_write_vsatp!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_vsatp {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    vsatp, {0}" , in(reg) $x);
        }
    );
}
/* vsatp: CSR Read and Write 
e.g.
    let v_ = csr_read_write_vsatp!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_vsatp {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, vsatp, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* vsatp: CSR Field Modifications - via register */

/*******************************************
 * mbase - MRW - Base register. 
 */
/* mbase: CSR Whole register access */
/* mbase: CSR read.
e.g.
    let _v = csr_read_mbase!();
 */
#[macro_export]
macro_rules! csr_read_mbase {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mbase" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mbase: CSR write 
e.g.
    csr_write_mbase!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mbase {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mbase, {0}" , in(reg) $x);
        }
    );
}
/* mbase: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mbase!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mbase {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mbase, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mbase: CSR Field Modifications - via register */

/*******************************************
 * mbound - MRW - Bound register. 
 */
/* mbound: CSR Whole register access */
/* mbound: CSR read.
e.g.
    let _v = csr_read_mbound!();
 */
#[macro_export]
macro_rules! csr_read_mbound {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mbound" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mbound: CSR write 
e.g.
    csr_write_mbound!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mbound {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mbound, {0}" , in(reg) $x);
        }
    );
}
/* mbound: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mbound!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mbound {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mbound, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mbound: CSR Field Modifications - via register */

/*******************************************
 * mibase - MRW - Instruction base register. 
 */
/* mibase: CSR Whole register access */
/* mibase: CSR read.
e.g.
    let _v = csr_read_mibase!();
 */
#[macro_export]
macro_rules! csr_read_mibase {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mibase" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mibase: CSR write 
e.g.
    csr_write_mibase!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mibase {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mibase, {0}" , in(reg) $x);
        }
    );
}
/* mibase: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mibase!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mibase {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mibase, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mibase: CSR Field Modifications - via register */

/*******************************************
 * mibound - MRW - Instruction bound register. 
 */
/* mibound: CSR Whole register access */
/* mibound: CSR read.
e.g.
    let _v = csr_read_mibound!();
 */
#[macro_export]
macro_rules! csr_read_mibound {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mibound" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mibound: CSR write 
e.g.
    csr_write_mibound!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mibound {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mibound, {0}" , in(reg) $x);
        }
    );
}
/* mibound: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mibound!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mibound {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mibound, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mibound: CSR Field Modifications - via register */

/*******************************************
 * mdbase - MRW - Data base register. 
 */
/* mdbase: CSR Whole register access */
/* mdbase: CSR read.
e.g.
    let _v = csr_read_mdbase!();
 */
#[macro_export]
macro_rules! csr_read_mdbase {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mdbase" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mdbase: CSR write 
e.g.
    csr_write_mdbase!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mdbase {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mdbase, {0}" , in(reg) $x);
        }
    );
}
/* mdbase: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mdbase!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mdbase {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mdbase, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mdbase: CSR Field Modifications - via register */

/*******************************************
 * mdbound - MRW - Data bound register. 
 */
/* mdbound: CSR Whole register access */
/* mdbound: CSR read.
e.g.
    let _v = csr_read_mdbound!();
 */
#[macro_export]
macro_rules! csr_read_mdbound {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mdbound" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mdbound: CSR write 
e.g.
    csr_write_mdbound!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mdbound {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mdbound, {0}" , in(reg) $x);
        }
    );
}
/* mdbound: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mdbound!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mdbound {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mdbound, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mdbound: CSR Field Modifications - via register */

/*******************************************
 * pmpcfg0 - MRW - Physical memory protection configuration. 
 */
/* pmpcfg0: CSR Whole register access */
/* pmpcfg0: CSR read.
e.g.
    let _v = csr_read_pmpcfg0!();
 */
#[macro_export]
macro_rules! csr_read_pmpcfg0 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, pmpcfg0" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* pmpcfg0: CSR write 
e.g.
    csr_write_pmpcfg0!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_pmpcfg0 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    pmpcfg0, {0}" , in(reg) $x);
        }
    );
}
/* pmpcfg0: CSR Read and Write 
e.g.
    let v_ = csr_read_write_pmpcfg0!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_pmpcfg0 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, pmpcfg0, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* pmpcfg0: CSR Field Modifications - via register */

/*******************************************
 * pmpcfg1 - MRW - Physical memory protection configuration, RV32 only. 
 */
/* pmpcfg1: CSR Whole register access */
/* pmpcfg1: CSR read.
e.g.
    let _v = csr_read_pmpcfg1!();
 */
#[macro_export]
macro_rules! csr_read_pmpcfg1 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, pmpcfg1" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* pmpcfg1: CSR write 
e.g.
    csr_write_pmpcfg1!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_pmpcfg1 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    pmpcfg1, {0}" , in(reg) $x);
        }
    );
}
/* pmpcfg1: CSR Read and Write 
e.g.
    let v_ = csr_read_write_pmpcfg1!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_pmpcfg1 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, pmpcfg1, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* pmpcfg1: CSR Field Modifications - via register */

/*******************************************
 * pmpcfg2 - MRW - Physical memory protection configuration. 
 */
/* pmpcfg2: CSR Whole register access */
/* pmpcfg2: CSR read.
e.g.
    let _v = csr_read_pmpcfg2!();
 */
#[macro_export]
macro_rules! csr_read_pmpcfg2 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, pmpcfg2" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* pmpcfg2: CSR write 
e.g.
    csr_write_pmpcfg2!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_pmpcfg2 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    pmpcfg2, {0}" , in(reg) $x);
        }
    );
}
/* pmpcfg2: CSR Read and Write 
e.g.
    let v_ = csr_read_write_pmpcfg2!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_pmpcfg2 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, pmpcfg2, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* pmpcfg2: CSR Field Modifications - via register */

/*******************************************
 * pmpcfg3 - MRW - Physical memory protection configuration, RV32 only. 
 */
/* pmpcfg3: CSR Whole register access */
/* pmpcfg3: CSR read.
e.g.
    let _v = csr_read_pmpcfg3!();
 */
#[macro_export]
macro_rules! csr_read_pmpcfg3 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, pmpcfg3" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* pmpcfg3: CSR write 
e.g.
    csr_write_pmpcfg3!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_pmpcfg3 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    pmpcfg3, {0}" , in(reg) $x);
        }
    );
}
/* pmpcfg3: CSR Read and Write 
e.g.
    let v_ = csr_read_write_pmpcfg3!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_pmpcfg3 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, pmpcfg3, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* pmpcfg3: CSR Field Modifications - via register */

/*******************************************
 * pmpaddr0 - MRW - Physical memory protection address register. 
 */
/* pmpaddr0: CSR Whole register access */
/* pmpaddr0: CSR read.
e.g.
    let _v = csr_read_pmpaddr0!();
 */
#[macro_export]
macro_rules! csr_read_pmpaddr0 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, pmpaddr0" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* pmpaddr0: CSR write 
e.g.
    csr_write_pmpaddr0!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_pmpaddr0 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    pmpaddr0, {0}" , in(reg) $x);
        }
    );
}
/* pmpaddr0: CSR Read and Write 
e.g.
    let v_ = csr_read_write_pmpaddr0!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_pmpaddr0 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, pmpaddr0, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* pmpaddr0: CSR Field Modifications - via register */

/*******************************************
 * pmpaddr1 - MRW - Physical memory protection address register. 
 */
/* pmpaddr1: CSR Whole register access */
/* pmpaddr1: CSR read.
e.g.
    let _v = csr_read_pmpaddr1!();
 */
#[macro_export]
macro_rules! csr_read_pmpaddr1 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, pmpaddr1" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* pmpaddr1: CSR write 
e.g.
    csr_write_pmpaddr1!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_pmpaddr1 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    pmpaddr1, {0}" , in(reg) $x);
        }
    );
}
/* pmpaddr1: CSR Read and Write 
e.g.
    let v_ = csr_read_write_pmpaddr1!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_pmpaddr1 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, pmpaddr1, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* pmpaddr1: CSR Field Modifications - via register */

/*******************************************
 * pmpaddr15 - MRW - Physical memory protection address register. 
 */
/* pmpaddr15: CSR Whole register access */
/* pmpaddr15: CSR read.
e.g.
    let _v = csr_read_pmpaddr15!();
 */
#[macro_export]
macro_rules! csr_read_pmpaddr15 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, pmpaddr15" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* pmpaddr15: CSR write 
e.g.
    csr_write_pmpaddr15!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_pmpaddr15 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    pmpaddr15, {0}" , in(reg) $x);
        }
    );
}
/* pmpaddr15: CSR Read and Write 
e.g.
    let v_ = csr_read_write_pmpaddr15!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_pmpaddr15 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, pmpaddr15, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* pmpaddr15: CSR Field Modifications - via register */

/*******************************************
 * mhpmcounter4 - MRW - Machine performance-monitoring counter. 
 */
/* mhpmcounter4: CSR Whole register access */
/* mhpmcounter4: CSR read.
e.g.
    let _v = csr_read_mhpmcounter4!();
 */
#[macro_export]
macro_rules! csr_read_mhpmcounter4 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mhpmcounter4" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mhpmcounter4: CSR write 
e.g.
    csr_write_mhpmcounter4!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mhpmcounter4 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mhpmcounter4, {0}" , in(reg) $x);
        }
    );
}
/* mhpmcounter4: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mhpmcounter4!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mhpmcounter4 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mhpmcounter4, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mhpmcounter4: CSR Field Modifications - via register */

/*******************************************
 * mhpmcounter31 - MRW - Machine performance-monitoring counter. 
 */
/* mhpmcounter31: CSR Whole register access */
/* mhpmcounter31: CSR read.
e.g.
    let _v = csr_read_mhpmcounter31!();
 */
#[macro_export]
macro_rules! csr_read_mhpmcounter31 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mhpmcounter31" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mhpmcounter31: CSR write 
e.g.
    csr_write_mhpmcounter31!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mhpmcounter31 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mhpmcounter31, {0}" , in(reg) $x);
        }
    );
}
/* mhpmcounter31: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mhpmcounter31!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mhpmcounter31 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mhpmcounter31, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mhpmcounter31: CSR Field Modifications - via register */

/*******************************************
 * mcycleh - MRW - Upper 32 bits of  mcycle, RV32I only. 
 */
/* mcycleh: CSR Whole register access */
/* mcycleh: CSR read.
e.g.
    let _v = csr_read_mcycleh!();
 */
#[macro_export]
macro_rules! csr_read_mcycleh {
    ( ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mcycleh" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mcycleh: CSR write 
e.g.
    csr_write_mcycleh!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mcycleh {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mcycleh, {0}" , in(reg) $x);
        }
    );
}
/* mcycleh: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mcycleh!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mcycleh {
    ( $x:expr ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mcycleh, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mcycleh: CSR Field Modifications - via register */

/*******************************************
 * minstreth - MRW - Upper 32 bits of  minstret, RV32I only. 
 */
/* minstreth: CSR Whole register access */
/* minstreth: CSR read.
e.g.
    let _v = csr_read_minstreth!();
 */
#[macro_export]
macro_rules! csr_read_minstreth {
    ( ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, minstreth" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* minstreth: CSR write 
e.g.
    csr_write_minstreth!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_minstreth {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    minstreth, {0}" , in(reg) $x);
        }
    );
}
/* minstreth: CSR Read and Write 
e.g.
    let v_ = csr_read_write_minstreth!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_minstreth {
    ( $x:expr ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, minstreth, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* minstreth: CSR Field Modifications - via register */

/*******************************************
 * mhpmcounter3h - MRW - Upper 32 bits of  mhpmcounter3, RV32I only. 
 */
/* mhpmcounter3h: CSR Whole register access */
/* mhpmcounter3h: CSR read.
e.g.
    let _v = csr_read_mhpmcounter3h!();
 */
#[macro_export]
macro_rules! csr_read_mhpmcounter3h {
    ( ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mhpmcounter3h" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mhpmcounter3h: CSR write 
e.g.
    csr_write_mhpmcounter3h!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mhpmcounter3h {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mhpmcounter3h, {0}" , in(reg) $x);
        }
    );
}
/* mhpmcounter3h: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mhpmcounter3h!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mhpmcounter3h {
    ( $x:expr ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mhpmcounter3h, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mhpmcounter3h: CSR Field Modifications - via register */

/*******************************************
 * mhpmcounter4h - MRW - Upper 32 bits of  mhpmcounter4, RV32I only. 
 */
/* mhpmcounter4h: CSR Whole register access */
/* mhpmcounter4h: CSR read.
e.g.
    let _v = csr_read_mhpmcounter4h!();
 */
#[macro_export]
macro_rules! csr_read_mhpmcounter4h {
    ( ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mhpmcounter4h" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mhpmcounter4h: CSR write 
e.g.
    csr_write_mhpmcounter4h!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mhpmcounter4h {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mhpmcounter4h, {0}" , in(reg) $x);
        }
    );
}
/* mhpmcounter4h: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mhpmcounter4h!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mhpmcounter4h {
    ( $x:expr ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mhpmcounter4h, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mhpmcounter4h: CSR Field Modifications - via register */

/*******************************************
 * mhpmcounter31h - MRW - Upper 32 bits of  mhpmcounter31, RV32I only. 
 */
/* mhpmcounter31h: CSR Whole register access */
/* mhpmcounter31h: CSR read.
e.g.
    let _v = csr_read_mhpmcounter31h!();
 */
#[macro_export]
macro_rules! csr_read_mhpmcounter31h {
    ( ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mhpmcounter31h" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mhpmcounter31h: CSR write 
e.g.
    csr_write_mhpmcounter31h!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mhpmcounter31h {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mhpmcounter31h, {0}" , in(reg) $x);
        }
    );
}
/* mhpmcounter31h: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mhpmcounter31h!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mhpmcounter31h {
    ( $x:expr ) => (
        {
            let tmp_value: UintCsr32;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mhpmcounter31h, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mhpmcounter31h: CSR Field Modifications - via register */

/*******************************************
 * mhpmevent4 - MRW - Machine performance-monitoring event selector. 
 */
/* mhpmevent4: CSR Whole register access */
/* mhpmevent4: CSR read.
e.g.
    let _v = csr_read_mhpmevent4!();
 */
#[macro_export]
macro_rules! csr_read_mhpmevent4 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mhpmevent4" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mhpmevent4: CSR write 
e.g.
    csr_write_mhpmevent4!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mhpmevent4 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mhpmevent4, {0}" , in(reg) $x);
        }
    );
}
/* mhpmevent4: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mhpmevent4!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mhpmevent4 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mhpmevent4, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mhpmevent4: CSR Field Modifications - via register */

/*******************************************
 * mhpmevent31 - MRW - Machine performance-monitoring event selector. 
 */
/* mhpmevent31: CSR Whole register access */
/* mhpmevent31: CSR read.
e.g.
    let _v = csr_read_mhpmevent31!();
 */
#[macro_export]
macro_rules! csr_read_mhpmevent31 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mhpmevent31" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mhpmevent31: CSR write 
e.g.
    csr_write_mhpmevent31!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mhpmevent31 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mhpmevent31, {0}" , in(reg) $x);
        }
    );
}
/* mhpmevent31: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mhpmevent31!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mhpmevent31 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mhpmevent31, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mhpmevent31: CSR Field Modifications - via register */

/*******************************************
 * tselect - MRW - Debug/Trace trigger register select. 
 */
/* tselect: CSR Whole register access */
/* tselect: CSR read.
e.g.
    let _v = csr_read_tselect!();
 */
#[macro_export]
macro_rules! csr_read_tselect {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, tselect" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* tselect: CSR write 
e.g.
    csr_write_tselect!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_tselect {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    tselect, {0}" , in(reg) $x);
        }
    );
}
/* tselect: CSR Read and Write 
e.g.
    let v_ = csr_read_write_tselect!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_tselect {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, tselect, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* tselect: CSR Field Modifications - via register */

/*******************************************
 * tdata1 - MRW - First Debug/Trace trigger data register. 
 */
/* tdata1: CSR Whole register access */
/* tdata1: CSR read.
e.g.
    let _v = csr_read_tdata1!();
 */
#[macro_export]
macro_rules! csr_read_tdata1 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, tdata1" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* tdata1: CSR write 
e.g.
    csr_write_tdata1!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_tdata1 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    tdata1, {0}" , in(reg) $x);
        }
    );
}
/* tdata1: CSR Read and Write 
e.g.
    let v_ = csr_read_write_tdata1!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_tdata1 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, tdata1, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* tdata1: CSR Field Modifications - via register */

/*******************************************
 * tdata2 - MRW - Second Debug/Trace trigger data register. 
 */
/* tdata2: CSR Whole register access */
/* tdata2: CSR read.
e.g.
    let _v = csr_read_tdata2!();
 */
#[macro_export]
macro_rules! csr_read_tdata2 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, tdata2" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* tdata2: CSR write 
e.g.
    csr_write_tdata2!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_tdata2 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    tdata2, {0}" , in(reg) $x);
        }
    );
}
/* tdata2: CSR Read and Write 
e.g.
    let v_ = csr_read_write_tdata2!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_tdata2 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, tdata2, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* tdata2: CSR Field Modifications - via register */

/*******************************************
 * tdata3 - MRW - Third Debug/Trace trigger data register. 
 */
/* tdata3: CSR Whole register access */
/* tdata3: CSR read.
e.g.
    let _v = csr_read_tdata3!();
 */
#[macro_export]
macro_rules! csr_read_tdata3 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, tdata3" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* tdata3: CSR write 
e.g.
    csr_write_tdata3!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_tdata3 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    tdata3, {0}" , in(reg) $x);
        }
    );
}
/* tdata3: CSR Read and Write 
e.g.
    let v_ = csr_read_write_tdata3!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_tdata3 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, tdata3, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* tdata3: CSR Field Modifications - via register */

/*******************************************
 * dcsr - DRW - Debug control and status register. 
 */
/* dcsr: CSR Whole register access */
/* dcsr: CSR read.
e.g.
    let _v = csr_read_dcsr!();
 */
#[macro_export]
macro_rules! csr_read_dcsr {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, dcsr" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* dcsr: CSR write 
e.g.
    csr_write_dcsr!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_dcsr {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    dcsr, {0}" , in(reg) $x);
        }
    );
}
/* dcsr: CSR Read and Write 
e.g.
    let v_ = csr_read_write_dcsr!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_dcsr {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, dcsr, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* dcsr: CSR Field Modifications - via register */

/*******************************************
 * dpc - DRW - Debug PC. 
 */
/* dpc: CSR Whole register access */
/* dpc: CSR read.
e.g.
    let _v = csr_read_dpc!();
 */
#[macro_export]
macro_rules! csr_read_dpc {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, dpc" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* dpc: CSR write 
e.g.
    csr_write_dpc!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_dpc {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    dpc, {0}" , in(reg) $x);
        }
    );
}
/* dpc: CSR Read and Write 
e.g.
    let v_ = csr_read_write_dpc!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_dpc {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, dpc, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* dpc: CSR Field Modifications - via register */

/*******************************************
 * dscratch0 - DRW - Debug scratch register 0. 
 */
/* dscratch0: CSR Whole register access */
/* dscratch0: CSR read.
e.g.
    let _v = csr_read_dscratch0!();
 */
#[macro_export]
macro_rules! csr_read_dscratch0 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, dscratch0" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* dscratch0: CSR write 
e.g.
    csr_write_dscratch0!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_dscratch0 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    dscratch0, {0}" , in(reg) $x);
        }
    );
}
/* dscratch0: CSR Read and Write 
e.g.
    let v_ = csr_read_write_dscratch0!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_dscratch0 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, dscratch0, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* dscratch0: CSR Field Modifications - via register */

/*******************************************
 * dscratch1 - DRW - Debug scratch register 1. 
 */
/* dscratch1: CSR Whole register access */
/* dscratch1: CSR read.
e.g.
    let _v = csr_read_dscratch1!();
 */
#[macro_export]
macro_rules! csr_read_dscratch1 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, dscratch1" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* dscratch1: CSR write 
e.g.
    csr_write_dscratch1!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_dscratch1 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    dscratch1, {0}" , in(reg) $x);
        }
    );
}
/* dscratch1: CSR Read and Write 
e.g.
    let v_ = csr_read_write_dscratch1!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_dscratch1 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, dscratch1, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* dscratch1: CSR Field Modifications - via register */

/*******************************************
 * hie - HRW - Hypervisor interrupt-enable register. 
 */
/* hie: CSR Whole register access */
/* hie: CSR read.
e.g.
    let _v = csr_read_hie!();
 */
#[macro_export]
macro_rules! csr_read_hie {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, hie" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* hie: CSR write 
e.g.
    csr_write_hie!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_hie {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    hie, {0}" , in(reg) $x);
        }
    );
}
/* hie: CSR Read and Write 
e.g.
    let v_ = csr_read_write_hie!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_hie {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, hie, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* hie: CSR Field Modifications - via register */

/*******************************************
 * hgeie - HRW - Hypervisor guest external interrupt-enable register. 
 */
/* hgeie: CSR Whole register access */
/* hgeie: CSR read.
e.g.
    let _v = csr_read_hgeie!();
 */
#[macro_export]
macro_rules! csr_read_hgeie {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, hgeie" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* hgeie: CSR write 
e.g.
    csr_write_hgeie!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_hgeie {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    hgeie, {0}" , in(reg) $x);
        }
    );
}
/* hgeie: CSR Read and Write 
e.g.
    let v_ = csr_read_write_hgeie!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_hgeie {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, hgeie, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* hgeie: CSR Field Modifications - via register */

/*******************************************
 * htval - HRW - Hypervisor bad guest physical address. 
 */
/* htval: CSR Whole register access */
/* htval: CSR read.
e.g.
    let _v = csr_read_htval!();
 */
#[macro_export]
macro_rules! csr_read_htval {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, htval" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* htval: CSR write 
e.g.
    csr_write_htval!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_htval {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    htval, {0}" , in(reg) $x);
        }
    );
}
/* htval: CSR Read and Write 
e.g.
    let v_ = csr_read_write_htval!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_htval {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, htval, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* htval: CSR Field Modifications - via register */

/*******************************************
 * hip - HRW - Hypervisor interrupt pending. 
 */
/* hip: CSR Whole register access */
/* hip: CSR read.
e.g.
    let _v = csr_read_hip!();
 */
#[macro_export]
macro_rules! csr_read_hip {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, hip" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* hip: CSR write 
e.g.
    csr_write_hip!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_hip {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    hip, {0}" , in(reg) $x);
        }
    );
}
/* hip: CSR Read and Write 
e.g.
    let v_ = csr_read_write_hip!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_hip {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, hip, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* hip: CSR Field Modifications - via register */

/*******************************************
 * htinst - HRW - Hypervisor trap instruction (transformed). 
 */
/* htinst: CSR Whole register access */
/* htinst: CSR read.
e.g.
    let _v = csr_read_htinst!();
 */
#[macro_export]
macro_rules! csr_read_htinst {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, htinst" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* htinst: CSR write 
e.g.
    csr_write_htinst!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_htinst {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    htinst, {0}" , in(reg) $x);
        }
    );
}
/* htinst: CSR Read and Write 
e.g.
    let v_ = csr_read_write_htinst!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_htinst {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, htinst, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* htinst: CSR Field Modifications - via register */

/*******************************************
 * hgeip - HRO - Hypervisor guest external interrupt pending. 
 */
/* hgeip: CSR Whole register access */
/* hgeip: CSR read.
e.g.
    let _v = csr_read_hgeip!();
 */
#[macro_export]
macro_rules! csr_read_hgeip {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, hgeip" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}

/* hgeip: CSR Field Modifications - via register */

/*******************************************
 * mtinst - MRW - Machine trap instruction (transformed). 
 */
/* mtinst: CSR Whole register access */
/* mtinst: CSR read.
e.g.
    let _v = csr_read_mtinst!();
 */
#[macro_export]
macro_rules! csr_read_mtinst {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mtinst" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mtinst: CSR write 
e.g.
    csr_write_mtinst!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mtinst {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mtinst, {0}" , in(reg) $x);
        }
    );
}
/* mtinst: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mtinst!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mtinst {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mtinst, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mtinst: CSR Field Modifications - via register */

/*******************************************
 * mtval2 - MRW - Machine bad guest physical address. 
 */
/* mtval2: CSR Whole register access */
/* mtval2: CSR read.
e.g.
    let _v = csr_read_mtval2!();
 */
#[macro_export]
macro_rules! csr_read_mtval2 {
    ( ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, mtval2" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
/* mtval2: CSR write 
e.g.
    csr_write_mtval2!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_mtval2 {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    mtval2, {0}" , in(reg) $x);
        }
    );
}
/* mtval2: CSR Read and Write 
e.g.
    let v_ = csr_read_write_mtval2!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_mtval2 {
    ( $x:expr ) => (
        {
            let tmp_value: UintXlen;
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, mtval2, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}

/* mtval2: CSR Field Modifications - via register */

/*******************************************
 * mstatus - MRW - Machine Status 
 */
pub const MSTATUS_MIE_BIT_OFFSET:isize   = 3;
pub const MSTATUS_MIE_BIT_WIDTH:isize    = 1;
pub const MSTATUS_MIE_BIT_MASK:UintXlen = 0x8;
pub const MSTATUS_MIE_ALL_SET_MASK:UintXlen = 0x1;
pub const MSTATUS_SIE_BIT_OFFSET:isize   = 2;
pub const MSTATUS_SIE_BIT_WIDTH:isize    = 1;
pub const MSTATUS_SIE_BIT_MASK:UintXlen = 0x4;
pub const MSTATUS_SIE_ALL_SET_MASK:UintXlen = 0x1;
pub const MSTATUS_MPIE_BIT_OFFSET:isize   = 7;
pub const MSTATUS_MPIE_BIT_WIDTH:isize    = 1;
pub const MSTATUS_MPIE_BIT_MASK:UintXlen = 0x80;
pub const MSTATUS_MPIE_ALL_SET_MASK:UintXlen = 0x1;
pub const MSTATUS_SPIE_BIT_OFFSET:isize   = 5;
pub const MSTATUS_SPIE_BIT_WIDTH:isize    = 1;
pub const MSTATUS_SPIE_BIT_MASK:UintXlen = 0x20;
pub const MSTATUS_SPIE_ALL_SET_MASK:UintXlen = 0x1;
pub const MSTATUS_MPRV_BIT_OFFSET:isize   = 17;
pub const MSTATUS_MPRV_BIT_WIDTH:isize    = 1;
pub const MSTATUS_MPRV_BIT_MASK:UintXlen = 0x20000;
pub const MSTATUS_MPRV_ALL_SET_MASK:UintXlen = 0x1;
pub const MSTATUS_MPP_BIT_OFFSET:isize   = 11;
pub const MSTATUS_MPP_BIT_WIDTH:isize    = 2;
pub const MSTATUS_MPP_BIT_MASK:UintXlen = 0x1800;
pub const MSTATUS_MPP_ALL_SET_MASK:UintXlen = 0x3;
pub const MSTATUS_SPP_BIT_OFFSET:isize   = 8;
pub const MSTATUS_SPP_BIT_WIDTH:isize    = 1;
pub const MSTATUS_SPP_BIT_MASK:UintXlen = 0x100;
pub const MSTATUS_SPP_ALL_SET_MASK:UintXlen = 0x1;

/*******************************************
 * mtvec - MRW - Machine Trap Vector Base Address 
 */
pub const MTVEC_BASE_BIT_OFFSET:isize   = 2;
pub const MTVEC_BASE_BIT_WIDTH:isize    = (__riscv_xlen-1)-(2) + 1;
pub const MTVEC_BASE_BIT_MASK:UintXlen = (1<<((__riscv_xlen-1)-(2) + 1-1)) << (2);
pub const MTVEC_BASE_ALL_SET_MASK:UintXlen = (1<<((__riscv_xlen-1)-(2) + 1-1)) << (0);
pub const MTVEC_MODE_BIT_OFFSET:isize   = 0;
pub const MTVEC_MODE_BIT_WIDTH:isize    = 2;
pub const MTVEC_MODE_BIT_MASK:UintXlen = 0x3;
pub const MTVEC_MODE_ALL_SET_MASK:UintXlen = 0x3;

/*******************************************
 * mip - MRW - Machine Interrupt Pending 
 */
pub const MIP_MSI_BIT_OFFSET:isize   = 3;
pub const MIP_MSI_BIT_WIDTH:isize    = 1;
pub const MIP_MSI_BIT_MASK:UintXlen = 0x8;
pub const MIP_MSI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIP_MTI_BIT_OFFSET:isize   = 7;
pub const MIP_MTI_BIT_WIDTH:isize    = 1;
pub const MIP_MTI_BIT_MASK:UintXlen = 0x80;
pub const MIP_MTI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIP_MEI_BIT_OFFSET:isize   = 11;
pub const MIP_MEI_BIT_WIDTH:isize    = 1;
pub const MIP_MEI_BIT_MASK:UintXlen = 0x800;
pub const MIP_MEI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIP_SSI_BIT_OFFSET:isize   = 1;
pub const MIP_SSI_BIT_WIDTH:isize    = 1;
pub const MIP_SSI_BIT_MASK:UintXlen = 0x2;
pub const MIP_SSI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIP_STI_BIT_OFFSET:isize   = 5;
pub const MIP_STI_BIT_WIDTH:isize    = 1;
pub const MIP_STI_BIT_MASK:UintXlen = 0x20;
pub const MIP_STI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIP_SEI_BIT_OFFSET:isize   = 9;
pub const MIP_SEI_BIT_WIDTH:isize    = 1;
pub const MIP_SEI_BIT_MASK:UintXlen = 0x200;
pub const MIP_SEI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIP_USI_BIT_OFFSET:isize   = 0;
pub const MIP_USI_BIT_WIDTH:isize    = 1;
pub const MIP_USI_BIT_MASK:UintXlen = 0x1;
pub const MIP_USI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIP_UTI_BIT_OFFSET:isize   = 4;
pub const MIP_UTI_BIT_WIDTH:isize    = 1;
pub const MIP_UTI_BIT_MASK:UintXlen = 0x10;
pub const MIP_UTI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIP_UEI_BIT_OFFSET:isize   = 8;
pub const MIP_UEI_BIT_WIDTH:isize    = 1;
pub const MIP_UEI_BIT_MASK:UintXlen = 0x100;
pub const MIP_UEI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIP_PLATFORM_DEFINED_BIT_OFFSET:isize   = 16;
pub const MIP_PLATFORM_DEFINED_BIT_WIDTH:isize    = (__riscv_xlen)-(16) + 1;
pub const MIP_PLATFORM_DEFINED_BIT_MASK:UintXlen = (1<<((__riscv_xlen)-(16) + 1-1)) << (16);
pub const MIP_PLATFORM_DEFINED_ALL_SET_MASK:UintXlen = (1<<((__riscv_xlen)-(16) + 1-1)) << (0);

/*******************************************
 * mie - MRW - Machine Interrupt Enable 
 */
pub const MIE_MSI_BIT_OFFSET:isize   = 3;
pub const MIE_MSI_BIT_WIDTH:isize    = 1;
pub const MIE_MSI_BIT_MASK:UintXlen = 0x8;
pub const MIE_MSI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIE_MTI_BIT_OFFSET:isize   = 7;
pub const MIE_MTI_BIT_WIDTH:isize    = 1;
pub const MIE_MTI_BIT_MASK:UintXlen = 0x80;
pub const MIE_MTI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIE_MEI_BIT_OFFSET:isize   = 11;
pub const MIE_MEI_BIT_WIDTH:isize    = 1;
pub const MIE_MEI_BIT_MASK:UintXlen = 0x800;
pub const MIE_MEI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIE_SSI_BIT_OFFSET:isize   = 1;
pub const MIE_SSI_BIT_WIDTH:isize    = 1;
pub const MIE_SSI_BIT_MASK:UintXlen = 0x2;
pub const MIE_SSI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIE_STI_BIT_OFFSET:isize   = 5;
pub const MIE_STI_BIT_WIDTH:isize    = 1;
pub const MIE_STI_BIT_MASK:UintXlen = 0x20;
pub const MIE_STI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIE_SEI_BIT_OFFSET:isize   = 9;
pub const MIE_SEI_BIT_WIDTH:isize    = 1;
pub const MIE_SEI_BIT_MASK:UintXlen = 0x200;
pub const MIE_SEI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIE_USI_BIT_OFFSET:isize   = 0;
pub const MIE_USI_BIT_WIDTH:isize    = 1;
pub const MIE_USI_BIT_MASK:UintXlen = 0x1;
pub const MIE_USI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIE_UTI_BIT_OFFSET:isize   = 4;
pub const MIE_UTI_BIT_WIDTH:isize    = 1;
pub const MIE_UTI_BIT_MASK:UintXlen = 0x10;
pub const MIE_UTI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIE_UEI_BIT_OFFSET:isize   = 8;
pub const MIE_UEI_BIT_WIDTH:isize    = 1;
pub const MIE_UEI_BIT_MASK:UintXlen = 0x100;
pub const MIE_UEI_ALL_SET_MASK:UintXlen = 0x1;
pub const MIE_PLATFORM_DEFINED_BIT_OFFSET:isize   = 16;
pub const MIE_PLATFORM_DEFINED_BIT_WIDTH:isize    = (__riscv_xlen)-(16) + 1;
pub const MIE_PLATFORM_DEFINED_BIT_MASK:UintXlen = (1<<((__riscv_xlen)-(16) + 1-1)) << (16);
pub const MIE_PLATFORM_DEFINED_ALL_SET_MASK:UintXlen = (1<<((__riscv_xlen)-(16) + 1-1)) << (0);

/*******************************************
 * mcountinhibit - MRW - Machine Counter Inhibit 
 */
pub const MCOUNTINHIBIT_CY_BIT_OFFSET:isize   = 0;
pub const MCOUNTINHIBIT_CY_BIT_WIDTH:isize    = 1;
pub const MCOUNTINHIBIT_CY_BIT_MASK:UintXlen = 0x1;
pub const MCOUNTINHIBIT_CY_ALL_SET_MASK:UintXlen = 0x1;
pub const MCOUNTINHIBIT_IR_BIT_OFFSET:isize   = 2;
pub const MCOUNTINHIBIT_IR_BIT_WIDTH:isize    = 1;
pub const MCOUNTINHIBIT_IR_BIT_MASK:UintXlen = 0x4;
pub const MCOUNTINHIBIT_IR_ALL_SET_MASK:UintXlen = 0x1;
pub const MCOUNTINHIBIT_HPM_BIT_OFFSET:isize   = 3;
pub const MCOUNTINHIBIT_HPM_BIT_WIDTH:isize    = 29;
pub const MCOUNTINHIBIT_HPM_BIT_MASK:UintXlen = 0xfffffff8;
pub const MCOUNTINHIBIT_HPM_ALL_SET_MASK:UintXlen = 0x1fffffff;

/*******************************************
 * mcounteren - MRW - Counter Enable 
 */
pub const MCOUNTEREN_CY_BIT_OFFSET:isize   = 0;
pub const MCOUNTEREN_CY_BIT_WIDTH:isize    = 1;
pub const MCOUNTEREN_CY_BIT_MASK:UintXlen = 0x1;
pub const MCOUNTEREN_CY_ALL_SET_MASK:UintXlen = 0x1;
pub const MCOUNTEREN_TM_BIT_OFFSET:isize   = 1;
pub const MCOUNTEREN_TM_BIT_WIDTH:isize    = 1;
pub const MCOUNTEREN_TM_BIT_MASK:UintXlen = 0x2;
pub const MCOUNTEREN_TM_ALL_SET_MASK:UintXlen = 0x1;
pub const MCOUNTEREN_IR_BIT_OFFSET:isize   = 2;
pub const MCOUNTEREN_IR_BIT_WIDTH:isize    = 1;
pub const MCOUNTEREN_IR_BIT_MASK:UintXlen = 0x4;
pub const MCOUNTEREN_IR_ALL_SET_MASK:UintXlen = 0x1;
pub const MCOUNTEREN_HPM_BIT_OFFSET:isize   = 3;
pub const MCOUNTEREN_HPM_BIT_WIDTH:isize    = 29;
pub const MCOUNTEREN_HPM_BIT_MASK:UintXlen = 0xfffffff8;
pub const MCOUNTEREN_HPM_ALL_SET_MASK:UintXlen = 0x1fffffff;

/*******************************************
 * mcause - MRW - Machine Exception Cause 
 */
pub const MCAUSE_INTERRUPT_BIT_OFFSET:isize   = __riscv_xlen-1;
pub const MCAUSE_INTERRUPT_BIT_WIDTH:isize    = 1;
pub const MCAUSE_INTERRUPT_BIT_MASK:UintXlen = 0x1 << (__riscv_xlen-1);
pub const MCAUSE_INTERRUPT_ALL_SET_MASK:UintXlen = 0x1;
pub const MCAUSE_EXCEPTION_CODE_BIT_OFFSET:isize   = 0;
pub const MCAUSE_EXCEPTION_CODE_BIT_WIDTH:isize    = (__riscv_xlen-2)-(0) + 1;
pub const MCAUSE_EXCEPTION_CODE_BIT_MASK:UintXlen = (1<<((__riscv_xlen-2)-(0) + 1-1)) << (0);
pub const MCAUSE_EXCEPTION_CODE_ALL_SET_MASK:UintXlen = (1<<((__riscv_xlen-2)-(0) + 1-1)) << (0);

/*******************************************
 * scause - SRW - Supervisor Exception Cause 
 */
pub const SCAUSE_INTERRUPT_BIT_OFFSET:isize   = __riscv_xlen-1;
pub const SCAUSE_INTERRUPT_BIT_WIDTH:isize    = 1;
pub const SCAUSE_INTERRUPT_BIT_MASK:UintXlen = 0x1 << (__riscv_xlen-1);
pub const SCAUSE_INTERRUPT_ALL_SET_MASK:UintXlen = 0x1;
pub const SCAUSE_EXCEPTION_CODE_BIT_OFFSET:isize   = 0;
pub const SCAUSE_EXCEPTION_CODE_BIT_WIDTH:isize    = (__riscv_xlen-2)-(0) + 1;
pub const SCAUSE_EXCEPTION_CODE_BIT_MASK:UintXlen = (1<<((__riscv_xlen-2)-(0) + 1-1)) << (0);
pub const SCAUSE_EXCEPTION_CODE_ALL_SET_MASK:UintXlen = (1<<((__riscv_xlen-2)-(0) + 1-1)) << (0);

/*******************************************
 * sstatus - SRW - Supervisor Status 
 */
pub const SSTATUS_SIE_BIT_OFFSET:isize   = 2;
pub const SSTATUS_SIE_BIT_WIDTH:isize    = 1;
pub const SSTATUS_SIE_BIT_MASK:UintXlen = 0x4;
pub const SSTATUS_SIE_ALL_SET_MASK:UintXlen = 0x1;
pub const SSTATUS_SPIE_BIT_OFFSET:isize   = 5;
pub const SSTATUS_SPIE_BIT_WIDTH:isize    = 1;
pub const SSTATUS_SPIE_BIT_MASK:UintXlen = 0x20;
pub const SSTATUS_SPIE_ALL_SET_MASK:UintXlen = 0x1;
pub const SSTATUS_SPP_BIT_OFFSET:isize   = 8;
pub const SSTATUS_SPP_BIT_WIDTH:isize    = 1;
pub const SSTATUS_SPP_BIT_MASK:UintXlen = 0x100;
pub const SSTATUS_SPP_ALL_SET_MASK:UintXlen = 0x1;

/*******************************************
 * stvec - SRW - Supervisor Trap Vector Base Address 
 */
pub const STVEC_BASE_BIT_OFFSET:isize   = 2;
pub const STVEC_BASE_BIT_WIDTH:isize    = (__riscv_xlen-1)-(2) + 1;
pub const STVEC_BASE_BIT_MASK:UintXlen = (1<<((__riscv_xlen-1)-(2) + 1-1)) << (2);
pub const STVEC_BASE_ALL_SET_MASK:UintXlen = (1<<((__riscv_xlen-1)-(2) + 1-1)) << (0);
pub const STVEC_MODE_BIT_OFFSET:isize   = 0;
pub const STVEC_MODE_BIT_WIDTH:isize    = 2;
pub const STVEC_MODE_BIT_MASK:UintXlen = 0x3;
pub const STVEC_MODE_ALL_SET_MASK:UintXlen = 0x3;

/*******************************************
 * sip - SRW - Supervisor Interrupt Pending 
 */
pub const SIP_SSI_BIT_OFFSET:isize   = 1;
pub const SIP_SSI_BIT_WIDTH:isize    = 1;
pub const SIP_SSI_BIT_MASK:UintXlen = 0x2;
pub const SIP_SSI_ALL_SET_MASK:UintXlen = 0x1;
pub const SIP_STI_BIT_OFFSET:isize   = 5;
pub const SIP_STI_BIT_WIDTH:isize    = 1;
pub const SIP_STI_BIT_MASK:UintXlen = 0x20;
pub const SIP_STI_ALL_SET_MASK:UintXlen = 0x1;
pub const SIP_SEI_BIT_OFFSET:isize   = 9;
pub const SIP_SEI_BIT_WIDTH:isize    = 1;
pub const SIP_SEI_BIT_MASK:UintXlen = 0x200;
pub const SIP_SEI_ALL_SET_MASK:UintXlen = 0x1;
pub const SIP_USI_BIT_OFFSET:isize   = 0;
pub const SIP_USI_BIT_WIDTH:isize    = 1;
pub const SIP_USI_BIT_MASK:UintXlen = 0x1;
pub const SIP_USI_ALL_SET_MASK:UintXlen = 0x1;
pub const SIP_UTI_BIT_OFFSET:isize   = 4;
pub const SIP_UTI_BIT_WIDTH:isize    = 1;
pub const SIP_UTI_BIT_MASK:UintXlen = 0x10;
pub const SIP_UTI_ALL_SET_MASK:UintXlen = 0x1;
pub const SIP_UEI_BIT_OFFSET:isize   = 8;
pub const SIP_UEI_BIT_WIDTH:isize    = 1;
pub const SIP_UEI_BIT_MASK:UintXlen = 0x100;
pub const SIP_UEI_ALL_SET_MASK:UintXlen = 0x1;

/*******************************************
 * sie - SRW - Supervisor Interrupt Enable 
 */
pub const SIE_SSI_BIT_OFFSET:isize   = 1;
pub const SIE_SSI_BIT_WIDTH:isize    = 1;
pub const SIE_SSI_BIT_MASK:UintXlen = 0x2;
pub const SIE_SSI_ALL_SET_MASK:UintXlen = 0x1;
pub const SIE_STI_BIT_OFFSET:isize   = 5;
pub const SIE_STI_BIT_WIDTH:isize    = 1;
pub const SIE_STI_BIT_MASK:UintXlen = 0x20;
pub const SIE_STI_ALL_SET_MASK:UintXlen = 0x1;
pub const SIE_SEI_BIT_OFFSET:isize   = 9;
pub const SIE_SEI_BIT_WIDTH:isize    = 1;
pub const SIE_SEI_BIT_MASK:UintXlen = 0x200;
pub const SIE_SEI_ALL_SET_MASK:UintXlen = 0x1;
pub const SIE_USI_BIT_OFFSET:isize   = 0;
pub const SIE_USI_BIT_WIDTH:isize    = 1;
pub const SIE_USI_BIT_MASK:UintXlen = 0x1;
pub const SIE_USI_ALL_SET_MASK:UintXlen = 0x1;
pub const SIE_UTI_BIT_OFFSET:isize   = 4;
pub const SIE_UTI_BIT_WIDTH:isize    = 1;
pub const SIE_UTI_BIT_MASK:UintXlen = 0x10;
pub const SIE_UTI_ALL_SET_MASK:UintXlen = 0x1;
pub const SIE_UEI_BIT_OFFSET:isize   = 8;
pub const SIE_UEI_BIT_WIDTH:isize    = 1;
pub const SIE_UEI_BIT_MASK:UintXlen = 0x100;
pub const SIE_UEI_ALL_SET_MASK:UintXlen = 0x1;

/*******************************************
 * ustatus - URW - User mode restricted view of mstatus 
 */
pub const USTATUS_UIE_BIT_OFFSET:isize   = 1;
pub const USTATUS_UIE_BIT_WIDTH:isize    = 1;
pub const USTATUS_UIE_BIT_MASK:UintXlen = 0x2;
pub const USTATUS_UIE_ALL_SET_MASK:UintXlen = 0x1;
pub const USTATUS_UPIE_BIT_OFFSET:isize   = 3;
pub const USTATUS_UPIE_BIT_WIDTH:isize    = 1;
pub const USTATUS_UPIE_BIT_MASK:UintXlen = 0x8;
pub const USTATUS_UPIE_ALL_SET_MASK:UintXlen = 0x1;

/*******************************************
 * uip - URW - User Interrupt Pending 
 */
pub const UIP_USI_BIT_OFFSET:isize   = 0;
pub const UIP_USI_BIT_WIDTH:isize    = 1;
pub const UIP_USI_BIT_MASK:UintXlen = 0x1;
pub const UIP_USI_ALL_SET_MASK:UintXlen = 0x1;
pub const UIP_UTI_BIT_OFFSET:isize   = 4;
pub const UIP_UTI_BIT_WIDTH:isize    = 1;
pub const UIP_UTI_BIT_MASK:UintXlen = 0x10;
pub const UIP_UTI_ALL_SET_MASK:UintXlen = 0x1;
pub const UIP_UEI_BIT_OFFSET:isize   = 8;
pub const UIP_UEI_BIT_WIDTH:isize    = 1;
pub const UIP_UEI_BIT_MASK:UintXlen = 0x100;
pub const UIP_UEI_ALL_SET_MASK:UintXlen = 0x1;

/*******************************************
 * uie - URW - User Interrupt Enable 
 */
pub const UIE_USI_BIT_OFFSET:isize   = 0;
pub const UIE_USI_BIT_WIDTH:isize    = 1;
pub const UIE_USI_BIT_MASK:UintXlen = 0x1;
pub const UIE_USI_ALL_SET_MASK:UintXlen = 0x1;
pub const UIE_UTI_BIT_OFFSET:isize   = 4;
pub const UIE_UTI_BIT_WIDTH:isize    = 1;
pub const UIE_UTI_BIT_MASK:UintXlen = 0x10;
pub const UIE_UTI_ALL_SET_MASK:UintXlen = 0x1;
pub const UIE_UEI_BIT_OFFSET:isize   = 8;
pub const UIE_UEI_BIT_WIDTH:isize    = 1;
pub const UIE_UEI_BIT_MASK:UintXlen = 0x100;
pub const UIE_UEI_ALL_SET_MASK:UintXlen = 0x1;

/*******************************************
 * ucause - URW - User Exception Cause 
 */
pub const UCAUSE_INTERRUPT_BIT_OFFSET:isize   = __riscv_xlen-1;
pub const UCAUSE_INTERRUPT_BIT_WIDTH:isize    = 1;
pub const UCAUSE_INTERRUPT_BIT_MASK:UintXlen = 0x1 << (__riscv_xlen-1);
pub const UCAUSE_INTERRUPT_ALL_SET_MASK:UintXlen = 0x1;
pub const UCAUSE_EXCEPTION_CODE_BIT_OFFSET:isize   = 0;
pub const UCAUSE_EXCEPTION_CODE_BIT_WIDTH:isize    = (__riscv_xlen-2)-(0) + 1;
pub const UCAUSE_EXCEPTION_CODE_BIT_MASK:UintXlen = (1<<((__riscv_xlen-2)-(0) + 1-1)) << (0);
pub const UCAUSE_EXCEPTION_CODE_ALL_SET_MASK:UintXlen = (1<<((__riscv_xlen-2)-(0) + 1-1)) << (0);

/*******************************************
 * utvec - URW - User Trap Vector Base Address 
 */
pub const UTVEC_BASE_BIT_OFFSET:isize   = 2;
pub const UTVEC_BASE_BIT_WIDTH:isize    = (__riscv_xlen-1)-(2) + 1;
pub const UTVEC_BASE_BIT_MASK:UintXlen = (1<<((__riscv_xlen-1)-(2) + 1-1)) << (2);
pub const UTVEC_BASE_ALL_SET_MASK:UintXlen = (1<<((__riscv_xlen-1)-(2) + 1-1)) << (0);
pub const UTVEC_MODE_BIT_OFFSET:isize   = 0;
pub const UTVEC_MODE_BIT_WIDTH:isize    = 2;
pub const UTVEC_MODE_BIT_MASK:UintXlen = 0x3;
pub const UTVEC_MODE_ALL_SET_MASK:UintXlen = 0x3;
