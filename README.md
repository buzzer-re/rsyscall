# RSyscall

A simple client to [syscall.sh](https://syscall.sh) written in Rust.


## Introduction

[syscall.sh](https://syscall.sh) provied a good table with all Linux syscalls for: x86, x64, ARM and ARM64, it's very useful when you don't know all syscalls and their arguments.

And their also have an [API](https://api.syscall.sh) which give access to everything in their website! So it's very nice to have a CLI which is simple to use and help a lot when developing somehting that's uses low level syscalls.

(I'm also learning Rust, so this was a cool project to learn A LOT)


## Usage

You can easily chek all the options with the `-h` modifier.

```
$ rsyscall -h                                   
rsyscall 1.0

USAGE:
    rsyscall --arch <ARCH> --syscall <SYSCALL>

OPTIONS:
    -a, --arch <ARCH>          Arch type to be searched
    -h, --help                 Print help information
    -s, --syscall <SYSCALL>    Syscall name to be searched
    -V, --version              Print version information
```

## Getting `openat` parameters for x64 and ARM

```
$ rsyscall -a x64 -s openat

openat:
Syscall Number = 257
return = rax
rdi = int dfd
rsi = const char *filename
rdx = int flags
r10 = umode_t mode
r8 = NULL
r9 = NULL

$ rsyscall -a arm -s openat
openat:
Syscall Number = 322
return = r0
r0 = int dfd
r1 = const char *filename
r2 = int flags
r3 = umode_t mode
r4 = NULL
r5 = NULL
```

It's very simple, isn't ?

The archs supported are:

* x86
* x64
* arm
* arm64

## Installing


The simpliest way to install is using `cargo`:

```
$ git clone github.com/aandersonl/rsyscall.git
$ cd rsyscall
$ cargo install --path .
```


## Future

There are a few features tha I want to implement soon, they are:

- [ ] Cache all syscalls in the local system for quick access
- [ ] Change the output to use tables


That's all.






