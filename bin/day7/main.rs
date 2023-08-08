// Day 7

use std::{path::Path, fs::File, io::{BufReader, BufRead, self}};

#[derive(Debug)]
struct Node<T> 
where T: PartialEq {
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T> 
where T: PartialEq {
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
where T: PartialEq {
    store: Vec<Node<T>>,
}

impl<T> Tree<T> 
where T: PartialEq {
    fn new() -> Self {
        Self {
            store: vec![],
        }
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
        self.store.iter().all(|n| n.children.iter().all(|c| {
            self.store.get(*c).map_or(false, |child| child.parent == Some(n.idx))
        }))
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
            (FileSystemNode::File(name0, size0), FileSystemNode::File(name1, size1)) => name0 == name1 && size0 == size1,
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
            },
            (Some(&"$"), Some(&"cd"), Some(d)) => {
                if tree.empty() {
                    current = tree.node(FileSystemNode::Dir(d.to_string())); // If no nodes have been added to the tree, it will become root
                } else {
                    let children = tree.store[current].children.clone();
                    current = *children.iter().find(|&i| {
                        match &tree.store[*i].val {
                            FileSystemNode::Dir(name) => d == name, 
                            _ => false,
                        }
                    }).unwrap();
                }
            },
            (Some(&"$"), Some(&"ls"), _) => (), // We actually don't care about "ls"
            (Some(&"dir"), Some(name), _) => {
                let child = tree.node(FileSystemNode::Dir(name.to_string()));
                tree.add_child(current, child);
            },
            (Some(size), Some(name), _) => {
                let child = tree.node(FileSystemNode::File(name.to_string(), size.parse::<usize>().unwrap()));
                tree.add_child(current, child);
            },
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
        FileSystemNode::Dir(_) => node.children.iter().map(|c| {
            find_dir_size(tree, *c)
        }).reduce(|acc, s| acc + s).unwrap(),
        FileSystemNode::File(_, size) => size,
    }
}

fn find_node_with_name<'a>(tree: &'a Tree<FileSystemNode>, name: &str) -> Option<&'a Node<FileSystemNode>> {
    tree.store.iter().find(|n| {
        match &n.val {
            FileSystemNode::Dir(dir_name) => dir_name == name,
            FileSystemNode::File(file_name, _) => file_name == name,
        }
    })
}

