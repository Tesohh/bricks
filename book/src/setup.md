# Setup
In order to setup a project (a "brick"), first of all run:

```sh
bricks init <project_name>
```
This will create a binary project with basic configuration using C99.

## Flags

- If you need C++, add the `--cpp` flag. 
- If this project is a library, add the `--lib` flag.

## The `brick.toml` file

This is where you configure your project, kind of like `Cargo.toml` or `package.json`.

Yours will look something like this:
```toml
[brick]
name = "<project_name>"
kind = "binary"
lang = "c"
edition = "c99"
```

See the next section so learn how to add libraries.
