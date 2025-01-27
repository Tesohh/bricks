pub fn config(name: &str, kind: &str, lang: &str, edition: &str) -> String {
    format!(
        r#"[brick]
name = "{}"
kind = "{}"
lang = "{}"
edition = "{}""#,
        name, kind, lang, edition
    )
}

pub fn main_c(name: &str) -> String {
    format!(
        r#"#include <stdio.h>

int main() {{
    printf("hello {}!\n");
}}"#,
        name
    )
}

pub fn main_cpp(name: &str) -> String {
    format!(
        r#"#include <iostream>

int main() {{
    cout << "hello {}!" << endl;
}}"#,
        name
    )
}
