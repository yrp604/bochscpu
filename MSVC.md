# msvc install

```sh
$ git checkout git@github.com:yrp604/bochscpu.git
$ cd bochscpu
$ svn co http://svn.code.sf.net/p/bochs/code/trunk/bochs bochs
$ cd bochs
$ svn -r13555 up # check out the second to last revision that actually builds
```

This command needs to be run WSL, Cygwin, Linux VM, whatever
```sh
$ source .conf.cpu-msvc
```

```sh
$ nmake # this command should fail with an error about bochs.h
$ cd cpu/fpu
$ nmake
$ rename libfpu.a fpu.lib
$ cd ..
$ rename libcpu.a cpu.lib
$ cd avx
$ rename libavx.a avx.lib
$ cd ../cpudb
$ rename libcpudb.a cpudb.lib
$ cd ../../
$ move build-msvc.rs build.rs
$ cargo build
```
