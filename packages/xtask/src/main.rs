use std::path::PathBuf;
use std::{array, env};

mod compress_app_wasm;
mod generate_app_js;
mod minify;
mod print_sizes;
mod serve;

type MyError = Box<dyn std::error::Error>;
type MyResult<T> = Result<T, MyError>;

fn main() -> MyResult<()> {
    let mut args = env::args();
    let [Some(_current_exe), Some(command), None] = array::from_fn(|_| args.next()) else {
        return Err("expected a single argument".into());
    };

    match command.as_str() {
        "compress-app-wasm" => compress_app_wasm::run(),
        "generate-app-js" => generate_app_js::run(),
        "minify" => minify::run(),
        "print-sizes" => print_sizes::run(),
        "serve" => serve::run(),
        _ => Err(format!("unknown command: {command}").into()),
    }
}

fn repo_root() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path
}
