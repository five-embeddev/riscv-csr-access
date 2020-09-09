/*
   Register access functions for RISC-V system registers.
   SPDX-License-Identifier: Unlicense

   https://five-embeddev.com/

*/

#ifndef RISCV_CSR_H
#define RISCV_CSR_H

#include <stdint.h>

#if __riscv_xlen==32 
typedef uint32_t uint_xlen_t;
typedef uint32_t uint_csr32_t;
typedef uint32_t uint_csr64_t;
#elif __riscv_xlen==64
typedef uint64_t uint_xlen_t;
typedef uint32_t uint_csr32_t;
typedef uint64_t uint_csr64_t;
#else
#error "Unknown XLEN"
#endif


{%- for reg_name,reg_data in data.regs.items() %}
    {%- if not reg_data.mmio %}

/*******************************************
 * {{reg_name}} - {{reg_data.priv}} - {{reg_data.desc}} 
 */
{%- set ctype_reg = reg_data|csr_ctype %}
{%- set ctype_arg = reg_data|arg_ctype %}
            {%- if "R" in reg_data.priv %}
static inline {{ctype_arg}} csr_read_{{reg_name}}(void) {
    {{ctype_reg}} value;        
    __asm__ volatile ("csrr    %0, {{reg_name}}" 
                      : "=r" (value)  /* output : register */
                      : /* input : none */
                      : /* clobbers: none */);
    return value;
}
            {%- endif %}
            {%- if "W" in reg_data.priv %}
static inline void csr_write_{{reg_name}}({{ctype_reg}} value) {
    __asm__ volatile ("csrw    {{reg_name}}, %0" 
                      : /* output: none */ 
                      : "r" (value) /* input : from register */
                      : /* clobbers: none */);
}
static inline {{ctype_arg}} csr_read_write_{{reg_name}}({{ctype_arg}} new_value) {
    {{ctype_reg}} prev_value;
    __asm__ volatile ("csrrw    %0, {{reg_name}}, %1"  
                      : "=r" (prev_value) /* output: register %0 */
                      : "r" (new_value)  /* input : register */
                      : /* clobbers: none */);
    return prev_value;
}
            {%- endif%}
            {%- if reg_data.fields %}
            {%- if "W" in reg_data.priv %}
/* Register CSR bit set and clear instructions */
static inline void csr_set_bits_{{reg_name}}({{ctype_arg}} mask) {
    __asm__ volatile ("csrrs    zero, {{reg_name}}, %0"  
                      : /* output: none */ 
                      : "r" (mask)  /* input : register */
                      : /* clobbers: none */);
}
static inline void csr_clr_bits_{{reg_name}}({{ctype_arg}} mask) {
    __asm__ volatile ("csrrc    zero, {{reg_name}}, %0"  
                      : /* output: none */ 
                      : "r" (mask)  /* input : register */
                      : /* clobbers: none */);
}
static inline {{ctype_arg}} csr_read_set_bits_{{reg_name}}({{ctype_arg}} mask) {
    {{ctype_reg}} value;
    __asm__ volatile ("csrrs    %0, {{reg_name}}, %1"  
                      : "=r" (value) /* output: register %0 */
                      : "r" (mask)  /* input : register */
                      : /* clobbers: none */);
    return value;
}
static inline {{ctype_arg}} csr_read_clr_bits_{{reg_name}}({{ctype_arg}} mask) {
    {{ctype_reg}} value;
    __asm__ volatile ("csrrc    %0, {{reg_name}}, %1"  
                                  : "=r" (value) /* output: register %0 */
                                  : "r" (mask)  /* input : register */
                                  : /* clobbers: none */);
    return value;
}
/* {{reg_name}}, CSR write value via immediate value (only up to 5 bits) */
#define CSR_WRITE_IMM_{{reg_name|upper}}(VALUE)                    \
    __asm__ volatile ("csrrwi    zero, {{reg_name}}, %0"           \
                      : /* output: none */                         \
                      : "i" (VALUE)  /* input : immediate  */      \
                      : /* clobbers: none */)

/* {{reg_name}}, CSR set bits via immediate value mask (only up to 5 bits) */
#define CSR_SET_BITS_IMM_{{reg_name|upper}}(MASK)                 \
    __asm__ volatile ("csrrsi    zero, {{reg_name}}, %0"          \
                      : /* output: none */                        \
                      : "i" (MASK)  /* input : immediate  */      \
                      : /* clobbers: none */)

/* {{reg_name}}, CSR clear bits via immediate value mask (only up to 5 bits) */
#define CSR_CLR_BITS_IMM_{{reg_name|upper}}(MASK)               \
    __asm__ volatile ("csrrci    zero, {{reg_name}}, %0"        \
                      : /* output: none */                      \
                      : "i" (MASK)  /* input : immediate */     \
                      : /* clobbers: none */)

            {%- endif %}
            {%-for field_name,field_data in reg_data.fields.items() %}
            {%- set bit_width = field_data|csr_bit_width %}
            {%- set bit_offset = field_data|csr_bit_offset %}
#define {{reg_name|upper}}_{{field_name|upper}}_BIT_OFFSET   {{bit_offset}}
#define {{reg_name|upper}}_{{field_name|upper}}_BIT_WIDTH    {{bit_width}}
#define {{reg_name|upper}}_{{field_name|upper}}_BIT_MASK     {{bit_offset|csr_format_mask(bit_width)}}
#define {{reg_name|upper}}_{{field_name|upper}}_ALL_SET_MASK {{0|csr_format_mask(bit_width)}}
            {%- endfor%}
        {%-endif%}
    {%- endif%}
{%- endfor%}


#endif // #define RISCV_CSR_H
