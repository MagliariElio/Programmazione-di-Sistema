use std::any::Any;
use std::fmt::{format, write, Display, Formatter};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
enum FileType {
    Text,
    Binary,
}

#[derive(Clone)]
struct File {
    name: String,
    content: Vec<u8>, // max 1000 bytes, rest of the file truncated
    creation_time: u64,
    type_: FileType,
}

impl File {
    pub fn new(param_name: String, file_type: FileType) -> Self {
        File {
            name: param_name,
            content: Vec::new(),
            creation_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Error Creation Time")
                .as_secs() as u64,
            type_: file_type,
        }
    }
}

#[derive(Clone)]
pub struct Dir {
    name: String,
    creation_time: u64,
    children: Vec<Node>,
}

impl Dir {
    fn new(param_name: String) -> Self {
        Dir {
            name: param_name,
            creation_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Error Creation Time")
                .as_secs() as u64,
            children: Vec::new(),
        }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for node in &self.children {
            write!(f, "dir: {} ", node)?;
        }
        Ok(())
    }

}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::File(file) => write!(f, "{}", file.name),
            Node::Dir(dir) => write!(f, "{}", dir.name)
        }
    }

}

#[derive(Clone)]
enum Node {
    File(File),
    Dir(Dir),
}

#[derive(Clone)]
pub struct FileSystem {
    pub root: Dir,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            root: Dir::new("/".to_string()),
        }
    }

    pub fn mk_dir(&mut self, param_path: String) {
        let mut path: Vec<String> = split_dir(param_path);  // separa ogni sotto path in un vettore di stringhe

        let mut dir = Dir::new("".to_string());

        let dir_to_add = path.get(path.len()-1).unwrap().clone();
        path.remove(path.len()-1);

        if path.len() == 0 {
            dir = Dir::new(dir_to_add.to_string());
        } else {
            for directory in path {
                let _ = self.root.children.iter().map(|x| println!("{}", *x));

                /*match self.root.children.iter().map(|&node| node.name == directory) {
                    Some(result) => println!("{}", result),
                    None => {
                        println!("Errore");
                        return;
                    }
                }*/
            }
        }

        println!("dirs: {:?}", dir.name);
        self.root.children.push(Node::Dir(dir));
















//        self.root.children.iter().find(|node| node)

        //println!("{:?}", self.root.children.iter().find(|&x| *x.name == path_separeted[0]));

        // se il vettore ha almeno un elemento inizializza la cartella padre dir
        /*if let Some(result) = path_separeted.get(0) {
            dir = Dir::new(result.to_string())
        } else {
            println!("{}", "Error path");
            return dir;
        }

        // inserisce ogni sottocartella nella cartella padre dir
        for sub in path_separeted {
            let sub_dir = Dir::new(sub);
            dir.children.push(Node::Dir(sub_dir));
        }

        // inserisce la cartella dir con le sue sottocartelle nel filesystem
        self.root.children.push(Node::Dir(dir.clone()));
        return dir;*/
    }
}

fn split_dir(path: String) -> Vec<String> {
    let mut result: Vec<String> = path
        .split("/")
        .filter(|&s| !s.is_empty())
        .map(|s| s.trim().to_owned())
        .collect();
    //result = result.iter().map(|s| format!("{}{}", s, "/")).collect();

    println!("{:?}", result);

    return result;
}
