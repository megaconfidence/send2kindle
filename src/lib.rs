use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

pub fn compress_pdf(in_path: &Path) -> Vec<u8> {
    let in_path_str = in_path.to_string_lossy().to_string();
    let out_path_str = format!("compressed_{}", in_path_str);
    let out_path = Path::new(&out_path_str);

    Command::new("gs")
        .args(
            [
                "-dQUIET",
                "-dBATCH",
                "-dNOPAUSE",
                "-sDEVICE=pdfwrite",
                "-dCompatibilityLevel=1.4",
                "-dPDFSETTINGS=/ebook",
            ]
            .iter(),
        )
        .arg(format!("-sOutputFile={}", out_path_str))
        .arg(in_path_str)
        .output()
        .expect("failed to compress pdf");

    fs::read(out_path).expect("error reading output pdf")
}

pub fn clean_files(file_id: &String) -> Result<()> {
    let re = Regex::new(file_id).unwrap();
    for entry in WalkDir::new(".") {
        let entry = entry?;
        if re.is_match(entry.path().to_str().unwrap()) {
            tracing::info!("cleaning {}", entry.path().display());
            fs::remove_file(entry.path())?;
        }
    }
    Ok(())
}
