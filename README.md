# bricks
build system and package manager for C/C++


## Installing
For now the only way to use `bricks` is by building from source.

```sh
git clone https://github.com/Tesohh/bricks.git
cd bricks
cargo install --path .
```

## Project setup
To initialize a new project (binary):

```sh
bricks init project_name
cd project_name
```

To add to an existing project:
- Add the basic config template to `brick.toml` in your project root
- See ()

After adding all dependencies to the `brick.toml` file, run `bricks install`

## Config
Here is an example config for a raylib project:

```toml
[brick]
name = "project_name"
kind = "binary"
lang = "c"
edition = "c99"

[libs.raylib]
kind = "system"
```

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
