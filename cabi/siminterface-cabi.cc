#include "bochs.h"

namespace rust {
extern "C" {
    bx_param_enum_c *sim_get_param_enum(const char *);
    bx_param_num_c *sim_get_param_num(const char *);
    bx_param_bool_c *sim_get_param_bool(const char *);
    bx_param_string_c *sim_get_param_string(const char *);
}
}

class bx_real_sim_c : public bx_simulator_interface_c {
public:
    bx_real_sim_c();
    virtual bx_param_enum_c *get_param_enum(const char *pname, bx_param_c *base=NULL);
    virtual bx_param_num_c *get_param_num(const char *pname, bx_param_c *base=NULL);
    virtual bx_param_bool_c *get_param_bool(const char *pname, bx_param_c *base=NULL);
    virtual bx_param_string_c *get_param_string(const char *pname, bx_param_c *base=NULL);
};

bx_real_sim_c::bx_real_sim_c() {}

bx_param_enum_c *bx_real_sim_c::get_param_enum(const char *pname, bx_param_c *base)
{
    return rust::sim_get_param_enum(pname);
}

bx_param_num_c *bx_real_sim_c::get_param_num(const char *pname, bx_param_c *base)
{
    return rust::sim_get_param_num(pname);
}

bx_param_bool_c *bx_real_sim_c::get_param_bool(const char *pname, bx_param_c *base)
{
    return rust::sim_get_param_bool(pname);
}

bx_param_string_c *bx_real_sim_c::get_param_string(const char *pname, bx_param_c *base)
{
    return rust::sim_get_param_string(pname);
}

extern "C" {
BOCHSAPI bx_param_enum_c* sim_new_param_enum(const char *name, const char **values,
        Bit32u idx)
{
    return new bx_param_enum_c(
            NULL,
            name,
            NULL,
            NULL,
            values,
            idx,
            0
    );
}

BOCHSAPI void sim_delete_param_enum(bx_param_enum_c *e) {
    delete e;
}

BOCHSAPI bx_param_num_c* sim_new_param_num(const char *name, Bit64u min, Bit64u max,
        Bit64u val)
{
    return new bx_param_num_c(
            NULL,
            name,
            NULL,
            NULL,
            min,
            max,
            val
    );
}

BOCHSAPI void sim_delete_param_num(bx_param_num_c *n) {
    delete n;
}

BOCHSAPI bx_param_bool_c* sim_new_param_bool(const char *name, bx_bool val)
{
    return new bx_param_bool_c(
            NULL,
            name,
            NULL,
            NULL,
            val
    );
}

BOCHSAPI void sim_delete_param_bool(bx_param_bool_c *b) {
    delete b;
}

BOCHSAPI bx_param_string_c* sim_new_param_string(const char *name, const char *val, unsigned max_sz)
{
    return new bx_param_string_c(
            NULL,
            name,
            NULL,
            NULL,
            val,
            max_sz
    );
}

BOCHSAPI void sim_delete_param_string(bx_param_string_c *b) {
    delete b;
}
}

logfunctions *siminterface_log = NULL;
bx_list_c *root_param = NULL;
bx_simulator_interface_c *SIM = new bx_real_sim_c();
