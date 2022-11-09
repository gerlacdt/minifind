use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn hello_world() {
    println!("Hello world")
}

fn dir_walker<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<PathBuf>> {
    let mut results: Vec<PathBuf> = vec![];
    tree_walk(path, &mut results);
    Ok(results)
}

fn tree_walk<P: AsRef<Path>>(
    path: P,
    results: &mut Vec<PathBuf>,
) -> std::io::Result<&Vec<PathBuf>> {
    for entry in fs::read_dir(path)? {
        let dir = entry?;
        results.push(dir.path());
        if dir.file_type()?.is_dir() {
            tree_walk(dir.path(), results);
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use crate::dir_walker;
    use assert_fs::{prelude::*, TempDir};
    use std::path::PathBuf;

    #[test]
    fn print_project_files() -> std::io::Result<()> {
        // let path = "/home/gerlacdt/src/rust/minifind";
        let path = ".";
        let actual = dir_walker(path)?;

        for entry in actual {
            println!("{}", entry.to_str().unwrap());
        }
        Ok(())
    }

    #[test]
    fn dir_walker_single_file_test() -> std::io::Result<()> {
        let temp = TempDir::new().unwrap();
        let file = temp.child("file.txt");
        file.touch().unwrap();

        let actual = dir_walker(temp.path())?;

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
    fn dir_walker_subdir_test() -> std::io::Result<()> {
        let temp = TempDir::new().unwrap();
        let file = temp.child("file.txt");
        file.touch().unwrap();
        let file1 = temp.child("subdir/file1.txt");
        file1.touch().unwrap();
        let file2 = temp.child("subdir/file2.txt");
        file2.touch().unwrap();

        let actual = dir_walker(temp.path())?;

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
}
