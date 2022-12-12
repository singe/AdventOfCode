use std::io::BufRead;

struct NODE {
  name: String,
  size: usize,
  parent: Option<usize>,
  children: Vec<usize>,
}

type Tree = Vec<NODE>;

fn add_dir(tree: &mut Tree, name: &str, parent: Option<usize>) {
    tree.push(NODE {
        name: name.to_owned(),
        size: 0,
        parent: parent,
        children: Vec::new(),
    });
    if let Some(parent_index) = parent {
        let last_index = tree.len()-1;
        tree[parent_index].children.push(last_index);
    }
}

fn cd_dir(tree: &Tree, index: usize, name: &str) -> usize {
    for nodeid in &tree[index].children {
        if tree[*nodeid].name == name {
            return *nodeid;
        }
    } 
    0
}

fn walk(tree: &Vec<NODE>, index: usize, total: &mut usize) -> usize {
    let name = &tree[index].name;
    let size = tree[index].size;
    let parent = tree[index].parent;
    let mut full_size = size;
    for child_index in &tree[index].children {
        full_size += walk(tree, *child_index, total);
    }
    if full_size <= 100000 {
        *total += full_size;
    }
    //println!("{parent:?} {name},{index} {size} {full_size}");
    full_size
}


fn main() {
    let mut tree: Tree = Vec::new();
    add_dir(&mut tree, "/",None);
    let mut cwd = tree.len()-1;

    let mut lines = std::io::stdin().lock().lines().map(|l| l.unwrap());
    for line in lines {
        let mut tokens = line.split(' ');
        match tokens.next() {
            Some("$") => match tokens.next() {
                Some("cd") => match tokens.next() { 
                    Some("..") => cwd = tree[cwd].parent.unwrap(),
                    Some(dir) => cwd = cd_dir(&mut tree, cwd, dir),
                    None => continue,
                },
                Some("ls") => continue,
                None => continue,
                Some(&_) => continue,
            },
            Some("dir") => add_dir(&mut tree, tokens.next().unwrap(), Some(cwd)),
            None => (),
            Some(size) => {
                let num = size.parse::<usize>().unwrap();
                tree[cwd].size += num;
            },
        }
    }
    let mut total = 0;
    walk(&tree, 0, &mut total);
    println!("{total}");
}
