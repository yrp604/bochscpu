# bochscpu

## install linux

```sh
$ git checkout git@github.com:yrp604/bochscpu.git
$ cd bochscpu
$ svn co http://svn.code.sf.net/p/bochs/code/trunk/bochs bochs # last known good: r13566
$ cd bochs
$ source .conf.cpu
$ make # this command should fail with an error about libinstrument.a
$ cd ..
$ cargo build
```

## install windows

In WSL/Cygwin/Linux VM

```sh
$ git checkout git@github.com:yrp604/bochscpu.git
$ cd bochscpu
$ svn co http://svn.code.sf.net/p/bochs/code/trunk/bochs bochs # last known good: r13566
$ cd bochs
$ source .conf.cpu-msvc
```

In VS Cmd Prompt in bochcpu

```
> cd bochs
> build.bat
> fixup-lib-names.bat
> cd ..
> cargo build
```

## usage

```rust
use bochscpu::cpu::Cpu;
use bochscpu::hook;

fn main() {
    stderrlog::new()
        .verbosity(11)
        .init()
        .unwrap();


    unsafe {
        hook::exception(|cpuid, vector, error_code, | {
            println!(
                "[!] exception cpuid: {} vector {:x} error code {:x}",
                cpuid,
                vector,
                error_code
            );
        });

        let c = Cpu::new(0);
        c.run();
    }
}
```

## snapshotting
```
0: kd> !gflag +ksl
0: kd> sxe ld <your binary.exe>
0: kd> g
# run you binary, you should get a bp when its loaded...
0: kd> bp target!function
0: kd> g
# should stop at your target
0: kd> .dump /ka path\to\snap.dmp
# we also need a bunch of msr's, docs on this forthcoming
```

## notes

This guy allocates a lot of pages. You'll likely need to tweak sysctls:
```
# sysctl vm.max_map_count # see the current limit
# sysctl -w sysctl vm.max_map_count=<something above the number of phys pages in your dump>
```
