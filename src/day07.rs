use std::vec;


#[derive(Debug)]
enum Node {
    Dir{ name: String, size: Option<usize>, content: Vec<Node>},
    File{ name: String, size: usize},
}

impl Node {
    fn name(&self) -> &str {
        match self {
            Node::Dir { name, ..} => name,
            Node::File { name, ..} => name,
        }
    }

    fn size(&self) -> usize {
        match self {
            Node::Dir { size, ..} => size.unwrap(),
            Node::File { size, ..} => *size,
        }
    }

    fn create_directories(&mut self, path: &[String]) -> &mut Node {
        if path.is_empty() {
            return self;
        }

        let subdir_name = path.first().unwrap();

        match self {
            Node::Dir { content ,..} => {
                if content.iter_mut().find(|child| child.name() == subdir_name).is_none() {
                    content.push(Node::Dir{name: subdir_name.to_owned(), size: None, content: vec![] });
                }
                let subdir = content.iter_mut().find(|child| child.name() == subdir_name).unwrap();
                subdir.create_directories(&path[1..])
            }
            Node::File { .. } => panic!(),
        }
    }

    fn set_file_size(&mut self, name: &str, new_size: usize) {
        match self {
            Node::Dir { content ,..} => {
                match content.iter_mut().find(|child| child.name() == name) {
                    Some(child) => match child {
                        Node::Dir { .. } => panic!(),
                        Node::File { size,  .. } => {*size = new_size;},
                    },
                    None => {
                        content.push(Node::File{name: name.to_owned(), size: new_size});
                    }
                };
            }
            Node::File { .. } => panic!(),
        }
    }
}

fn parse(input: &str) -> Node {
    let mut node = Node::Dir { name:"/".to_owned(), content: vec![], size: None};
    let mut cwd = vec![];
    let mut cwd_node = &mut node;
    for line in input.lines() {
        if line.starts_with("$") {
            let command = &line[2..];
            if command.starts_with("cd ") {
                let argument = &command[3..];
                if argument == ".." {
                    cwd.pop();
                } else if argument == "/" {
                    cwd.clear();
                } else {
                    cwd.push(argument.to_owned());
                }

                cwd_node = node.create_directories(&cwd);
            } else if command.starts_with("ls") {
                // ignore
            }
        } else if line.starts_with("dir") {
            // ignore
        } else {
            let mut parts = line.split_ascii_whitespace(); 
            let size: usize = parts.next().unwrap().parse().unwrap();
            let name = parts.next().unwrap();
            cwd_node.set_file_size(name, size);
        }
    }

    node
}

fn compute_sizes(node: &mut Node) -> usize {
    match node {
        Node::Dir { name, size, content } => {
            let content_size = content.iter_mut().map(compute_sizes).sum();

            *size = Some(content_size);

            content_size
        },
        Node::File { name, size } => *size,
    }
}

fn directory_sizes(node: &Node, sizes: &mut Vec<usize>) {
    match node {
        Node::Dir { size, content, .. } => {
            sizes.push(size.unwrap());

            for node in content {
                directory_sizes(node, sizes);
            }
        }
        _ => {},
    }
}

#[test]
fn test() {
    let mut node = Node::Dir{name: "/".to_owned(), size: None, content: vec![
        Node::Dir{name:"a".to_owned(), size: None, content: vec![
            Node::File{name:"small".to_owned(), size: 1000},
            Node::File{name:"small_2".to_owned(), size: 1000},
        ]},
        Node::File{name:"small".to_owned(), size: 1000},
        Node::File{name:"small_2".to_owned(), size: 1000},
    ]};

    compute_sizes(&mut node);

    assert_eq!(if let Node::Dir{size, .. } = node {size.unwrap()} else {0}, 4000);

    let input = "$ cd /
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
7214296 k";

    let mut node = parse(input);

    compute_sizes(&mut node);

    let mut sizes = Vec::<usize>::new();
    directory_sizes(&node, &mut sizes);

    let total:usize = sizes.iter().filter(|size| **size <= 100000).sum();
    assert_eq!(95437, total);

    let root_size = node.size();
    let capacity = 70000000;

    let free_space = capacity - root_size;

    assert_eq!(21618835, free_space);

    let required_min = 30000000 - free_space;

    assert_eq!(8381165, required_min);

    sizes.sort();

    let &value = sizes.iter().filter(|&&val| val >= required_min ).next().unwrap();

    assert_eq!(24933642, value);
}

#[test]
fn real() {
    let input = include_str!("../input/day07.txt");

    let mut node = parse(input);

    compute_sizes(&mut node);

    let mut sizes = Vec::<usize>::new();
    directory_sizes(&node, &mut sizes);

    let total:usize = sizes.iter().filter(|size| **size <= 100000).sum();
    assert_eq!(2061777, total);

    let root_size = node.size();
    let capacity = 70000000;

    let free_space = capacity - root_size;

    let required_min = 30000000 - free_space;

    sizes.sort();

    let &value = sizes.iter().filter(|&&val| val >= required_min ).next().unwrap();

    assert_eq!(4473403, value);
    
}

