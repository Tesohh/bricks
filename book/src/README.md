# `bricks`
build system and package manager for C/C++

## why `bricks`
`bricks` is a simple solution to the dreaded build system.

Yes, I know that there are already *enough* build systems, meta build systems, and all that
(insert obligatory xkcd comic here).

Here are the "killer features" in my opinion:
- Super quick and easy project setup
- Easy to integrate in existing projects
- Package management, without a central repository

(by the way, it's called `bricks` because i imagined every package being the brick of a building.)

## is this for me?
`bricks` is for you if:
- you don't want to be bothered with build systems
- you use an editor that supports `clangd` (neovim/vim, vscode)

`bricks` is not for you if:
- you need to generate multiple build scripts like cmake does, for IDEs and such
