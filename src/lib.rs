use anyhow::{Context, Result};
use regex::Regex;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub mod clap;

pub fn hello_world() {
    println!("Hello world")
}

pub struct Options {
    pub directory: String,
    pub pattern: Option<Regex>,
}

pub fn find(opts: Options) -> Result<()> {
    let pattern = opts.pattern.as_ref();
    let results = dir_walker(opts.directory, pattern).context("Failed directory walking")?;
    output(results);
    Ok(())
}

fn output(filenames: Vec<PathBuf>) {
    for f in filenames {
        println!("{}", f.to_str().unwrap());
    }
}

fn dir_walker<P: AsRef<Path>>(path: P, pattern: Option<&Regex>) -> Result<Vec<PathBuf>> {
    let mut results: Vec<PathBuf> = vec![];
    tree_walk(path, &mut results, pattern).context("Failed tree_walk()")?;
    Ok(results)
}

fn tree_walk<'a, P: AsRef<Path>>(
    path: P,
    results: &'a mut Vec<PathBuf>,
    pattern: Option<&Regex>,
) -> Result<&'a Vec<PathBuf>> {
    for entry in fs::read_dir(path)? {
        let dir = entry.context("Failed to extract directory")?;
        if let Some(re) = pattern {
            if re.is_match(dir.path().to_str().unwrap()) {
                results.push(dir.path());
            }
        } else {
            results.push(dir.path());
        }
        if dir
            .file_type()
            .context("Failed to extraced file type")?
            .is_dir()
        {
            tree_walk(dir.path(), results, pattern)?;
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use crate::dir_walker;
    use anyhow::{Context, Result};
    use assert_fs::{prelude::*, TempDir};
    use regex::Regex;

    #[test]
    fn print_project_files() -> Result<()> {
        let path = ".";
        let pattern = None;
        let actual = dir_walker(path, pattern).context("Failed dir_walker()")?;

        for entry in actual {
            println!("{}", entry.to_str().unwrap());
        }
        Ok(())
    }

    #[test]
    fn dir_walker_single_file_test() -> Result<()> {
        let temp = TempDir::new().unwrap();
        let file = temp.child("file.txt");
        file.touch().unwrap();
        let pattern = None;
        let actual = dir_walker(temp.path(), pattern).context("Failed dir_walker()")?;

        println!("{:?}", actual);

        let expected = 1;
        assert_eq!(expected, actual.len());

        let expected_filename = temp.join("file.txt");
        assert_eq!(
            expected_filename.to_str().unwrap(),
            actual.get(0).unwrap().to_str().unwrap()
        );

        Ok(())
    }

    #[test]
    fn dir_walker_subdir_test() -> Result<()> {
        let temp = TempDir::new().unwrap();
        let file = temp.child("file.txt");
        file.touch().unwrap();
        let file1 = temp.child("subdir/file1.txt");
        file1.touch().unwrap();
        let file2 = temp.child("subdir/file2.txt");
        file2.touch().unwrap();
        let pattern = None;
        let actual = dir_walker(temp.path(), pattern).context("Failed dir_walker()")?;

        println!("{:?}", actual);

        let expected = 4;
        assert_eq!(expected, actual.len());

        let expected = vec![
            temp.join("file.txt"),
            temp.join("subdir"),
            temp.join("subdir/file1.txt"),
            temp.join("subdir/file2.txt"),
        ];
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn dir_walker_subdir_pattern_test() -> Result<()> {
        let temp = TempDir::new().unwrap();
        let file = temp.child("file.txt");
        file.touch().unwrap();
        let file1 = temp.child("subdir/file1.txt");
        file1.touch().unwrap();
        let file2 = temp.child("subdir/file2.txt");
        file2.touch().unwrap();
        let pattern = Regex::new(r#"file1.*"#).unwrap();
        let actual = dir_walker(temp.path(), Some(&pattern)).context("Failed dir_walker()")?;

        println!("{:?}", actual);

        let expected = 1;
        assert_eq!(expected, actual.len());

        let expected = vec![temp.join("subdir/file1.txt")];
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn dir_walker_subdir_pattern_multimatchtest() -> Result<()> {
        let temp = TempDir::new().unwrap();
        let file = temp.child("file.txt");
        file.touch().unwrap();
        let file1 = temp.child("subdir/file1.txt");
        file1.touch().unwrap();
        let file2 = temp.child("subdir/file2.txt");
        file2.touch().unwrap();
        let pattern = Regex::new(r#"file"#).unwrap();
        let actual = dir_walker(temp.path(), Some(&pattern)).context("Failed dir_walker()")?;

        println!("{:?}", actual);

        let expected = 3;
        assert_eq!(expected, actual.len());

        let expected = vec![
            temp.join("file.txt"),
            temp.join("subdir/file1.txt"),
            temp.join("subdir/file2.txt"),
        ];
        assert_eq!(expected, actual);

        Ok(())
    }
}
