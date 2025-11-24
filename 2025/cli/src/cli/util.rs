use std::{fs::File, io::Write, path::Path};

pub(crate) fn save_to_file(content: String, path: &Path) -> Result<(), String> {
    let mut file = File::create(path).map_err(|e| e.to_string())?;
    write!(&mut file, "{}", content).map_err(|e| e.to_string())?;
    Ok(())
}
