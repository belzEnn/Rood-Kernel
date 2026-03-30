// In-memory filesystem (RAM disk)

use alloc::vec::Vec;
use alloc::string::String;

// Тип узла / Node type
#[derive(PartialEq)]
pub enum NodeType {
    File,
    Dir,
}

// Filesystem node
pub struct Node {
    pub name:      String,       // Имя файла/папки / File/dir name
    pub node_type: NodeType,     // Тип: файл или папка / Type: file or dir
    pub data:      Vec<u8>,      // Содержимое файла / File contents
    pub parent:    usize,        // Индекс родителя / Parent index
}

// Node table
static mut NODES: Option<Vec<Node>> = None;

// Current directory
static mut CWD: usize = 0;

// Init — create root dir
pub unsafe fn init() {
    let mut nodes = Vec::new();

    // Root
    nodes.push(Node {
        name:      String::from("/"),
        node_type: NodeType::Dir,
        data:      Vec::new(),
        parent:    0,
    });

    NODES = Some(nodes);
    CWD = 0;
}

// Get node table
unsafe fn nodes() -> &'static mut Vec<Node> {
    NODES.as_mut().unwrap()
}

// Get current directory index
pub unsafe fn cwd() -> usize {
    CWD
}

// Get current directory name
pub unsafe fn cwd_name() -> &'static str {
    &nodes()[CWD].name
}

// Find node by name in current dir
pub unsafe fn find(name: &str) -> Option<usize> {
    let ns = nodes();
    for (i, node) in ns.iter().enumerate() {
        if node.parent == CWD && node.name == name {
            return Some(i);
        }
    }
    None
}

// Create file
pub unsafe fn create_file(name: &str) -> Result<(), &'static str> {
    if find(name).is_some() {
        return Err("File already exists");
    }
    nodes().push(Node {
        name:      String::from(name),
        node_type: NodeType::File,
        data:      Vec::new(),
        parent:    CWD,
    });
    Ok(())
}

// Create directory
pub unsafe fn create_dir(name: &str) -> Result<(), &'static str> {
    if find(name).is_some() {
        return Err("Directory already exists");
    }
    nodes().push(Node {
        name:      String::from(name),
        node_type: NodeType::Dir,
        data:      Vec::new(),
        parent:    CWD,
    });
    Ok(())
}

// Remove file
pub unsafe fn remove(name: &str) -> Result<(), &'static str> {
    let idx = find(name).ok_or("File not found")?;
    if nodes()[idx].node_type == NodeType::Dir {
        return Err("Use rmdir to remove directories");
    }
    nodes().remove(idx);
    Ok(())
}

// Write data to file
pub unsafe fn write(name: &str, data: &[u8]) -> Result<(), &'static str> {
    let idx = find(name).ok_or("File not found")?;
    if nodes()[idx].node_type == NodeType::Dir {
        return Err("Cannot write to directory");
    }
    nodes()[idx].data = Vec::from(data);
    Ok(())
}

// Read file
pub unsafe fn read(name: &str) -> Result<&'static [u8], &'static str> {
    let idx = find(name).ok_or("File not found")?;
    if nodes()[idx].node_type == NodeType::Dir {
        return Err("Cannot read directory");
    }
    Ok(&nodes()[idx].data)
}

// Change directory
pub unsafe fn chdir(name: &str) -> Result<(), &'static str> {
    if name == ".." {
        // Go to parent dir
        if CWD != 0 {
            CWD = nodes()[CWD].parent;
        }
        return Ok(());
    }
    let idx = find(name).ok_or("Directory not found")?;
    if nodes()[idx].node_type != NodeType::Dir {
        return Err("Not a directory");
    }
    CWD = idx;
    Ok(())
}

// List files in current dir
pub unsafe fn list() -> Vec<(&'static str, bool)> {
    let ns = nodes();
    let mut result = Vec::new();
    for node in ns.iter() {
        if node.parent == CWD && node.name != "/" {
            result.push((
                node.name.as_str(),
                node.node_type == NodeType::Dir,
            ));
        }
    }
    result
}
