//!day_07.rs

use anyhow::Result;
use my_lib::my_tree::TreeNode;
use std::rc::Rc;

#[derive(PartialEq, Default)]
struct File {
    name: String,
    size: usize,
}

impl From<&str> for File {
    fn from(value: &str) -> Self {
        let (size, name) = value
            .split_once(' ')
            .map(|(s, n)| (s.parse::<usize>().expect("bad input"), n.to_string()))
            .unwrap();
        Self { name, size }
    }
}

#[derive(PartialEq, Default)]
struct Directrory {
    name: String,
    files: Vec<File>,
    total_size: usize,
}

impl Directrory {
    fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
    fn calc_total_size(&mut self, size_of_sub_dirs: usize) {
        self.total_size = size_of_sub_dirs + self.files.iter().map(|f| f.size).sum::<usize>();
    }
}

struct FileTree {
    tree_root: Rc<TreeNode<Directrory>>,
    children_capacity: usize,
    filesystem_size: usize,
}

impl FileTree {
    fn new(children_capacity: usize, filesystem_size: usize) -> Self {
        let root = Directrory::new("/".into());
        Self {
            tree_root: TreeNode::seed_root(root, children_capacity),
            children_capacity,
            filesystem_size,
        }
    }
    fn parse(&self, input: &str) {
        let mut current_dir = self.tree_root.clone();
        for line in input.lines().filter(|l| !(*l == "$ cd /" || *l == "$ ls")) {
            if line.starts_with("dir") {
                let dir_name = line.split_once(' ').unwrap().1.to_string();
                let new_dir = Directrory::new(dir_name);
                current_dir.add_child(new_dir, self.children_capacity);
            } else if line == "$ cd .." {
                current_dir = current_dir.get_parent().unwrap();
            } else if line.starts_with("$ cd ") {
                let dir_name = line.split_once("$ cd ").unwrap().1.to_string();
                current_dir = current_dir
                    .iter_children()
                    .find(|c| c.get_value().name == dir_name)
                    .unwrap();
            } else {
                let file = File::from(line);
                current_dir.get_mut_value().add_file(file);
            }
        }
    }
    fn calc_dir_sizes(&self) {
        for node in self.tree_root.iter_post_order_traversal() {
            let size_of_sub_dirs = node
                .iter_children()
                .map(|c| c.get_value().total_size)
                .sum::<usize>();
            node.get_mut_value().calc_total_size(size_of_sub_dirs);
        }
    }
    fn sum_dir_size_bounded(&self, upper_bound: usize) -> usize {
        self.tree_root
            .iter_post_order_traversal()
            .filter(|c| c.get_value().total_size <= upper_bound)
            .map(|c| c.get_value().total_size)
            .sum::<usize>()
    }
    fn find_smallest_dir_to_delete(&self, min_required_free_size: usize) -> usize {
        assert!(self.filesystem_size >= self.tree_root.get_value().total_size);
        let current_free_size = self.filesystem_size - self.tree_root.get_value().total_size;
        assert!(current_free_size < min_required_free_size);
        let min_delete_size = min_required_free_size - current_free_size;
        self.tree_root
            .iter_post_order_traversal()
            .filter(|d| d.get_value().total_size >= min_delete_size)
            .map(|d| d.get_value().total_size)
            .min()
            .unwrap()
    }
}

pub fn day_07() -> Result<()> {
    let input = include_str!("../../assets/day_07.txt");
    let file_tree = FileTree::new(5, 70_000_000);
    file_tree.parse(input);
    file_tree.calc_dir_sizes();
    let result_part1 = file_tree.sum_dir_size_bounded(100_000);
    println!("result day 07 part 1: {}", result_part1);
    assert_eq!(result_part1, 1_644_735);
    let result_part2 = file_tree.find_smallest_dir_to_delete(30_000_000);
    println!("result day 07 part 2: {}", result_part2);
    assert_eq!(result_part2, 1_300_850);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = "$ cd /\n\
                           $ ls\n\
                           dir a\n\
                           14848514 b.txt\n\
                           8504156 c.dat\n\
                           dir d\n\
                           $ cd a\n\
                           $ ls\n\
                           dir e\n\
                           29116 f\n\
                           2557 g\n\
                           62596 h.lst\n\
                           $ cd e\n\
                           $ ls\n\
                           584 i\n\
                           $ cd ..\n\
                           $ cd ..\n\
                           $ cd d\n\
                           $ ls\n\
                           4060174 j\n\
                           8033020 d.log\n\
                           5626152 d.ext\n\
                           7214296 k";
        let file_tree = FileTree::new(5, 70_000_000);
        file_tree.parse(input);
        file_tree.calc_dir_sizes();
        let result_part1 = file_tree.sum_dir_size_bounded(100_000);
        println!("result example day 07 part 1: {}", result_part1);
        assert_eq!(result_part1, 95_437);
        let result_part2 = file_tree.find_smallest_dir_to_delete(30_000_000);
        println!("result example day 07 part 2: {}", result_part2);
        assert_eq!(result_part2, 24_933_642);
        Ok(())
    }
}
