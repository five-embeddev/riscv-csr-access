/*
   Register access classes for RISC-V system registers.
   SPDX-License-Identifier: Unlicense

   https://five-embeddev.com/

*/

#ifndef RISCV_CSR_HPP
#define RISCV_CSR_HPP

#include <cstdint>

// Test for Zicsr extension, if relevant
#if defined(__riscv_arch_test)
#if !defined(__riscv_zicsr)
#error "-march must include zicsr to access CSRs" 
#endif
#endif

// ------------------------------------------------------------------------
// Base and common classes

namespace riscv {
    namespace csr {
#if __riscv_xlen==32 
        using uint_xlen_t  = std::uint32_t;
        using uint_csr32_t = std::uint32_t;
        using uint_csr64_t = std::uint32_t;
#elif __riscv_xlen==64
        using uint_xlen_t  = std::uint64_t;
        using uint_csr32_t = std::uint32_t;
        using uint_csr64_t = std::uint64_t;
#else
#error "riscv::csr: unknown __riscv_xlen"
#endif
        /** Immediate instructions use a 5 bit immediate field */
        static constexpr uint_xlen_t CSR_IMM_OP_MASK = 0x01F; 
        
        /** CSR: Read only, and read-write base class */
        template<class C> class read_only_reg {
        public :
            using read_datatype_t = typename C::datatype;

            read_only_reg(void) {}
            read_only_reg(const read_only_reg&) = delete;
            read_only_reg& operator=(const read_only_reg&) = delete;
        
            /** Read the CSR value */
            static inline read_datatype_t read(void) {
                return C::read();
            }
            /** Operator alias to read the CSR value */
            inline read_datatype_t operator()(void) {
                return C::read();
            }
        };
        /** CSR: Write only, and read-write base class */
        template<class C> class write_only_reg {
        public :
            using write_datatype_t = typename C::datatype;

            write_only_reg(void) {}
            write_only_reg(const write_only_reg&) = delete;
            write_only_reg& operator=(const write_only_reg&) = delete;

            /** Write a constant to the CSR. */
            template<write_datatype_t VALUE> void write_const(void) {
                if constexpr ((VALUE & CSR_IMM_OP_MASK) == VALUE) {
                    C::write_imm(VALUE);
                } else {
                    C::write(VALUE);
                }
            }
            /** Write to the CSR. */
            inline void write(const write_datatype_t value) {
                C::write(value);
            }
            /** Set a constant mask of bits in the CSR. */
            template<write_datatype_t MASK> void set_const(void) {
                if constexpr ((MASK & CSR_IMM_OP_MASK) == MASK) {
                    C::set_bits_imm(MASK);
                } else {
                    C::set_bits(MASK);
                }
            }
            /** Set a mask of bits in the CSR. */
            inline void set(write_datatype_t mask) {
                C::set_bits(mask);
            }
            /** Clear a constant mask of bits in the CSR. */
            template<write_datatype_t MASK> void clr_const(void) {
                if constexpr ((MASK & CSR_IMM_OP_MASK) == MASK) {
                    C::clr_bits_imm(MASK);
                } else {
                    C::clr_bits(MASK);
                }
            }
            /** Clear a mask of bits in the CSR. */
            inline void clr(write_datatype_t mask) {
                C::clr_bits(mask);
            }
            /** Operator alias to set mask of bits in the CSR. */
            inline void operator|=(write_datatype_t mask) {
                C::set_bits(mask);
            }
        };
        /** CSR: Read-write base class */
        template<class C> class read_write_reg : public read_only_reg<C>,
                                                 public write_only_reg<C>  {
        public:
            using datatype_t = typename C::datatype;

            read_write_reg(void) 
                : read_only_reg<C>()
                , write_only_reg<C>()
                {}
            read_write_reg(const read_write_reg&)=delete;
            read_write_reg& operator=(const read_write_reg&)=delete;

            /** Read from, then write a constant value to the CSR. */
            template<datatype_t VALUE> datatype_t read_write_const(void) {
                if constexpr ((VALUE & CSR_IMM_OP_MASK) == VALUE) {
                    return C::read_write_imm(VALUE);
                } else {
                    return C::read_write(VALUE);
                }
            }
            /** Read from, then write to the CSR. */
            inline datatype_t read_write(const datatype_t value) {
                return C::read_write(value);
            }
            /** Read from, then set a constant bit mask to the CSR. */
            template<datatype_t MASK> datatype_t read_set_bits_const(void) {
                if constexpr ((MASK & CSR_IMM_OP_MASK) == MASK) {
                    return C::read_set_bits_imm(MASK);
                } else {
                    return C::read_set_bits(MASK);
                }
            }
            /** Read from, then set a bit mask to the CSR. */
            inline datatype_t read_set_bits(const datatype_t mask) {
                return C::read_set_bits(mask);
            }
            /** Read from, then clear a constant bit mask to the CSR. */
            template<datatype_t MASK> datatype_t read_clr_bits_const(void) {
                if constexpr ((MASK & CSR_IMM_OP_MASK) == MASK) {
                    return C::read_clr_bits_imm(MASK);
                } else {
                    return C::read_clr_bits(MASK);
                }
            }
            /** Read from, then clear a bit mask to the CSR. */
            inline datatype_t read_clr_bits(const datatype_t mask) {
                return C::read_clr_bits(mask);
            }
        };
        /** CSR Field: Read only, and read-write base class */
        template<class C, class F> class read_only_field {
        public:
            using read_datatype_t = typename F::datatype;

            read_only_field(void) {}
            read_only_field(const read_only_field&)=delete;
            read_only_field& operator=(const read_only_field&)=delete;

            /** Read a given field value from a CSR */
            read_datatype_t read(void) {
                return (read_datatype_t) ((C::read() & F::BIT_MASK) >> F::BIT_OFFSET);
            }
        };
        /** CSR Field: Write only, and read-write base class */
        template<class C, class F> class write_only_field {
        public:
            using write_datatype_t = typename F::datatype;
            using reg_write_datatype_t = typename C::datatype;

            write_only_field(void) {}
            write_only_field(const write_only_field&)=delete;
            write_only_field& operator=(const write_only_field&)=delete;

            inline void set(void) {
                if constexpr ((F::BIT_MASK & CSR_IMM_OP_MASK) == F::BIT_MASK) {
                    C::set_bits_imm(F::BIT_MASK);
                } else {
                    C::set_bits(F::BIT_MASK);
                }
            }
            inline void clr(void) {
                if constexpr ((F::BIT_MASK & CSR_IMM_OP_MASK) == F::BIT_MASK) {
                    C::clr_bits_imm(F::BIT_MASK);
                } else {
                    C::clr_bits(F::BIT_MASK);
                }
            }
        };
        /** CSR Field: Read-write base class */
        template<class C, class F> class read_write_field 
            : public read_only_field<C,F>
            , public write_only_field<C,F>  {
        public:
            using datatype_t = typename F::datatype;
            using reg_datatype_t = typename C::datatype;

            read_write_field(void) 
                : read_only_field<C,F>()
                , write_only_field<C,F>()
                {}
            read_write_field(const read_write_field&)=delete;
            read_write_field& operator=(const read_write_field&)=delete;

            /* Read-modify-write to write a field.
               NOTE - not atomic.
             */
            inline void write(const datatype_t value) {
                auto org_value = C::read();
                auto new_value = (org_value & ~F::BIT_MASK) | 
                    (((reg_datatype_t)value << F::BIT_OFFSET) & F::BIT_MASK);
                C::write(new_value);
            }
            /* Read-modify-write to set a field, and return original value.
               NOTE - not atomic.
             */
            inline datatype_t read_write(const datatype_t value) {
                auto org_value = C::read();
                auto new_value = (org_value & ~F::BIT_MASK) | 
                    (((reg_datatype_t)value << F::BIT_OFFSET) & F::BIT_MASK);
                C::write(new_value);
                return (datatype_t) ((org_value & F::BIT_MASK) >> F::BIT_OFFSET);
            }

        };

