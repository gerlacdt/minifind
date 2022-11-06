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
    use crate::{dir_walker, tree_walk};

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
}
