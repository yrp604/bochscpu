- [x] Write rust harness to get log output
- [x] Figure out how to zero init the bxcpu
- [ ] Figure out why Asan doesnt work with a rust binary
- [ ] Figure out how to do mutable references to a context struct
- [x] Figure out how to init memory
	* [ ] Maybe replace lazy static with memory init routines
	* [ ] Maybe replace const expr vecs with init routines
- [ ] Figure out how to exit `cpu_loop`
- [ ] Get bochscpu dump and add methods to load/store the cpu state
- [ ] Write windbg-js to dump register state
- [ ] Figure out patching bochs in a not-shit way
	* [ ] Fix rdrand via patching method with rip ^ branch ctr