        /** CSR access context and read/write permission.
          */
        typedef enum {            
            {% for p in data.addr.priv['values'] -%}
            {% for rw in data.addr.rw['values'] -%}
            {{p.key}}{{rw.key}},
            {% endfor -%}
            {% endfor %}
            {% for rw in data.addr.rw['values'] -%}
            D{{rw.key}},
            {% endfor -%}
        } priv_t;

    } /* csr */
} /* riscv */

// ------------------------------------------------------------------------
// Assembler operations and bit field definitions

namespace riscv {
    namespace csr {
{%- for reg_name,reg_data in data.regs.items() %}

        // ----------------------------------------------------------------
        // {{reg_name}} - {{reg_data.priv}} - {{reg_data.desc}} 
        //
    {%- set ctype_reg = reg_data|csr_ctype %}
    {%- set ctype_arg = reg_data|arg_ctype %}
    {%- set ctype_imm = "const uint8_t" %}
    {%- if not reg_data.mmio %}
        /** {{reg_data.desc}} assembler operations */
        struct {{reg_name}}_ops  {
            using datatype = {{ctype_reg}};
            static constexpr priv_t priv = {{reg_data.priv}}; 
            {% if "R" in reg_data.priv %}
            /** Read {{reg_name}} */
            static {{ctype_arg}} read(void) {
                {{ctype_reg}} value;        
                __asm__ volatile ("csrr    %0, {{reg_name}}" 
                                  : "=r" (value)  /* output : register */
                                  : /* input : none */
                                  : /* clobbers: none */);
                return value;
            }
            {%endif%}
            {% if "W" in reg_data.priv %}
            /** Write {{reg_name}} */
            static void write({{ctype_reg}} value) {
                __asm__ volatile ("csrw    {{reg_name}}, %0" 
                                  : /* output: none */ 
                                  : "r" (value) /* input : from register */
                                  : /* clobbers: none */);
            }
            /** Write immediate value to {{reg_name}} */
            static void write_imm({{ctype_reg}} value) {
                __asm__ volatile ("csrwi    {{reg_name}}, %0" 
                                  : /* output: none */ 
                                  : "i" (value) /* input : from immediate */
                                  : /* clobbers: none */);
            }
            /** Read and then write to {{reg_name}} */
            static {{ctype_arg}} read_write({{ctype_reg}} new_value) {
                {{ctype_reg}} prev_value;
                __asm__ volatile ("csrrw    %0, {{reg_name}}, %1"  
                                  : "=r" (prev_value) /* output: register %0 */
                                  : "r" (new_value)  /* input : register */
                                  : /* clobbers: none */);
                return prev_value;
            }
            /** Read and then write immediate value to {{reg_name}} */
            static {{ctype_arg}} read_write_imm({{ctype_imm}} new_value) {
                {{ctype_reg}} prev_value;
                __asm__ volatile ("csrrwi    %0, {{reg_name}}, %1"  
                                  : "=r" (prev_value) /* output: register %0 */
                                  : "i" (new_value)  /* input : register */
                                  : /* clobbers: none */);
                return prev_value;
            }
        
