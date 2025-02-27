# Platforms

Platforms allow you to have certain configs only for certain platforms.

Here is an example config that uses 

```toml
[brick]
name = "project"
kind = "binary"
lang = "c"
edition = "c99"
macos.ldflags = "-framework Cocoa -framework OpenGL -framework IOKit -framework CoreVideo"
macos.cflags = "-Wall"
windows.ldflags = "-lgdi32 -lwinmm -lopengl32"
```
