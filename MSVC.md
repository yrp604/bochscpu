# msvc install

## build bochs

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
$ build.bat
$ fixup-lib-names.bat
$ cd ..
```

## build bindings
Once bochs is built
```
$ cargo build
```
