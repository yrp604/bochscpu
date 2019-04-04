# bochscpu

## install

```sh
$ git checkout git@github.com:pquux/bochscpu.git
$ cd bochscpu
$ svn co http://svn.code.sf.net/p/bochs/code/trunk/bochs bochs
$ cd bochs
$ svn -r13555 up # check out the second to last revision that actually builds
$ source .conf.cpu
$ make
$ cd ..
$ cargo build
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
    }

    let c = Cpu::new(0);
    c.run();
}
```
