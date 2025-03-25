use std::fs;

use minifier::css;
use minify_js::{Session, TopLevelMode};

use crate::MyResult;

pub fn run() -> MyResult<()> {
    let cfg = minify_html::Cfg::new();
    let root = crate::repo_root();

    for res in fs::read_dir(root.join("dist"))? {
        let entry = res?;

        if !entry.file_type()?.is_file() {
            continue;
        }

        let path = entry.path();
        let Some(extension) = path.extension() else {
            continue;
        };

        let minified = if extension == "html" {
            let src = fs::read(&path)?;
            minify_html::minify(&src, &cfg)
        } else if extension == "css" {
            let src = fs::read_to_string(&path)?;
            css::minify(&src)?.to_string().into_bytes()
        } else if extension == "js" {
            let src = fs::read(&path)?;
            let mut output = vec![];
            minify_js::minify(&Session::new(), TopLevelMode::Module, &src, &mut output)
                .map_err(|e| e.to_string())?;
            output
        } else {
            continue;
        };

        fs::write(&path, minified)?;
    }

    Ok(())
}
