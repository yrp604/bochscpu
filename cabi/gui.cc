#include "bochs.h"
#include "gui/gui.h"

bx_gui_c *bx_gui = NULL;

bx_gui_c::~bx_gui_c() {}

void bx_gui_c::beep_on(float frequency) {}
void bx_gui_c::beep_off() {}
void bx_gui_c::get_capabilities(Bit16u *xres, Bit16u *yres, Bit16u *bpp) {}
void bx_gui_c::show_ips(Bit32u ips_count) {}
bx_svga_tileinfo_t *bx_gui_c::graphics_tile_info(bx_svga_tileinfo_t *info) { return NULL; }
Bit8u *bx_gui_c::graphics_tile_get(unsigned x0, unsigned y0, unsigned *w, unsigned *h) { return NULL; }
void bx_gui_c::graphics_tile_update_in_place(unsigned x0, unsigned y0, unsigned w, unsigned h) {}