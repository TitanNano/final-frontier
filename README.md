final-frontier
==============

GLFrontier's (Frontier: Elite II's OpenGL port), partially ported to rust.

The project has only been tested on macOS.

Building currently happens in two steps:
```
make
cargo build
```


## Building on macOS

To build on macOS you have to run

```
CC="gcc-11" make
xcrun cargo build
```

Or the c code will never finish compiling and cargo won't be able to find the OpenGL framework. You also have to install SDL2 via homebrew.
