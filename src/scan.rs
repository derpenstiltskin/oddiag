use std::fs;
use std::os::windows::prelude::*;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use anyhow::{Result, bail};
use jwalk::WalkDir;
use csv::Writer;
use chrono::{DateTime, Local};
use super::is_bit_set;
use super::ntfs::*;

#[derive(Debug, Clone)]
pub struct ScanResult {
    path: PathBuf,
    attributes: u32,
    reparse_tag: u32,
    last_modified: SystemTime,
    size: u64,
}

impl ScanResult {
    pub fn new<P: Into<PathBuf>>(path: P, attributes: u32, reparse_tag: u32, last_modified: SystemTime, size: u64) -> ScanResult {
        ScanResult {
            path: path.into(),
            attributes,
            reparse_tag,
            last_modified,
            size,
        }        
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_attributes(&self) -> u32 {
        self.attributes
    }

    pub fn get_reparse_tag(&self) -> u32 {
        self.reparse_tag
    }

    pub fn get_last_modified(&self) -> SystemTime {
        self.last_modified
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }
}

#[derive(Debug, Clone)]
pub struct Scan {
    path: PathBuf,
    files: Vec<ScanResult>,
    size: u64,
}

impl Scan {
    pub fn new<P: Into<PathBuf>>(path: P) -> Scan {
        Scan {
            path: path.into(),
            files: Vec::new(),
            size: 0,
        }
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_files(&self) -> &Vec<ScanResult> {
        &self.files
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_count(&self) -> u64 {
        self.files.len() as u64
    }

    pub fn scan(&mut self) -> Result<()> {
        for contents in WalkDir::new(&self.path) {
            match contents {
                Ok(child) => {
                    if child.file_type.is_file() {
                        let metadata = fs::metadata(&child.path().into_os_string())?;
                        let attributes = metadata.file_attributes();
                        let size = metadata.len();
                        let last_modified = metadata.modified()?;

                        if (is_bit_set!(attributes, NtfsFileAttributes::Pinned as u32) &&
                            !(is_bit_set!(attributes, NtfsFileAttributes::Hidden as u32))) ||
                            (!(is_bit_set!(attributes, NtfsFileAttributes::Pinned as u32)) &&
                            !(is_bit_set!(attributes, NtfsFileAttributes::Unpinned as u32)) &&
                            !(is_bit_set!(attributes, NtfsFileAttributes::RecallOnDataAccess as u32)) &&
                            !(is_bit_set!(attributes, NtfsFileAttributes::Hidden as u32))) {
                            self.files.push(ScanResult::new(&child.path(), attributes, 0, last_modified, size));

                            self.size += size;
                        }
                    }
                }
                Err(error) => {
                    bail!("Read error: {:?}", error);
                }
            }
        }

        Ok(())
    }

    pub fn backup<P: Into<PathBuf> + Copy>(&mut self, path: P) -> Result<()> {
        for file in &self.files {
            let backup_dest_root: PathBuf = path.into();
            let backup_source = &file.get_path().to_str().unwrap().strip_prefix("C:\\Users\\").unwrap();
            let backup_dest_str = format!("{}{}", &backup_dest_root.to_str().unwrap(), &backup_source);
            let backup_dest = Path::new(&backup_dest_str);

            fs::create_dir_all(&backup_dest.parent().unwrap())?;
            fs::copy(&file.get_path().as_path(), &backup_dest)?;
        }

        Ok(())
    }

    pub fn report<P: Into<PathBuf> + Copy>(&mut self, path: P) -> Result<()> {
        let report_path: PathBuf = path.into();
        let report_path_parent = report_path.parent().unwrap();

        fs::create_dir_all(&report_path_parent)?;
        let mut writer = Writer::from_path(path.into())?;
        writer.write_record(&["file", "size_in_bytes", "last_modified"])?;

        for file in &self.files {
            let size = format!("{}", file.get_size());
            let last_modified_dt: DateTime<Local> = file.get_last_modified().clone().into();
            let last_modified = &last_modified_dt.to_string();
            writer.write_record(&[&file.get_path().to_str().unwrap(), &size.as_str(), &last_modified.as_str()])?
        }

        writer.flush()?;

        Ok(())
    }
}