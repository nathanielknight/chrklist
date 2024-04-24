use crate::ChecklistError;
use platform_dirs::AppDirs;
use std::{env, fs, io::ErrorKind, path::PathBuf};

pub fn checklist_dir() -> Result<PathBuf, ChecklistError> {
    AppDirs::new(Some(env!("CARGO_PKG_NAME")), true)
        .map(|d| d.data_dir)
        .ok_or(ChecklistError {
            msg: "Couldn't get app data dir".to_owned(),
        })
}

pub fn get_checklists() -> Result<Vec<String>, ChecklistError> {
    let dir = checklist_dir()?;
    let entries = match fs::read_dir(dir.clone()) {
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Err(ChecklistError {
                    msg: format!("Create a checklists directory at {}", dir.to_string_lossy()),
                });
            } else {
                return Err(ChecklistError::from(e));
            }
        }
        Ok(entries) => entries,
    };
    let mut checklists: Vec<String> = Vec::new();
    for direntry in entries.flatten() {
        if direntry.file_type()?.is_file() {
            checklists.push(
                direntry
                    .file_name()
                    .into_string()
                    .map_err(|e| ChecklistError {
                        msg: format!("Error converting filename to string: {:?}", e),
                    })?,
            );
        }
    }
    Ok(checklists)
}

pub fn get_checklist(name: &str) -> Result<Vec<String>, ChecklistError> {
    let mut dir = checklist_dir()?;
    dir.push(name);
    let raw_contents = fs::read(dir.as_path()).map_err(ChecklistError::from)?;
    let contents = String::from_utf8_lossy(&raw_contents);
    let lines = contents
        .split('\n')
        .map(|l| l.trim().to_owned())
        .filter(|s| !s.is_empty())
        .collect();
    Ok(lines)
}
