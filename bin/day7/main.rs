// Day 7

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug)]
struct Tree<T>
where
    T: PartialEq,
{
    store: Vec<Node<T>>,
}

impl<T> Tree<T>
where
    T: PartialEq,
{
    fn new() -> Self {
        Self { store: vec![] }
    }

    fn empty(&self) -> bool {
        self.store.is_empty()
    }

    fn node(&mut self, val: T) -> usize {
        self.store.push(Node::new(self.store.len(), val));
        self.store.len() - 1
    }

    fn add_child(&mut self, parent: usize, child: usize) {
        self.store[parent].children.push(child);
        self.store[child].parent = Some(parent);
    }

    fn validate(&self) -> bool {
        self.store.iter().all(|n| {
            n.children.iter().all(|c| {
                self.store
                    .get(*c)
                    .map_or(false, |child| child.parent == Some(n.idx))
            })
        })
    }
}

#[derive(Debug)]
enum FileSystemNode {
    Dir(String),
    File(String, usize),
}

impl PartialEq for FileSystemNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FileSystemNode::Dir(name0), FileSystemNode::Dir(name1)) => name0 == name1,
            (FileSystemNode::Dir(_), FileSystemNode::File(_, _)) => false,
            (FileSystemNode::File(_, _), FileSystemNode::Dir(_)) => false,
            (FileSystemNode::File(name0, size0), FileSystemNode::File(name1, size1)) => {
                name0 == name1 && size0 == size1
            }
        }
    }
}

fn load(tree: &mut Tree<FileSystemNode>, lines: &Vec<String>) {
    let mut current: usize = usize::default();
    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        match (tokens.get(0), tokens.get(1), tokens.get(2)) {
            (Some(&"$"), Some(&"cd"), Some(&"..")) => {
                let parent = tree.store[current].parent;
                current = parent.unwrap();
            }
            (Some(&"$"), Some(&"cd"), Some(d)) => {
                if tree.empty() {
                    current = tree.node(FileSystemNode::Dir(d.to_string())); // If no nodes have been added to the tree, it will become root
                } else {
                    let children = tree.store[current].children.clone();
                    current = *children
                        .iter()
                        .find(|&i| match &tree.store[*i].val {
                            FileSystemNode::Dir(name) => d == name,
                            _ => false,
                        })
                        .unwrap();
                }
            }
            (Some(&"$"), Some(&"ls"), _) => (), // We actually don't care about "ls"
            (Some(&"dir"), Some(name), _) => {
                let child = tree.node(FileSystemNode::Dir(name.to_string()));
                tree.add_child(current, child);
            }
            (Some(size), Some(name), _) => {
                let child = tree.node(FileSystemNode::File(
                    name.to_string(),
                    size.parse::<usize>().unwrap(),
                ));
                tree.add_child(current, child);
            }
            (_, _, _) => todo!(),
        }
    }
    assert!(tree.validate());
}

fn is_node_dir(node: &Node<FileSystemNode>) -> bool {
    match node.val {
        FileSystemNode::Dir(_) => true,
        FileSystemNode::File(_, _) => false,
    }
}

fn find_dir_size(tree: &Tree<FileSystemNode>, idx: usize) -> usize {
    let node = &tree.store[idx];
    match node.val {
        FileSystemNode::Dir(_) => node
            .children
            .iter()
            .map(|c| find_dir_size(tree, *c))
            .reduce(|acc, s| acc + s)
            .unwrap(),
        FileSystemNode::File(_, size) => size,
    }
}

fn find_node_with_name<'a>(
    tree: &'a Tree<FileSystemNode>,
    name: &str,
) -> Option<&'a Node<FileSystemNode>> {
    tree.store.iter().find(|n| match &n.val {
        FileSystemNode::Dir(dir_name) => dir_name == name,
        FileSystemNode::File(file_name, _) => file_name == name,
    })
}

fn main() -> io::Result<()> {
    let path = Path::new("./bin/day7/input");
    let file = File::open(path)?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(|r| r.ok())
        .collect();

    let mut tree: Tree<FileSystemNode> = Tree::new();
    load(&mut tree, &lines);

    println!("Part 1");
    let dirs_total_size = tree
        .store
        .iter()
        .filter(|n| is_node_dir(n))
        .map(|n| find_dir_size(&tree, n.idx))
        .filter(|size| *size < 100000)
        .reduce(|acc, e| acc + e)
        .unwrap();
    println!("Total dir size for dirs < 100000: {:?}", dirs_total_size);

    let total_disk_size = 70_000_000;
    let space_required_update = 30_000_000;

    println!("Part 2");
    println!("Total disk size: {}", total_disk_size);
    println!("Space required for update: {}", space_required_update);
    let root = find_node_with_name(&tree, "/").unwrap();
    let root_size = find_dir_size(&tree, root.idx);
    println!("Root dir size: {}", root_size);
    let space_needed =
        (total_disk_size as i32 - space_required_update as i32 - root_size as i32).abs();
    println!("Space needed: {}", space_needed);

    let dir_to_be_deleted = tree
        .store
        .iter()
        .filter(|n| is_node_dir(n))
        .map(|n| find_dir_size(&tree, n.idx))
        .filter(|s| *s > space_needed as usize)
        .reduce(|acc, e| acc.min(e))
        .unwrap();

    println!("Size of dir to be deleted: {}", dir_to_be_deleted);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    const LINES: &str = r#"$ cd /
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

    #[test]
    fn test_find_dir_size() {
        let lines = LINES.lines().map(String::from).collect();

        let mut tree: Tree<FileSystemNode> = Tree::new();
        load(&mut tree, &lines);

        let node_e = find_node_with_name(&tree, "e");
        assert_eq!(find_dir_size(&tree, node_e.unwrap().idx), 584);

        let node_a = find_node_with_name(&tree, "a");
        assert_eq!(find_dir_size(&tree, node_a.unwrap().idx), 94853);

        let node_d = find_node_with_name(&tree, "d");
        assert_eq!(find_dir_size(&tree, node_d.unwrap().idx), 24933642);

        let node_root = find_node_with_name(&tree, "/");
        assert_eq!(find_dir_size(&tree, node_root.unwrap().idx), 48381165);
    }

    #[derive(Debug)]
    struct Fubar {
        value: i32,
    }

    #[test]
    fn testing_rc_fubar() {
        // let h0: Rc<Fubar> = Rc::new(Fubar { value: 42 });
        // let h0 = Rc::new(Fubar { value: 42 });

        let fubar = Fubar { value: 42 };
        let h0 = Rc::new(fubar);

        assert_eq!(h0.value, 42);

        let h1 = h0.clone();
        let h2 = h0.clone();
        assert_eq!(h1.value, 42);
        assert_eq!(h2.value, 42);
    }

    #[test]
    fn testing_rc_refcell_fubar() {
        let fubar = Fubar { value: 42 };

        // let h0: Rc<RefCell<Fubar>> = Rc::new(RefCell::new(Fubar { value: 42 }));
        let h0: Rc<RefCell<Fubar>> = Rc::new(RefCell::new(fubar));
        assert_eq!(h0.borrow().value, 42);

        let h1 = h0.clone();
        let h2 = h0.clone();
        assert_eq!(h1.borrow().value, 42);
        assert_eq!(h2.borrow().value, 42);

        h1.borrow_mut().value = 22;
        assert_eq!(h0.borrow().value, 22);
        assert_eq!(h1.borrow().value, 22);
        assert_eq!(h2.borrow().value, 22);
    }
}
