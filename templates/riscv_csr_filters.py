# Regiter size conversion used by yaml_jinja.py
# 
# Jinja filters for the following register data structure
# 'bits' : (msb, lsb)

import re

def _xlen_replace(bit):
    return re.sub('[mxsu]xlen','__riscv_xlen', str(bit))

def _csr_bit_to_int(bit):
    if isinstance(bit, int):
        return bit
    raise "Compile time defined."

def csr_bit_width(field_data):
    if len(field_data["bits"])==1 :
        return 1
    else :
        try:
            msb = _csr_bit_to_int(field_data["bits"][0])
            lsb = _csr_bit_to_int(field_data["bits"][1])
            return (msb - lsb) + 1
        except:
            return "((" + _xlen_replace(field_data["bits"][0]) + ")-(" + _xlen_replace(field_data["bits"][1]) + ") + 1)"

def csr_bit_width_rs(field_data):
    if len(field_data["bits"])==1 :
        return 1
    else :
        try:
            msb = _csr_bit_to_int(field_data["bits"][0])
            lsb = _csr_bit_to_int(field_data["bits"][1])
            return (msb - lsb) + 1
        except:
            return "(" + _xlen_replace(field_data["bits"][0]) + ")-(" + _xlen_replace(field_data["bits"][1]) + ") + 1"

def csr_field_imm_valid(field_data):
    # Immediate can only store 5 bits
    try:
        msb = _csr_bit_to_int(field_data["bits"][0])
        if msb < 5:
            return True
    except:
        pass
    return False


def csr_format_mask(bit_offset, bit_width):
    """ Convert a bit offset and width into a mask.
    .e.g bit_offset5, bit_width=4 will convert to 0x1C0
    """
    if isinstance(bit_offset, int) and isinstance(bit_width, int):
        return "0x%x" % (((1<<bit_width)-1)<<bit_offset)
    else:
        if isinstance(bit_width, int) :
            mask = "0x%xUL" % (((1<<bit_width)-1))
        else :
            mask = "(1UL<<(" + str(bit_width) + "-1))"
        return "(" + mask + " << (" + str(bit_offset) + "))"

def csr_format_mask_rs(bit_offset, bit_width):
    """ Convert a bit offset and width into a mask.
    .e.g bit_offset5, bit_width=4 will convert to 0x1C0
    """
    if isinstance(bit_offset, int) and isinstance(bit_width, int):
        return "0x%x" % (((1<<bit_width)-1)<<bit_offset)
    else:
        if isinstance(bit_width, int) :
            mask = "0x%x" % (((1<<bit_width)-1))
        else :
            mask = "(1<<(" + str(bit_width) + "-1))"
        return  mask + " << (" + str(bit_offset) + ")"


def csr_bit_offset(field_data):
    try:
        return _csr_bit_to_int(field_data["bits"][-1])
    except:
        return "(" + _xlen_replace(field_data["bits"][-1]) + ")"

def csr_bit_offset_rs(field_data):
    try:
        return _csr_bit_to_int(field_data["bits"][-1])
    except:
        return  _xlen_replace(field_data["bits"][-1]) 
        
def arg_ctype(field_data):
    """ The type passed to, or returned from, the wrapper function.
    """
    if "width" in field_data:
        width=str(field_data["width"])
        if width.find("xlen") > 0:
            return "uint_xlen_t"
        else:
            return "uint" + str(width) + "_t"
    return "uint_xlen_t"

def csr_ctype(field_data):
    """ The type passed to, or returned from, the assembler instruction.
    """
    if "width" in field_data:
        width=str(field_data["width"])
        if width.find("xlen") > 0:
            return "uint_xlen_t"
        else:
            return "uint_csr" + str(width) + "_t"
    return "uint_xlen_t"

def csr_ctype_rs(field_data):
    """ The type passed to, or returned from, the assembler instruction.
    """
    if "width" in field_data:
        width=str(field_data["width"])
        if width.find("xlen") > 0:
            return "UintXlen"
        else:
            return "UintCsr" + str(width) 
    return "UintXlen"


def setup(env):
    """ Register the filters in this file to the Jinja environment.
    """
    env.filters['csr_bit_width'] = csr_bit_width
    env.filters['csr_bit_width_rs'] = csr_bit_width_rs
    env.filters['csr_bit_offset'] = csr_bit_offset 
    env.filters['csr_bit_offset_rs'] = csr_bit_offset_rs
    env.filters['csr_format_mask'] = csr_format_mask
    env.filters['csr_format_mask_rs'] = csr_format_mask_rs
    env.filters['csr_ctype'] = csr_ctype
    env.filters['csr_ctype_rs'] = csr_ctype_rs
    env.filters['arg_ctype'] = arg_ctype
    env.filters['csr_field_imm_valid'] = csr_field_imm_valid
