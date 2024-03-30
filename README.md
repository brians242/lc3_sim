# A LC-3 Virtual Machine in Rust

An LC-3 virtual machine (ANSI terminal) implemented in Rust. Built with the intention of learning computer architecture implementations while giving a way pass time in places without internet access.

Implemented using windows API (for good default ANSI usage), while created on a mac.

## Sources:
Learning:
- https://www.jmeiners.com/lc3-vm/
    - https://github.com/justinmeiners/lc3-vm
- https://www.rodrigoaraujo.me/posts/lets-build-an-lc-3-virtual-machine/

Games:
- https://github.com/rpendleton/lc3-2048
- https://github.com/justinmeiners/lc3-rogue
- https://github.com/jameslu1/Connect-4-on-LC-3

## Running
`cargo run -- src/games/<game_name>.obj`

If you choose to use the LC-3 VM for any other purpose, and create an LC-3 assembly program that you convert to a .obj file:
- You can drag it into the games folder (or rename it for your own purposes) and just run `cargo run -- src/games/<project_name>.obj`