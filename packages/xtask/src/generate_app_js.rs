use std::collections::HashMap;
use std::fs;
use std::process::Command;

use minijinja::{Environment, context};
use wasmparser::{Parser, Payload};

use crate::MyResult;

pub fn run() -> MyResult<()> {
    const FILE: &str = "app.js";

    let commit_hash = commit_hash()?;

    let mut env = Environment::empty();
    let root = crate::repo_root();
    let tmpl = fs::read_to_string(root.join("src").join(format!("{FILE}.j2")))?;

    env.add_template(FILE, &tmpl)?;

    let expanded = env
        .get_template(FILE)?
        .render(context! { commit_hash, env => load_env()? })?;

    fs::write(root.join("dist").join(FILE), expanded)?;

    Ok(())
}

fn load_env() -> MyResult<Vec<String>> {
    let root = crate::repo_root();

    let env_js = fs::read_to_string(root.join("src").join("env.js"))?;
    let all_funcs = env_js
        .split("function $")
        .skip(1)
        .map(|name_and_body| {
            if let Some((name, body)) = name_and_body.split_once('(') {
                Ok((format!("${name}"), format!("({body}")))
            } else {
                Err("malformed env.js")
            }
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    let app_wasm = fs::read(root.join("dist").join("app.wasm"))?;

    let mut used_funcs = vec![];
    for res in Parser::new(0).parse_all(&app_wasm) {
        let payload = res?;

        if let Payload::ImportSection(section) = payload {
            for (index, res) in section.into_iter().enumerate() {
                let import = res?;

                let name = import.name;
                let body = all_funcs
                    .get(name)
                    .ok_or_else(|| format!("env.js is missing the function: {name}"))?;

                used_funcs.push(format!("${index}{body}"));
            }
        }
    }

    Ok(used_funcs)
}

fn commit_hash() -> MyResult<String> {
    stdout(Command::new("git").args(["rev-parse", "HEAD"]))
}

fn stdout(command: &mut Command) -> MyResult<String> {
    let output = command.output()?;
    if !output.status.success() {
        return Err(format!(
            "`{command:?}` failed with exit code {:?}",
            output.status.code()
        )
        .into());
    }
    let mut stdout = String::from_utf8(output.stdout)?;

    while stdout.ends_with(|c: char| c.is_ascii_whitespace()) {
        stdout.pop();
    }

    Ok(stdout)
}
