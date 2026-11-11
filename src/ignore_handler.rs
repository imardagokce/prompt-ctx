use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use ignore::WalkBuilder;

/// Finds readable UTF-8 text files, honoring ignore files and excluding hidden
/// paths and common binary/media formats.
pub fn find_text_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in WalkBuilder::new(root)
        .hidden(true)
        .standard_filters(true)
        .build()
    {
        let entry = entry.with_context(|| format!("failed to walk {}", root.display()))?;
        if !entry.file_type().is_some_and(|kind| kind.is_file()) {
            continue;
        }
        let path = entry.into_path();
        if is_known_binary_or_media(&path) {
            continue;
        }
        let bytes = fs::read(&path).with_context(|| format!("failed to read {}", path.display()))?;
        if !bytes.contains(&0) && std::str::from_utf8(&bytes).is_ok() {
            files.push(path);
        }
    }
    Ok(files)
}

fn is_known_binary_or_media(path: &Path) -> bool {
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or_default();
    matches!(
        extension.to_ascii_lowercase().as_str(),
        "7z" | "a" | "avi" | "bin" | "bmp" | "class" | "dll" | "dylib" | "eot"
            | "exe" | "flac" | "gif" | "gz" | "ico" | "jar" | "jpeg" | "jpg"
            | "mp3" | "mp4" | "ogg" | "otf" | "pdf" | "png" | "so" | "sqlite"
            | "tar" | "tif" | "tiff" | "ttf" | "wav" | "webp" | "woff" | "woff2"
            | "zip"
    )
}
