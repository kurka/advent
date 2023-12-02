use std::{
    cell::RefCell,
    cmp,
    collections::HashMap,
    fs,
    rc::{Rc, Weak},
    usize,
};

#[derive(Clone, Debug, Default)]
struct Folder {
    file_sizes: usize,
    subdirs: HashMap<String, Rc<RefCell<Folder>>>, // Vec<Rc<RefCell<Folder>>>,
    parent: Weak<RefCell<Folder>>,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input07.in").unwrap());
    println!("Day 7:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input))
}

fn parse_input(input: String) -> Rc<RefCell<Folder>> {
    let root: Rc<RefCell<Folder>> = Rc::new(RefCell::new(Folder::default()));
    let mut cur_line = 0;
    let mut cur_dir: Rc<RefCell<Folder>> = Rc::clone(&root);
    let lines: Vec<&str> = input.lines().skip(1).collect();
    let n_lines = lines.len();

    while cur_line < n_lines {
        let line = lines[cur_line];
        let (command, arg) = line.split_at(4);
        match command {
            "$ ls" => {
                while cur_line < n_lines - 1 {
                    let next_line = lines[cur_line + 1];
                    if next_line.starts_with("$") {
                        break;
                    }
                    let (file_info, file_name) = next_line.split_once(" ").unwrap();
                    match file_info {
                        "dir" => {
                            _ = cur_dir.borrow_mut().subdirs.insert(
                                String::from(file_name),
                                Rc::new(RefCell::new(Folder {
                                    file_sizes: 0,
                                    subdirs: HashMap::new(),
                                    parent: Rc::downgrade(&cur_dir),
                                })),
                            )
                        }
                        _ => cur_dir.borrow_mut().file_sizes += file_info.parse::<usize>().unwrap(),
                    }
                    cur_line += 1;
                }
            }
            "$ cd" => {
                let next_dir_name = arg.trim();
                cur_dir = match next_dir_name {
                    ".." => Rc::clone(&cur_dir.borrow().parent.upgrade().unwrap()),
                    _ => Rc::clone(&cur_dir.borrow().subdirs.get(next_dir_name).unwrap()),
                }
            }
            _ => panic!("Invalid input: {line}"),
        }
        cur_line += 1
    }

    root
}

fn sum_dirs_and_find_smalls(tree: &Folder) -> (usize, usize) {
    let mut subdir_sizes = 0;
    let mut acc_size = 0;
    for (subdir_size, subdir_acc_size) in tree
        .subdirs
        .values()
        .map(|f| sum_dirs_and_find_smalls(&f.borrow()))
    {
        subdir_sizes += subdir_size;
        acc_size += subdir_acc_size;
    }

    let dir_size = tree.file_sizes + subdir_sizes;
    if dir_size <= 100_000 {
        acc_size += dir_size
    }

    (dir_size, acc_size)
}

fn sum_dirs_and_find_folder_to_delete(tree: &Folder, target: usize) -> (usize, usize) {
    let mut subdir_sizes = 0;
    let mut acc_size = usize::MAX;
    for (subdir_size, subdir_acc_size) in tree
        .subdirs
        .values()
        .map(|f| sum_dirs_and_find_folder_to_delete(&f.borrow(), target))
    {
        subdir_sizes += subdir_size;
        acc_size = cmp::min(acc_size, subdir_acc_size);
    }

    let dir_size = tree.file_sizes + subdir_sizes;
    if dir_size >= target {
        acc_size = cmp::min(acc_size, dir_size);
    }

    (dir_size, acc_size)
}

fn solve_part_a(root: &Rc<RefCell<Folder>>) -> usize {
    sum_dirs_and_find_smalls(&root.borrow()).1
}

fn solve_part_b(root: &Rc<RefCell<Folder>>) -> usize {
    let total_size = sum_dirs_and_find_smalls(&root.borrow()).0;
    let necessary_space = 30_000_000 - (70_000_000 - total_size);
    sum_dirs_and_find_folder_to_delete(&root.borrow(), necessary_space).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

        let input = parse_input(sample.to_string());

        assert_eq!(95437, solve_part_a(&input));
        assert_eq!(24933642, solve_part_b(&input));
    }
}
