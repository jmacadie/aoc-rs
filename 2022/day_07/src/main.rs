use itertools::Itertools;
use std::collections::HashMap;

pub fn main() {
    let data = include_str!("input.txt");
    let fs = prelude(data);
    println!("Part 1: {}", part_one(&fs));
    println!("Part 2: {}", part_two(&fs));
}

pub fn bench() {
    let data = include_str!("input.txt");
    let fs = prelude(data);
    let _ = part_one(&fs);
    let _ = part_two(&fs);
}

fn prelude(data: &'static str) -> FileSystem {
    let mut fs = create_fs(data);
    compute_folder_sizes(&mut fs);
    fs
}

fn part_one(fs: &FileSystem) -> u64 {
    fs.iter()
        .filter(|(_, dir)| dir.size < 100_000)
        .map(|(_, dir)| dir.size)
        .sum()
}

fn part_two(fs: &FileSystem) -> u64 {
    const TOTAL_SPACE: u64 = 70_000_000;
    const TARGET_SPACE: u64 = 30_000_000;
    let mut sorted_folders = fs
        .iter()
        .map(|(k, v)| (k, v.size))
        .sorted_by_key(|&(_, b)| std::cmp::Reverse(b));
    let free_space = TOTAL_SPACE - sorted_folders.next().unwrap().1;
    let target = TARGET_SPACE - free_space;
    sorted_folders
        .filter(|(_, b)| b > &target)
        .sorted_by_key(|&(_, b)| b)
        .next()
        .unwrap()
        .1
}

#[derive(Debug)]
enum TermLine {
    FSElem(FSElem),
    Command(Command),
}

#[allow(dead_code)]
#[derive(Debug)]
enum FSElem {
    Dir(&'static str),
    File { name: &'static str, size: u64 },
}

#[derive(Debug)]
enum Command {
    CD { target: &'static str },
    LS,
}

#[derive(Debug)]
struct Directory {
    size: u64,
    children: Vec<FSElem>,
}

type FileSystem = HashMap<String, Directory>;

fn compute_folder_sizes(fs: &mut FileSystem) {
    let mut known = HashMap::new();
    let size = folder_size(fs, &mut known, "/");
    known.insert("/".to_owned(), size);
    for (folder, size) in known {
        if let Some(dir) = fs.get_mut(&folder) {
            dir.size = size;
        }
    }
}

fn folder_size(fs: &FileSystem, known: &mut HashMap<String, u64>, folder: &str) -> u64 {
    let mut total = 0_u64;
    if let Some(dir) = fs.get(folder) {
        for elem in dir.children.iter() {
            match &elem {
                FSElem::Dir(name) => {
                    let sub_folder = comp_folder(name, folder);
                    match known.get(&sub_folder) {
                        Some(size) => {
                            total += size;
                        }
                        None => {
                            let size = folder_size(fs, known, &sub_folder);
                            known.insert(sub_folder, size);
                            total += size;
                        }
                    };
                }
                FSElem::File { size, .. } => {
                    total += size;
                }
            };
        }
    }
    total
}

fn get_parent(folder: &str) -> Option<String> {
    folder.rfind('/').map(|i| folder.split_at(i).0.to_owned())
}

fn create_fs(data: &'static str) -> FileSystem {
    let mut curr_folder = "/".to_owned();
    let mut fs = HashMap::new();
    add_new_folder(&mut fs, &curr_folder);
    for line in data.lines().map(read_line) {
        match line {
            TermLine::Command(c) => {
                if let Command::CD { target } = c {
                    if target == ".." {
                        if let Some(folder) = get_parent(&curr_folder) {
                            curr_folder = folder;
                        }
                    } else if target == "/" {
                        curr_folder = target.to_owned();
                    } else if curr_folder == "/" {
                        curr_folder = format!("/{target}");
                    } else {
                        curr_folder = format!("{curr_folder}/{target}");
                    }
                };
            }
            TermLine::FSElem(e) => {
                if let FSElem::Dir(name) = e {
                    let folder = comp_folder(name, &curr_folder);
                    if !fs.contains_key(&folder) {
                        add_new_folder(&mut fs, &folder);
                    };
                };
                if let Some(dir) = fs.get_mut(&curr_folder) {
                    dir.children.push(e);
                }
            }
        }
    }
    fs
}

fn comp_folder(folder: &str, parent: &str) -> String {
    if parent.is_empty() {
        return "/".to_owned();
    }
    if parent == "/" {
        let mut out = "/".to_owned();
        out.push_str(folder);
        return out;
    }
    let mut comp_folder = parent.to_owned();
    comp_folder.push('/');
    comp_folder.push_str(folder);
    comp_folder
}

fn add_new_folder(fs: &mut FileSystem, folder: &str) {
    fs.insert(
        folder.to_owned(),
        Directory {
            size: 0,
            children: Vec::new(),
        },
    );
}

fn read_line(line: &'static str) -> TermLine {
    let mut parts = line.split(' ');
    match parts.next() {
        Some("$") => match parts.next() {
            Some("ls") => TermLine::Command(Command::LS),
            Some("cd") => TermLine::Command(Command::CD {
                target: parts.next().unwrap(),
            }),
            _ => unreachable!(),
        },
        Some("dir") => TermLine::FSElem(FSElem::Dir(parts.next().unwrap())),
        Some(file_size) => TermLine::FSElem(FSElem::File {
            name: parts.next().unwrap(),
            size: file_size.parse().unwrap(),
        }),
        None => unreachable!(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let fs = prelude(data);
        assert_eq!(95_437, part_one(&fs));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let fs = prelude(data);
        assert_eq!(24_933_642, part_two(&fs));
    }
}
