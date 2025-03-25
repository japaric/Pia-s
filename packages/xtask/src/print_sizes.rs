use std::collections::HashMap;
use std::io::Write as _;
use std::{fs, io};

use num_format::{Buffer, Locale};

use crate::MyResult;

pub fn run() -> MyResult<()> {
    let mut totals = HashMap::<_, u64>::new();
    for res in fs::read_dir(crate::repo_root().join("dist"))? {
        let entry = res?;
        let path = entry.path();

        let Some(extension) = path.extension() else {
            continue;
        };

        *totals.entry(extension.to_owned()).or_default() += entry.metadata()?.len();
    }

    let mut sorted: Vec<_> = totals.into_iter().collect();
    sorted.sort_by_key(|(_extension, total)| *total);

    let stderr = io::stderr();
    let mut stderr = stderr.lock();
    writeln!(stderr, "File\tSize (B)    Pct (%)")?;
    let total: u64 = sorted.iter().map(|(_extension, subtotal)| subtotal).sum();
    let locale = Locale::en;
    for (extension, subtotal) in sorted.into_iter().rev() {
        let mut buf = Buffer::default();
        buf.write_formatted(&subtotal, &locale);
        let pct = 100. * subtotal as f64 / total as f64;

        writeln!(
            stderr,
            "*.{}\t{:>8}    {:>7.02}",
            extension.to_string_lossy(),
            buf.as_str(),
            pct
        )?;
    }
    let mut buf = Buffer::default();
    buf.write_formatted(&total, &locale);

    writeln!(stderr, "Total\t{:>8}", buf.as_str())?;

    Ok(())
}
