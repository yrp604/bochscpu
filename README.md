# bochscpu

## install

Run `cargo build` - it will download the prebuilt artifacts from [bochscpu-build](https://github.com/yrp604/bochscpu-build) and extract the `lib` and `bochs` folders into the `bochscpu` checkout. 

This will result in a tree like follow:

```
bochscpu$ ls -l
total 20
drwxrwxrwx 1 x x 4096 Jan  3 00:09 bochs      # bochs directory from pre-built artifacts
-rwxrwxrwx 1 x x 8318 Jan  2 23:28 build.rs
drwxrwxrwx 1 x x 4096 Jan  2 23:28 cabi
-rwxrwxrwx 1 x x 4772 Jan  3 00:16 Cargo.lock
-rwxrwxrwx 1 x x  427 Jan  2 23:28 Cargo.toml
drwxrwxrwx 1 x x 4096 Jan  3 00:09 lib        # lib directory from pre-built artifacts
-rwxrwxrwx 1 x x  502 Jan  2 23:28 README.md
drwxrwxrwx 1 x x 4096 Jan  3 00:20 src
drwxrwxrwx 1 x x 4096 Jan  3 00:16 target
-rwxrwxrwx 1 x x  276 Jan  2 23:28 TODO.md
```

## usage

bochscpu exposes all the instrumentation points that bochs does. These are
documented [here](https://github.com/bochs-emu/Bochs/blob/master/bochs/instrument/instrumentation.txt).

For an example of initalizing and using the emulator, see the source code for
the [benchmark example](https://github.com/yrp604/bochscpu-bench/blob/master/src/fib.rs).
