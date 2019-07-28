#include "bochs.h"

bx_pc_system_c::bx_pc_system_c() {
    a20_mask =  BX_CONST64(0xffffffffffffffff);
    kill_bochs_request = 0;
    currCountdown = 1;
}

int bx_pc_system_c::register_timer(void *this_ptr, bx_timer_handler_t,
    Bit32u useconds, bx_bool continuous, bx_bool active, const char *id)
{
    assert(false);
    return 0;
}

int bx_pc_system_c::register_timer_ticks(void* this_ptr,
    bx_timer_handler_t funct, Bit64u ticks, bx_bool continuous, bx_bool active,
    const char *id)
{
    return 0; // timer id
}

void bx_pc_system_c::activate_timer_ticks(unsigned int index,
    Bit64u instructions, bx_bool continuous)
{
    assert(false);
}

void bx_pc_system_c::deactivate_timer(unsigned int timer_index) { assert(false); }

int bx_pc_system_c::Reset(unsigned int) { assert(false); return 0; }

bx_bool bx_pc_system_c::get_enable_a20(void) { assert(false); return true; }

void bx_pc_system_c::countdownEvent(void)
{
    bx_pc_system.currCountdown = 1;
}
void bx_pc_system_c::invlpg(bx_address addr) { assert(false); }

bx_pc_system_c bx_pc_system;
