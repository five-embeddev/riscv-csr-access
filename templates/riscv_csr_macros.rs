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


{%- for reg_name,reg_data in data.regs.items() %}
    {%- if not reg_data.mmio %}

/*******************************************
 * {{reg_name}} - {{reg_data.priv}} - {{reg_data.desc}} 
 */
{%- set ctype_reg = reg_data|csr_ctype_rs %}
{%- set ctype_arg = reg_data|arg_ctype %}
/* {{reg_name}}: CSR Whole register access */
            {%- if "R" in reg_data.priv %}
/* {{reg_name}}: CSR read.
e.g.
    let _v = csr_read_{{reg_name}}!();
 */
#[macro_export]
macro_rules! csr_read_{{reg_name}} {
    ( ) => (
        {
            let tmp_value: {{ctype_reg}};
            unsafe {
                use core::arch::asm;
                asm!("csrr    {0}, {{reg_name}}" , out(reg) tmp_value);
            }
            tmp_value
        }
    );
}
            {%- endif %}
            {%- if "W" in reg_data.priv %}
/* {{reg_name}}: CSR write 
e.g.
    csr_write_{{reg_name}}!(0x1234567);
*/
#[macro_export]
macro_rules! csr_write_{{reg_name}} {
    ( $x:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrw    {{reg_name}}, {0}" , in(reg) $x);
        }
    );
}
/* {{reg_name}}: CSR Read and Write 
e.g.
    let v_ = csr_read_write_{{reg_name}}!(0x1234567);
*/
#[macro_export]
macro_rules! csr_read_write_{{reg_name}} {
    ( $x:expr ) => (
        {
            let tmp_value: {{ctype_reg}};
            unsafe {
                use core::arch::asm;
                asm!("csrrw    {0}, {{reg_name}}, {1}" , out(reg) tmp_value, in(reg) $x);
            }
            tmp_value
        }
    );
}
            {%- endif%}

/* {{reg_name}}: CSR Field Modifications - via register */

            {%- if reg_data.fields %}
                {%- if "W" in reg_data.priv %}
/* Register CSR bit set instructions.
e.g.
csr_set_bits_{{reg_name}}!(0x0F0F0F);
*/
#[macro_export]
macro_rules! csr_set_bits_{{reg_name}} {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrs    zero, {{reg_name}}, {{'{'}}{{0}}{{'}'}}", in(reg) $mask);
        }
    );
}
/* Register CSR bit clear instructions
e.g.
csr_clr_bits_{{reg_name}}!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_clr_bits_{{reg_name}} {
    ( $mask:expr ) => (
        unsafe {
            use core::arch::asm;
            asm!("csrrc    zero, {{reg_name}}, {{'{'}}{{0}}{{'}'}}", in(reg) $mask);
        }
    );
}
/* Register CSR read and then bit set instructions
e.g.
let org_value_ = csr_read_set_bits_{{reg_name}}!(0x0F0F0F)
*/
#[macro_export]
macro_rules! csr_read_set_bits_{{reg_name}} {
    ( $mask:expr ) => (
        {
            let tmp_value: {{ctype_reg}};
            unsafe {
                use core::arch::asm;
                asm!("csrrs    {{'{'}}{{0}}{{'}'}}, {{reg_name}}, {{'{'}}{{1}}{{'}'}}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* Register CSR read and then bit clear instructions
e.g.
let org_value_ = csr_read_clr_bits_{{reg_name}}!(0x0F0F0F)
 */
#[macro_export]
macro_rules! csr_read_clr_bits_{{reg_name}} {
    ( $mask:expr ) => (
        {
            let tmp_value: {{ctype_reg}};
            unsafe {
                use core::arch::asm;
                asm!("csrrc    {{'{'}}{{0}}{{'}'}}, {{reg_name}}, {{'{'}}{{1}}{{'}'}}", out(reg) tmp_value, in(reg) $mask);
            }
            tmp_value
        }
    );
}
/* {{reg_name}}: CSR Field Modifications - via immediate */
/* {{reg_name}}, CSR write value via immediate value (only up to 5 bits).
e.g.
csr_write_imm_{{reg_name}}!(0x1F);
 */
#[macro_export]
macro_rules! csr_write_imm_{{reg_name}} {
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrwi    zero, {{reg_name}}, ", stringify!($value)));
        }
    );
}

/* {{reg_name}}, CSR set bits via immediate value mask (only up to 5 bits).
e.g.
csr_set_bits_imm_{{reg_name}}!(0x1F);
 */
#[macro_export]
macro_rules! csr_set_bits_imm_{{reg_name}} {
    {%- if reg_data.fields %}
        {%-for field_name,field_data in reg_data.fields.items() %}
            {%- if field_data|csr_field_imm_valid %}
                {%- set bit_width = field_data|csr_bit_width_rs %}
                {%- set bit_offset = field_data|csr_bit_offset_rs %}
    ( {{reg_name|upper}}_{{field_name|upper}}_BIT_MASK) => { csr_set_bits_imm_{{reg_name}}!({{bit_offset|csr_format_mask_rs(bit_width)}})};
            {%- endif %}
        {%- endfor %}
    {%- endif %}
    ( $value:literal ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrsi    zero, {{reg_name}}, ", stringify!($value)));
        }
    );
}
/* {{reg_name}}, CSR clear bits via immediate value mask (only up to 5 bits).
e.g.
csr_clr_bits_imm_{{reg_name}}!(0x1F);
 */
#[macro_export]
macro_rules! csr_clr_bits_imm_{{reg_name}} {
    {%- if reg_data.fields %}
        {%-for field_name,field_data in reg_data.fields.items() %}
            {%- if field_data|csr_field_imm_valid %}
                {%- set bit_width = field_data|csr_bit_width_rs %}
                {%- set bit_offset = field_data|csr_bit_offset_rs %}
    ( {{reg_name|upper}}_{{field_name|upper}}_BIT_MASK) => { csr_clr_bits_imm_{{reg_name}}!({{bit_offset|csr_format_mask_rs(bit_width)}})};
            {%- endif %}
        {%- endfor %}
    {%- endif %}
    ( $value:ident ) => (
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrrci    zero, {{reg_name}}, ", stringify!($value)));
        }
    );
}
                {%- endif %}
        {%-endif%}
    {%- endif%}
{%- endfor%}

{%- for reg_name,reg_data in data.regs.items() %}
    {%- if not reg_data.mmio %}
        {%- if reg_data.fields %}

/*******************************************
 * {{reg_name}} - {{reg_data.priv}} - {{reg_data.desc}} 
 */
{%- set ctype_reg = reg_data|csr_ctype %}
{%- set ctype_arg = reg_data|arg_ctype %}
            {%-for field_name,field_data in reg_data.fields.items() %}
            {%- set bit_width = field_data|csr_bit_width_rs %}
            {%- set bit_offset = field_data|csr_bit_offset_rs %}
pub const {{reg_name|upper}}_{{field_name|upper}}_BIT_OFFSET:isize   = {{bit_offset}};
pub const {{reg_name|upper}}_{{field_name|upper}}_BIT_WIDTH:isize    = {{bit_width}};
pub const {{reg_name|upper}}_{{field_name|upper}}_BIT_MASK:UintXlen = {{bit_offset|csr_format_mask_rs(bit_width)}};
pub const {{reg_name|upper}}_{{field_name|upper}}_ALL_SET_MASK:UintXlen = {{0|csr_format_mask_rs(bit_width)}};
            {%- endfor%}
        {%-endif%}
    {%-endif%}
{%- endfor%}

