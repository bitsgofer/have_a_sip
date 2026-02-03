# `Justfile` instead of `Makefile`

I use `Makefile` quite extensively at work. However, because I mostly use
languages that already comes with a package manager (e.g: Go, Rust),
there is very little value that `make` provides.

Most of the time, I ended up with lots of `.PHONY` targets and hard-to-read
pre-processing code.

Because we already use `nix` to package the tools and `cargo` to compile
the source code, there is very little use for Make besides being a task runner.

Hence, I decided to try out `just` for this new project.

Let's see how it will go from there.
