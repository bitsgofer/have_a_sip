# Towards reproducible tools

I use [nix](https://nix.dev/#what-can-you-do-with-nix) to create a reproducible
development environment.

Basically it will install versioned-lock tools (e.g: Rust, just, git-lfs,
pkg-config, udex, vulkan, etc) that are totally isolated from the the host.
This ensures everyone use the same version of tools to produce similar binary.

> Admittedly, because of `nix`'s nature, it's not very friendly for developing
> on Windows. But I think Linux and OSX is where most developers on anyway.

Nix also make it possible to declare different tools for different platforms
(e.g: Linux machine with AMD CPU/GPU, Macbook with ARM CPU, etc) so we get
packaged dev environment with native performance.

A few years past, a lot of server-side software uses `Dockerfile` to package
a Linux-based dev environment. This doesn't work well for game as we would have
to funnel the graphics, sound, etc outside of the container. And the performace
on Apple's CPU would be horrible.