            // ------------------------------------------
            // Register CSR bit set and clear instructions

            /** Atomic modify and set bits for {{reg_name}} */
            static void set_bits({{ctype_reg}} mask) {
                __asm__ volatile ("csrrs    zero, {{reg_name}}, %0"  
                                  : /* output: none */ 
                                  : "r" (mask)  /* input : register */
                                  : /* clobbers: none */);
            }
            /** Atomic read and then and set bits for {{reg_name}} */
            static uint32_t read_set_bits({{ctype_reg}} mask) {
                {{ctype_reg}} value;
                __asm__ volatile ("csrrs    %0, {{reg_name}}, %1"  
                                  : "=r" (value) /* output: register %0 */
                                  : "r" (mask)  /* input : register */
                                  : /* clobbers: none */);
                return value;
            }
            /** Atomic modify and clear bits for {{reg_name}} */
            static void clr_bits({{ctype_reg}} mask) {
                __asm__ volatile ("csrrc    zero, {{reg_name}}, %0"  
                                  : /* output: none */ 
                                  : "r" (mask)  /* input : register */
                                  : /* clobbers: none */);
            }
            /** Atomic read and then and clear bits for {{reg_name}} */
            static uint32_t read_clr_bits({{ctype_reg}} mask) {
                {{ctype_reg}} value;
                __asm__ volatile ("csrrc    %0, {{reg_name}}, %1"  
                                  : "=r" (value) /* output: register %0 */
                                  : "r" (mask)  /* input : register */
                                  : /* clobbers: none */);
                return value;
            }
        
            // ------------------------------------------
            // Immediate value CSR bit set and clear instructions (only up to 5 bits)
        
