use std::env;
use std::fs;
use std::path::PathBuf;

pub fn copy_to_temp(original: &PathBuf, name: &str) -> Result<PathBuf, String> {
    let mut temp = env::temp_dir();
    temp.push(name);

    fs::copy(original, &temp)
        .map_err(|e| e.to_string())?;

    Ok(temp)
}
