# Overrides

In order to use libraries that don't support bricks, you will need to add overrides to your config.

You can override the following (optional) properties:
- `build`: the `;` separated commands used to build the project 
- `run`: the `;` separated commands used to run the project after building 
- `include_dir`: the relative path to where to look for header files instead of `<lib>/build/include` 
- `lib_dir`: the relative path to where to look for lib files instead of `<lib>/build/lib` 

Here's an example for using raylib through git:
```toml
[libs.raylib]
kind = "git"
repo = "https://github.com/raysan5/raylib.git"
version = "5.5"
overrides.build = "cd src; make"
overrides.include_dir = "src"
overrides.lib_dir = "src"
# yes, raylib builds into src for some reason
```

## Brick level overrides

If you want to add `bricks` support to your project, you can use overrides there too if you need.

```toml
[brick]
name = "game"
kind = "binary"
lang = "c"
edition = "c99"
overrides.build = "make"
overrides.include_dir = "src"
overrides.run = "./game"
```

Of course, this can also be someone else's library!

If you want to use an external library that doesn't support `bricks`, it is recommended to **fork** the repo, 
adding a `brick.toml` (with overrides if needed)
