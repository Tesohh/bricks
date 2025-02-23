# Adding libraries

## System libraries
If you want to link a system-wide installed library (such as one you installed through your system's package manager),
use the `kind = "system"` property.

Here's an example of linking raylib:

```toml
[brick]
name = "<project_name>"
kind = "binary"
lang = "c"
edition = "c99"

[libs.raylib]
kind = "system"
```

<div class="warning">
    this requires `pkg-config` to be installed.
</div>

## Through git
If you want to download a specific version of a library from a git repository:

```toml
[libs.<brickname>]
kind = "git"
repo = "<repo_uri>"
version = "v2"
```

> Note: the `<brickname>` should be exactly the same as the one in the repo's `brick.toml`.

> Other note: The `version` must be a commit id such as `801e950` or a tag name such as `v2`

This will clone the full repository under `~/.bricks/libs/<repo_uri>/full`, checkout to that commit,
and then copy the library into `<repo_uri>/<version>`.

If this library already has a `brick.toml`, you're done.

If it doesn't, you need to add [overrides](overrides.md) to your config.

> you might need to run `bricks build` before you see headers working in your editor.
