use anyhow::Result;
use bricks::{config::Config, pretty};
use owo_colors::OwoColorize;

fn _main() -> Result<()> {
    let toml_str = r#"
[brick]
name = "smarciopoly"
kind = "binary"
lang = "c"
edition = "c99"

[packages]
list = ["utils"]

[libs.raylib]
src = "system"

[libs.strings]
src = "web:github.com/Tesohh/strings"
version = "latest"

[libs.slices]
src = "web:github.com/Tesohh/vectors"
version = "b46ca7f"
build = "make"
headers = ["slices.h"]
objects = ["slices.o"]
"#;

    let config: Config = toml::from_str(toml_str)?;
    pretty::msg(
        "brick",
        format!(
            "{} {}",
            config.brick.name,
            format!(
                "({}, {}, {})",
                config.brick.kind, config.brick.lang, config.brick.edition
            )
            .dimmed()
        ),
    );
    Ok(())
}

fn main() {
    let result = _main();
    match result {
        Ok(_) => {}
        Err(err) => pretty::error(err),
    }
}