            /** Atomic modify and set bits from immediate for {{reg_name}} */
            static void set_bits_imm({{ctype_imm}} mask) {
                __asm__ volatile ("csrrsi    zero, {{reg_name}}, %0"  
                                  : /* output: none */ 
                                  : "i" (mask)  /* input : register */
                                  : /* clobbers: none */);
            }
            /** Atomic read and then and set bits from immediate for {{reg_name}} */
            static {{ctype_arg}} read_set_bits_imm({{ctype_imm}} mask) {
                {{ctype_reg}} value;
                __asm__ volatile ("csrrsi    %0, {{reg_name}}, %1"  
                                  : "=r" (value) /* output: register %0 */
                                  : "i" (mask)  /* input : register */
                                  : /* clobbers: none */);
                return value;
            }
            /** Atomic modify and clear bits from immediate for {{reg_name}} */
            static void clr_bits_imm({{ctype_imm}} mask) {
                __asm__ volatile ("csrrci    zero, {{reg_name}}, %0"  
                                  : /* output: none */ 
                                  : "i" (mask)  /* input : register */
                                  : /* clobbers: none */);
            }
            /** Atomic read and then and clear bits from immediate for {{reg_name}} */
            static {{ctype_arg}} read_clr_bits_imm({{ctype_imm}} mask) {
                {{ctype_reg}} value;
                __asm__ volatile ("csrrci    %0, {{reg_name}}, %1"  
                                  : "=r" (value) /* output: register %0 */
                                  : "i" (mask)  /* input : register */
                                  : /* clobbers: none */);
                return value;
            }
            {%endif%}
        }; /* {{reg_name}}_ops */
        {%- if reg_data.fields %}
        /** Parameter data for fields in {{reg_name}} */
        namespace {{reg_name}}_data {
            {%-for field_name,field_data in reg_data.fields.items() %}
            {%- set bit_width = field_data|csr_bit_width %}
            {%- set bit_offset = field_data|csr_bit_offset %}
            /** Parameter data for {{field_name}} */
            struct {{field_name}} {
                using datatype = {{field_data|csr_ctype}};
                static constexpr {{ctype_reg}} BIT_OFFSET = {{bit_offset}};
                static constexpr {{ctype_reg}} BIT_WIDTH  = {{bit_width}};
                static constexpr {{ctype_reg}} BIT_MASK   = {{bit_offset|csr_format_mask(bit_width)}};
                static constexpr {{ctype_reg}} ALL_SET_MASK = {{0|csr_format_mask(bit_width)}};
            };
            {%- endfor%}
        } /* {{reg_name}}_data */
        {%-endif%}
    {%- endif%}
{%- endfor%}
    } /* csr */
} /* riscv */


// ------------------------------------------------------------------------
// Register and field interface classes.
namespace riscv {
    namespace csr {
{%-for reg_name,reg_data in data.regs.items() %}
    {%-if not reg_data.mmio %}
        {%- set reg_template_class = "riscv::csr::" + reg_name + "_ops" %}
        {%-if "WO" in reg_data.priv %}
            {%- set priv = "write_only" %}
        {%-elif "RW" in reg_data.priv %}
            {%- set priv = "read_write" %}
        {%-else%}
            {%- set priv = "read_only" %}
        {%-endif%}
        /* {{reg_data.desc}} */
        template<class OPS> class {{reg_name}}_reg : public {{priv}}_reg<OPS>
        {
            {%- if reg_data.fields %}
            public:
              {%-for field_name,field_data in reg_data.fields.items() %}
                {%- set field_template_class = "OPS, riscv::csr::" + reg_name + "_data::" + field_name %}
                {{priv}}_field<{{field_template_class}}> {{field_name}};
            {%- endfor %}
            {%- endif %}
        };
        using {{reg_name}} = {{reg_name}}_reg<{{reg_template_class}}>;
    {%-endif%}
{%-endfor%}

        /** Encapsulate all CSRs in a single structure.
           - No storage is required by this class.
         */
        struct all {
        {%-for reg_name,reg_data in data.regs.items() %}
            {%-if not reg_data.mmio %}
            /* {{reg_data.desc}} */
            riscv::csr::{{reg_name}} {{reg_name}};
            {%-endif%}
        {%-endfor%}
        };

    } /* csr */

    static csr::all csrs;

} /* riscv */

#endif // #define RISCV_CSR_HPP
