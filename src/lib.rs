use anyhow::{Context, Result};
use regex::Regex;
use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

pub mod clap;

pub struct Options {
    pub directory: String,
    pub pattern: Option<Regex>,
    pub filetype: Option<String>,
}

pub fn find(opts: Options) -> Result<()> {
    let pattern = opts.pattern.as_ref();
    let filetype = opts.filetype.as_deref();
    let results =
        dir_walker(opts.directory, pattern, filetype).context("Failed directory walking")?;
    output(results);
    Ok(())
}

fn output(filenames: Vec<PathBuf>) {
    for f in filenames {
        println!("{}", f.to_str().unwrap());
    }
}

fn dir_walker<P: AsRef<Path>>(
    path: P,
    pattern: Option<&Regex>,
    filetype: Option<&str>,
) -> Result<Vec<PathBuf>> {
    let mut results: Vec<PathBuf> = vec![];
    tree_walk(path, &mut results, pattern, filetype).context("Failed tree_walk()")?;
    Ok(results)
}

fn tree_walk<'a, P: AsRef<Path>>(
    path: P,
    results: &'a mut Vec<PathBuf>,
    pattern: Option<&Regex>,
    filetype: Option<&str>,
) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let dir = entry.context("Failed to extract directory")?;
        if is_ok(&dir, pattern, filetype) {
            results.push(dir.path())
        }
        if dir
            .file_type()
            .context("Failed to extraced file type")?
            .is_dir()
        {
            tree_walk(dir.path(), results, pattern, filetype)?;
        }
    }

    Ok(())
}

fn is_ok(dir: &DirEntry, pattern: Option<&Regex>, filetype: Option<&str>) -> bool {
    if let Some(re) = pattern {
        if !re.is_match(dir.path().to_str().unwrap()) {
            return false;
        }
    }
    if let Some(ft) = filetype {
        if !((dir.metadata().unwrap().is_dir() && ft == "d")
            || (dir.metadata().unwrap().is_file() && ft == "f"))
        {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::dir_walker;
    use anyhow::{Context, Result};
    use assert_fs::{prelude::*, TempDir};
    use regex::Regex;
    use std::path::PathBuf;

    fn setup_fs() -> TempDir {
        let temp = TempDir::new().unwrap();
        let file = temp.child("file.txt");
        file.touch().unwrap();
        let file1 = temp.child("subdir/file1.txt");
        file1.touch().unwrap();
        let file2 = temp.child("subdir/file2.txt");
        file2.touch().unwrap();
        temp
    }

    #[test]
    fn find_test() -> Result<()> {
        let temp_dir = setup_fs();
        let pattern = None;
        let filetype = None;
        let actual =
            dir_walker(temp_dir.path(), pattern, filetype).context("Failed dir_walker()")?;

        println!("{:?}", actual);

        let expected = 4;
        assert_eq!(expected, actual.len());

        let expected = vec![
            temp_dir.join("file.txt"),
            temp_dir.join("subdir"),
            temp_dir.join("subdir/file1.txt"),
            temp_dir.join("subdir/file2.txt"),
        ];
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn find_with_pattern_test() -> Result<()> {
        let temp_dir = setup_fs();
        let pattern = Regex::new(r#"file1.*"#).unwrap();
        let filetype = None;
        let actual =
            dir_walker(temp_dir.path(), Some(&pattern), filetype).context("Failed dir_walker()")?;

        println!("{:?}", actual);

        let expected = 1;
        assert_eq!(expected, actual.len());

        let expected = vec![temp_dir.join("subdir/file1.txt")];
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn find_with_pattern_multimatch_test() -> Result<()> {
        let temp_dir = setup_fs();
        let pattern = Regex::new(r#"file"#).unwrap();
        let filetype = None;
        let actual =
            dir_walker(temp_dir.path(), Some(&pattern), filetype).context("Failed dir_walker()")?;

        println!("{:?}", actual);

        let expected = 3;
        assert_eq!(expected, actual.len());

        let expected = vec![
            temp_dir.join("file.txt"),
            temp_dir.join("subdir/file1.txt"),
            temp_dir.join("subdir/file2.txt"),
        ];
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn find_with_filetype_test() -> Result<()> {
        let temp_dir = setup_fs();
        let filetype = Some("d");
        let actual = dir_walker(temp_dir.path(), None, filetype).context("Failed dir_walker()")?;

        println!("{:?}", actual);

        let expected = 1;
        assert_eq!(expected, actual.len());

        let expected = vec![temp_dir.join("subdir")];
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn find_with_pattern_and_filetype_test() -> Result<()> {
        let temp_dir = setup_fs();
        let pattern = Regex::new(r#"file.txt"#).unwrap();
        let filetype = Some("f");
        let actual =
            dir_walker(temp_dir.path(), Some(&pattern), filetype).context("Failed dir_walker()")?;

        println!("{:?}", actual);

        let expected = 1;
        assert_eq!(expected, actual.len());

        let expected = vec![temp_dir.join("file.txt")];
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn find_with_pattern_and_filetype_nomatch_test() -> Result<()> {
        let temp_dir = setup_fs();
        let pattern = Regex::new(r#"file.txt"#).unwrap();
        let filetype = Some("d");
        let actual =
            dir_walker(temp_dir.path(), Some(&pattern), filetype).context("Failed dir_walker()")?;

        println!("{:?}", actual);

        let expected = 0;
        assert_eq!(expected, actual.len());

        let expected: Vec<PathBuf> = vec![];
        assert_eq!(expected, actual);

        Ok(())
    }
}
