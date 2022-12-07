use crate::tree::{NodeId, Tree};
use itertools::Itertools;

#[derive(Debug)]
enum File {
    File { size: usize, name: String },
    Dir { name: String },
}
#[derive(Debug)]
enum Cmd {
    Cd { target: String },
    Ls { files: Vec<File> },
}

fn main() {
    let commands = parse(include_str!("../input.txt"));
    //part 1
    let res = count_small_dirs(&commands);
    println!("Smaller dirs: {}", res);
    //part 2
    let res = find_target_delete(&commands);
    println!("Delete target: {}", res);
}
fn parse(input: &str) -> Vec<Cmd> {
    let mut cmds = Vec::new();
    let linev: Vec<&str> = input.lines().collect();
    let mut lines = linev.iter();
    while let Some(line) = lines.next() {
        let mut tokens = line.split_ascii_whitespace();
        let t = tokens.next().expect("nothing on line");
        assert_eq!(t, "$", "Unexpected token");
        let t = tokens.next().expect("no cmd");
        let cmd = match t {
            "cd" => Cmd::Cd {
                target: tokens.next().expect("no cd target").to_string(),
            },
            "ls" => {
                let files = lines
                    .peeking_take_while(|l| !l.starts_with('$'))
                    .map(|l| {
                        let mut ftokens = l.split_ascii_whitespace();
                        let t = ftokens.next().expect("no file element");
                        match t {
                            "dir" => File::Dir {
                                name: ftokens.next().expect("no dir name").to_string(),
                            },
                            _ => File::File {
                                size: t.parse().expect("not int"),
                                name: ftokens.next().expect("no name").to_string(),
                            },
                        }
                    })
                    .collect();
                Cmd::Ls { files }
            }
            _ => panic!("Unexpected command {}", t),
        };
        cmds.push(cmd);
    }
    cmds
}

#[derive(Debug)]
struct DirSize {
    name: String, // We don't use names, but keep them for debugging
    size: usize,
}

mod tree {
    #[derive(Debug)]
    struct Leaf<T> {
        children: Vec<NodeId>,
        parent: NodeId,
        el: T,
    }

    #[derive(Debug)]
    pub struct Tree<T> {
        elements: Vec<Leaf<T>>,
    }

    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    pub struct NodeId {
        inner: usize,
    }
    impl NodeId {
        pub fn root() -> Self {
            NodeId { inner: 0 }
        }
    }
    impl From<NodeId> for usize {
        fn from(n: NodeId) -> usize {
            n.inner
        }
    }
    impl From<usize> for NodeId {
        fn from(inner: usize) -> NodeId {
            NodeId { inner }
        }
    }

    impl<T> Tree<T> {
        pub fn new() -> Self {
            Tree::<T> {
                elements: Vec::new(),
            }
        }
        pub fn add_child(&mut self, parent: NodeId, child: T) -> NodeId {
            if self.elements.is_empty() {
                assert!(parent == NodeId::root(), "Tree has no root yet");
            } else {
                assert!(usize::from(parent) < self.elements.len(), "Invalid node id");
            }
            self.elements.push(Leaf::<T> {
                children: Vec::new(),
                el: child,
                parent,
            });
            let id = self.elements.len() - 1;
            if id != parent.into() {
                self.elements[usize::from(parent)].children.push(id.into());
            }
            NodeId { inner: id }
        }
        pub fn get(&self, id: NodeId) -> &T {
            assert!(usize::from(id) < self.elements.len(), "Invalid node id");
            &self.elements[usize::from(id)].el
        }
        fn get_mut(&mut self, id: NodeId) -> &mut T {
            assert!(usize::from(id) < self.elements.len(), "Invalid node id");
            &mut self.elements[usize::from(id)].el
        }
        pub fn parent(&self, id: NodeId) -> NodeId {
            self.elements[usize::from(id)].parent
        }
        pub fn apply_parents<F>(&mut self, mut id: NodeId, f: F)
        where
            F: Fn(&mut T),
        {
            assert!(usize::from(id) < self.elements.len(), "Invalid node id");
            loop {
                f(&mut self.elements[usize::from(id)].el);
                if id == NodeId::root() {
                    break;
                }
                id = self.elements[usize::from(id)].parent;
            }
        }
        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.elements.iter().map(|d| &d.el)
        }
    }
}

fn eval_tree(commands: &[Cmd]) -> Tree<DirSize> {
    let mut t = Tree::<DirSize>::new();
    //let mut current_dir = "".to_string();
    let mut parent_id = NodeId::root();
    let mut current_id = NodeId::root();
    for cmd in commands.iter() {
        match cmd {
            Cmd::Cd { target } => {
                if target == ".." {
                    assert!(
                        parent_id != current_id,
                        "Going up too much ? {:?}, {:?}, status: {:?}",
                        current_id,
                        parent_id,
                        &t,
                    );
                    // go up
                    (parent_id, current_id) = (t.parent(parent_id), parent_id);
                } else {
                    let new = t.add_child(
                        current_id,
                        DirSize {
                            name: target.clone(),
                            size: 0,
                        },
                    );
                    (parent_id, current_id) = (current_id, new);
                }
            }
            Cmd::Ls { files } => {
                files.iter().for_each(|f| match f {
                    File::File { size, .. } => {
                        t.apply_parents(current_id, |d| d.size += size);
                    }
                    File::Dir { .. } => {}
                });
            }
        }
    }
    t
}
fn count_small_dirs(commands: &[Cmd]) -> usize {
    let tree = eval_tree(commands);
    tree.iter().map(|ds| ds.size).filter(|s| *s < 100000).sum()
}

fn find_target_delete(commands: &[Cmd]) -> usize {
    let tree = eval_tree(commands);
    let mut dir_sizes: Vec<usize> = tree.iter().map(|ds| ds.size).collect();
    dir_sizes.sort();
    let free_space = 70000000 - tree.get(NodeId::root()).size;
    let target_del = 30000000 - free_space;
    dir_sizes
        .into_iter()
        .find(|x| *x >= target_del)
        .expect("no target")
}

#[test]
fn test() {
    let commands = parse(include_str!("../sample.txt"));
    //part 1
    let res = count_small_dirs(&commands);
    assert_eq!(res, 95437);
    //part 2
    let res = find_target_delete(&commands);
    assert_eq!(res, 24933642);
}
