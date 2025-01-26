# bricks
build system and package manager for C/C++

## Why?
I would like to make more stuff in C, but I hate the tooling around it.. so I wanted to make my own

I'm making this mostly for myself, but if other libraries use it, that would be great.
`bricks` is designed so that it can be used without interfering with other build systems

## Package management
`bricks` works by 
- parsing the `brick.toml` file in your project
- figuring out what type of project it is, what kind of libraries we need to link etc.
- for every library:
    - if it's a system library, just link it
    - if it comes from github,
        - and it's a `bricks` package:
            - put it in the `~/.bricks/libs/<lib name>/<lib version>`
            - build it through `bricks`
            - add headers to `.clangd` file 
        - if it's a non `bricks` project:
            - put it in the `~/.bricks/libs/<lib name>/<lib version>`
            - we need more hints from the config
            - use the `build` command provided in the config
            - link using the `link` property in the config 
            - add headers defined in config to `.clangd` file

## Build system
- Building works like any other build system.
    - compile all .c files
    - link them if binary,
    - `ar` them if library

## Project structure
Project structure is similar to what you might find in a go or cargo project:

```
project/
    src/
        main.c
        test.c
        utils/
            utils.h
            math.c
            types.c
        game/
            game.h
            game.c
    .git
    README.md
    ...
```

- in this case `utils` and `game` are packages inside the project.
    - it should only have 1 header, with the same name as the directory
    - then all .c files inside will implement that header.

- as for testing, it is all managed by `test.c`. 
    - Perhaps, I will create a testing framework for `bricks`, but that's beyond my scope for now.
