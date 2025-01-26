use bricks::config::Config;

fn main() {
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

    let config: Config = toml::from_str(toml_str).unwrap();
    println!("{:#?}", config);
}