fn main() -> io::Result<()> {
    let path = Path::new("./bin/day7/input");
    let file = File::open(path)?;
    let lines: Vec<String>  = BufReader::new(file).lines().filter_map(|r| r.ok()).collect();

    let mut tree: Tree<FileSystemNode> = Tree::new();
    load(&mut tree, &lines);

    println!("Part 1");
    let dirs_total_size = tree.store.iter()
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
    let space_needed = (total_disk_size as i32 - space_required_update as i32 - root_size as i32).abs();
    println!("Space needed: {}", space_needed);

    let dir_to_be_deleted = tree.store.iter()
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
    use std::{rc::Rc, cell::RefCell};

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

// #[derive(Debug)]
// enum TreeNode {
//     Dir {
//         name: String,
//         children: Vec<Rc<RefCell<TreeNode>>>,
//         parent: Option<Rc<RefCell<TreeNode>>>,
//     },
//     File {
//         name: String,
//         size: u32,
//         parent: Option<Rc<RefCell<TreeNode>>>,
//     }
// }
//
// impl TreeNode {
//     fn add_child(&mut self, child: Rc<RefCell<TreeNode>>) {
//         if let TreeNode::Dir { children, .. } = self {
//             children.push(child.clone());
//         }
//     }
//
//     // fn add_file(&mut self, name: &str, size: u32) {
//     //     if let TreeNode::Dir { children, .. } = self {
//     //         let file_node = TreeNode::File { name: name.to_string(), size, parent: None };
//     //         children.push(Rc::new(RefCell::new(file_node)));
//     //     }
//     // }
// }
//

// #[derive(Debug)]
// struct TreeNode {
//     value: Option<String>,
//     children: Vec<Rc<RefCell<TreeNode>>>,
//     parent: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     fn new(value: Option<String>, children: Vec<Rc<RefCell<TreeNode>>>, parent: Option<Rc<RefCell<TreeNode>>>) -> Self {
//         Self { value, children, parent }
//     }
// }




// use std::{cell::RefCell, rc::Rc};

// struct State {
//     current: Option<Rc<RefCell<Node>>>,
//     root: Rc<RefCell<Node>>,
// }
//
// impl State {
//     fn new(root: Node) -> Self {
//         let root = Rc::new(RefCell::new(root));
//         State {
//             current: Some(root.clone()),
//             root,
//         }
//     }

// $ cd /
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// $ cd a
// $ ls
// dir e
// 29116 f
// 2557 g
// 62596 h.lst
// $ cd e
// $ ls
// 584 i
// $ cd ..
// $ cd ..
// $ cd d
// $ ls
// 4060174 j
// 8033020 d.log
// 5626152 d.ext
// 7214296 k
    // fn load(&mut self, lines: &Vec<String>) {
    //     for line in lines {
    //         let tokens: Vec<&str> = line.split_whitespace().collect();
    //         if tokens.len() == 0 {
    //             continue;
    //         }
    //
    //         if tokens[0] == "$" {
    //             match tokens[1] {
    //                 "cd" => {
    //                     // self.current.unwrap().borrow()
    //                     ()
    //                 },
    //                 _ => (),
    //             }
    //             
    //
    //
    //
    //         }
    //
    //
    //
    //     }
    // }

    // fn run_cmds(&mut self, cmds: &Vec<String>)  {
    //     for cmd in cmds {
    //         let tokens: Vec<&str> = cmd.split_whitespace().collect();
    //         if tokens.len() == 0 || tokens[0] != "$" {
    //             continue;
    //         }
    //
    //         match tokens[1] {
    //             "cd" => println!("cd {}", tokens[2]),
    //             "ls" => println!("ls"),
    //             _ => ()
    //         }
    //     }
    // }
// }



// #[derive(Debug)]
// enum Node {
//     Dir(Option<Rc<RefCell<Node>>>, String, Vec<Rc<RefCell<Node>>>),
//     File(Option<Rc<RefCell<Node>>>, String, u32),
// }
//
// fn main() {
//     let lines: Vec<String> = r#"
// $ cd /
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// $ cd a
// $ ls
// dir e
// 29116 f
// 2557 g
// 62596 h.lst
// $ cd e
// $ ls
// 584 i
// $ cd ..
// $ cd ..
// $ cd d
// $ ls
// 4060174 j
// 8033020 d.log
// 5626152 d.ext
// 7214296 k"#.lines().map(String::from).collect();
//
//     let mut root: Option<Rc<RefCell<Node>>> = None;
//     let mut current: Option<Rc<RefCell<Node>>> = None;
//
//     let mut it = lines.iter();
//     while let Some(line) = it.next() {
//         let tokens: Vec<&str> = line.split_whitespace().collect();
//
//         if tokens.len() == 0 {
//             continue;
//         }
//
//         if tokens[0] == "$" {
//             match tokens[1] {
//                 "cd" => {
//                     if root.is_none() { // If no root, then the first 'cd' command will become root.
//                         root = Some(Rc::new(RefCell::new(Node::Dir(None, tokens[2].to_string(), vec![]))));
//                         current = root.clone();
//                     } 
//                     // else if let Some(node) = &current {
//                     //     
//                     //
//                     // 
//                     //
//                     //
//                     // }
//                 },
//                 // Ignoring "ls" 
//                 _ => ()
//             }
//         } else {
//             match tokens[0] {
//                 "dir" => {
//                     if let Some(n) = &current {
//                         if let Node::Dir(_, _,  children) = &*n.borrow() {
//                             children.push(Rc::new(RefCell::new(Node::Dir(current.clone(), tokens[1].to_string(), vec![]))));
//                         }
//                     }
//                 }, 
//                 _ => ()
//             }
//         }
//
//     }
//
//     println!("{:?}", root);
//
//     
//
//
//
//
//
//
//
//
//
// //     let out = r#"
// // - / (dir)
// //   - a (dir)
// //     - e (dir)
// //       - i (file, size=584)
// //     - f (file, size=29116)
// //     - g (file, size=2557)
// //     - h.lst (file, size=62596)
// //   - b.txt (file, size=14848514)
// //   - c.dat (file, size=8504156)
// //   - d (dir)
// //     - j (file, size=4060174)
// //     - d.log (file, size=8033020)
// //     - d.ext (file, size=5626152)
// //     - k (file, size=7214296)"#;
//
//     // let state = State::new(Node::Dir(String::from("/"), vec![
//     //     Node::Dir(String::from("a"), vec![
//     //         Node::Dir(String::from("e"), vec![
//     //             Node::File(String::from("i"), 584),
//     //         ]),
//     //         Node::File(String::from("f"), 29116),
//     //         Node::File(String::from("g"), 2557),
//     //         Node::File(String::from("h.lst"), 62596),
//     //     ]),
//     //     Node::File(String::from("b.txt"), 14848514),
//     //     Node::File(String::from("c.dat"), 8504156),
//     //     Node::Dir(String::from("d"), vec![
//     //         Node::File(String::from("j"), 4060174),
//     //         Node::File(String::from("d.log"), 8033020),
//     //         Node::File(String::from("d.ext"), 5626152),
//     //         Node::File(String::from("k"), 7214296),
//     //     ]),
//     // ]));
//     //
//     // println!("node size: {}", find_node_size(&state.root));
//
//     // let mut state = State::new(Node::Dir(None, String::from("/"), vec![]));
//
//
//     // state.load(&cmds);
//
//
//
// }
//
// #[cfg(test)]
// mod tests {
//     use std::{rc::Rc, cell::RefCell};
//
//     use super::*;
//
//     // fn find_node_size(node: &Node) -> u32 {
//     //     match node {
//     //         Node::Dir(_, _, nodes) => {
//     //             let mut size: u32 = 0;
//     //             for n in nodes {
//     //                 size += find_node_size(n);
//     //             }
//     //             size
//     //         },
//     //         Node::File(_, _, size) => *size,
//     //     }
//     // }
//     //
//     // #[test]
//     // fn test_find_node_size_single_file() {
//     //     let nodes = Node::File(None, String::from("a"), 10);
//     //     assert_eq!(find_node_size(&nodes), 10)
//     // }
//     //
//     // #[test]
//     // fn test_find_node_size_dir_with_files() {
//     //     let nodes = Node::Dir(None, String::from("a"), vec![
//     //         Node::File(None, String::from("b"), 1),
//     //         Node::File(None, String::from("c"), 1),
//     //     ]);
//     //     assert_eq!(find_node_size(&nodes), 2)
//     // }
//     //
//     // #[test]
//     // fn test_find_node_size_multiple_dir_with_files() {
//     //     let nodes = Node::Dir(None, String::from("a"), vec![
//     //         Node::Dir(None, String::from("d"), vec![
//     //             Node::File(None, String::from("e"), 2),
//     //             Node::File(None, String::from("f"), 2),
//     //             Node::File(None, String::from("g"), 2),
//     //         ]),
//     //         Node::Dir(None, String::from("j"), vec![
//     //             Node::File(None, String::from("k"), 2),
//     //             Node::File(None, String::from("l"), 2),
//     //             Node::File(None, String::from("m"), 2),
//     //         ])
//     //     ]);
//     //     assert_eq!(find_node_size(&nodes), 12)
//     // }
//     //
//     // #[test]
//     // fn test_find_node_size_dir_with_files_and_nested_dirs() {
//     //     let nodes = Node::Dir(None, String::from("a"), vec![
//     //         Node::File(None, String::from("b"), 1),
//     //         Node::File(None, String::from("c"), 1),
//     //         Node::Dir(None, String::from("d"), vec![
//     //             Node::File(None, String::from("e"), 2),
//     //             Node::File(None, String::from("f"), 2),
//     //             Node::File(None, String::from("g"), 2),
//     //             Node::Dir(None, String::from("h"), vec![
//     //                 Node::File(None, String::from("i"), 3),
//     //             ]),
//     //         ]),
//     //         Node::Dir(None, String::from("j"), vec![
//     //             Node::File(None, String::from("k"), 2),
//     //             Node::File(None, String::from("l"), 2),
//     //             Node::File(None, String::from("m"), 2),
//     //             Node::File(None, String::from("n"), 2),
//     //         ]),
//     //     ]);
//     //     assert_eq!(find_node_size(&nodes), 19)
//     // }
//
//     struct Fubar {
//         value: i32,
//     }
//
//     #[test]
//     fn testing_rc_fubar() {
//         let h0: Rc<Fubar> = Rc::new(Fubar { value: 42 });
//         assert_eq!(h0.value, 42);
//
//         let h1 = h0.clone();
//         let h2 = h0.clone();
//         assert_eq!(h1.value, 42);
//         assert_eq!(h2.value, 42);
//     }
//
//     #[test]
//     fn testing_rc_refcell_fubar() {
//         let fubar = Fubar { value: 42 };
//
//         // let h0: Rc<RefCell<Fubar>> = Rc::new(RefCell::new(Fubar { value: 42 }));
//         let h0: Rc<RefCell<Fubar>> = Rc::new(RefCell::new(fubar));
//         assert_eq!(h0.borrow().value, 42);
//
//         let h1 = h0.clone();
//         let h2 = h0.clone();
//         assert_eq!(h1.borrow().value, 42);
//         assert_eq!(h2.borrow().value, 42);
//
//         h1.borrow_mut().value = 22;
//         assert_eq!(h0.borrow().value, 22);
//         assert_eq!(h1.borrow().value, 22);
//         assert_eq!(h2.borrow().value, 22);
//     }
// }
//
