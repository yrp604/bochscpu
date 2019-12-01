# bochscpu

## install

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
