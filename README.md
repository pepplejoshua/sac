# sac

sac or small arm compiler is my first attempt at compiling to native code, targetting ARM.

## to compile a sac file (with .sac extension)

My set up is for Linux running on x86_64. I need:

- Rust toolchain is [installed](https://www.rust-lang.org/tools/install).
- arm-linux-gnueabihf-gcc is installed.
  - `sudo apt install gcc-arm-linux-gnueabihf`
- qemu-user is installed.
  - `sudo apt install qemu-user`

to compile play.sac, use:

```bash
./c play
```

it will generate an ARM32 file, `play.s`. To show running time of exe, you can uncomment line 11 in `./c`:

```bash
# time -p qemu-arm "$1"
```
