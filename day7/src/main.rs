use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(PartialEq, Debug)]
enum NodeType {
    FILE(String),
    DIR(String),
}

#[derive(PartialEq)]
struct TreeNode {
    pub node_type: Option<NodeType>,
    pub node_size: Option<u32>,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new() -> TreeNode {
        return TreeNode {
            node_type: None,
            node_size: None,
            children: vec![],
            parent: None,
        };
    }

    pub fn update_size(&mut self) -> u32 {
        if let Some(value) = &self.node_type {
            match value {
                NodeType::FILE(_) => return self.node_size.unwrap(),
                NodeType::DIR(_) => {
                    let children_size: u32 = self.children.iter().map(|tn| tn.borrow_mut().update_size()).sum();
                    self.node_size = Some(children_size);
                    return children_size;
                }
            }
        }
        return 0;
    }

    pub fn traverse_with_sum(&self, accum: &mut u32) {
        if let Some(value) = &self.node_type {
            if let NodeType::DIR(_) = value {
                let node_size  = self.node_size.unwrap();
                if node_size <= 100000 {
                    *accum = *accum + node_size;
                }
            }
        }
        self.children.iter().map(|tn| tn.borrow().traverse_with_sum(accum)).collect::<Vec<_>>();
    }

    pub fn traverse_with_check(&self, size_needed: u32, min_size: &mut u32) {
        if let Some(value) = &self.node_type {
            if let NodeType::DIR(dirname) = value {
                let node_size  = self.node_size.unwrap();
                if node_size >= size_needed && node_size < *min_size {
                    *min_size = node_size;
                }
            }
        }
        self.children.iter().map(|tn| tn.borrow().traverse_with_check(size_needed, min_size)).collect::<Vec<_>>();
    }


    pub fn print(&self) -> String {
        let mut res = String::new();
        if let Some(value) = &self.node_type {
            res = format!("{:?} {:?}", value, self.node_size);
        }
        if !&self.children.is_empty() {
            let children_str = String::from("[")
            + &self
                .children
                .iter()
                .map(|tn| tn.borrow().print())
                .collect::<Vec<String>>()
                .join(", ")
            + "]";
            res.push_str(&children_str);
        }
        res
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input2.txt") {
        let root = Rc::new(RefCell::new(TreeNode::new()));
        {
            let mut mut_root = root.borrow_mut();
            mut_root.node_type = Some(NodeType::DIR("root".to_owned()));
            mut_root.node_size = Some(0);
        }
        let mut current = Rc::clone(&root);
        for line in lines {
            if let Ok(data) = line {
                if data.starts_with("$ cd") {
                    if data.starts_with("$ cd /") {
                        current = Rc::clone(&root);
                    } else if data.starts_with("$ cd ..") {
                        let current_clone = Rc::clone(&current);
                        current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                    } else {
                        // cd dir
                        let (_, target_dir) = data.split_at(5);
                        let cloned_current = Rc::clone(&current);
                        for child in &cloned_current.borrow().children {
                            if let Some(NodeType::DIR(dirname)) = &child.borrow().node_type {
                                if dirname == target_dir {
                                    current = Rc::clone(&child);
                                    break;
                                }
                            }
                        }
                    }
                } else if data.starts_with("$ ls") {
                    //do nothing
                } else {
                    //should be ls output
                    let v: Vec<&str> = data.split(' ').collect();
                    assert_eq!(v.len(), 2);
                    let name: String = String::from(v[1]);
                    let node_type;
                    let mut node_size = None;
                    if v[0] == "dir" {
                        node_type = Some(NodeType::DIR(name));
                    } else {
                        node_type = Some(NodeType::FILE(name));
                        node_size = Some(v[0].parse::<u32>().unwrap());
                    }
                    let child = Rc::new(RefCell::new(TreeNode::new()));
                    current.borrow_mut().children.push(Rc::clone(&child));
                    {
                        let mut mut_child = child.borrow_mut();
                        mut_child.parent = Some(Rc::clone(&current));
                        mut_child.node_type = node_type;
                        mut_child.node_size = node_size;
                    }
                }
            }
        }

        root.borrow_mut().update_size();
        //println!("Current tree is {}", root.borrow().print());
        let unused_space = 70000000 - root.borrow().node_size.unwrap();
        let space_needed = 30000000 - unused_space;
        println!("Size needed is {}", &space_needed);
        //let mut accum = 0u32;
        //root.borrow().traverse_with_sum(&mut accum);
        //println!("Sum is {}", accum);
        let mut min_folder_size = 70000000u32;
        root.borrow().traverse_with_check(space_needed, &mut min_folder_size);
        println!("Min folder size is {}", min_folder_size);
    }
}
