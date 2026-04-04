use alloc::vec::Vec;
use alloc::string::{String};
use alloc::string::ToString;
use crate::drivers::disk::ata;

// Format constants

const MAGIC:         [u8; 4] = *b"ROOD"; // Magic number
const HEADER_SECTOR: u32     = 1;        // Header sector
const DATA_SECTOR:   u32     = 2;        // First data sector
const MAX_NAME:      usize   = 64;       // Max name length
const MAX_DATA:      usize   = 435;      // Max data per sector

// Node type

#[derive(PartialEq, Clone, Copy)]
pub enum NodeType {
    File = 0,
    Dir  = 1,
}

// Node

pub struct Node {
    pub name:      String,
    pub node_type: NodeType,
    pub data:      Vec<u8>,
    pub parent:    usize,
}

// Global state

static mut NODES: Option<Vec<Node>> = None;
static mut CWD:   usize             = 0;

unsafe fn nodes() -> &'static mut Vec<Node> {
    NODES.as_mut().unwrap()
}

// Serialize node to sector

fn serialize_node(node: &Node, buf: &mut [u8; ata::SECTOR_SIZE]) {
    buf.fill(0);

    // Type
    buf[0] = node.node_type as u8;

    // Parent (u32 LE)
    let p = node.parent as u32;
    buf[1] = (p & 0xFF) as u8;
    buf[2] = ((p >> 8)  & 0xFF) as u8;
    buf[3] = ((p >> 16) & 0xFF) as u8;
    buf[4] = ((p >> 24) & 0xFF) as u8;

    // Name
    let name = node.name.as_bytes();
    let name_len = name.len().min(MAX_NAME) as u32;
    buf[5] = (name_len & 0xFF) as u8;
    buf[6] = ((name_len >> 8)  & 0xFF) as u8;
    buf[7] = ((name_len >> 16) & 0xFF) as u8;
    buf[8] = ((name_len >> 24) & 0xFF) as u8;
    buf[9..9 + name_len as usize].copy_from_slice(&name[..name_len as usize]);

    // Data
    let data_len = node.data.len().min(MAX_DATA) as u32;
    buf[73] = (data_len & 0xFF) as u8;
    buf[74] = ((data_len >> 8)  & 0xFF) as u8;
    buf[75] = ((data_len >> 16) & 0xFF) as u8;
    buf[76] = ((data_len >> 24) & 0xFF) as u8;
    buf[77..77 + data_len as usize].copy_from_slice(&node.data[..data_len as usize]);
}

// Deserialize node from sector

fn deserialize_node(buf: &[u8; ata::SECTOR_SIZE]) -> Node {
    // Type
    let node_type = if buf[0] == 1 { NodeType::Dir } else { NodeType::File };

    // Parent
    let parent = buf[1] as usize
        | ((buf[2] as usize) << 8)
        | ((buf[3] as usize) << 16)
        | ((buf[4] as usize) << 24);

    // Name
    let name_len = (buf[5] as usize)
        | ((buf[6] as usize) << 8)
        | ((buf[7] as usize) << 16)
        | ((buf[8] as usize) << 24);
    let name_len = name_len.min(MAX_NAME);
    let name = core::str::from_utf8(&buf[9..9 + name_len])
        .unwrap_or("?")
        .to_string();

    // Data
    let data_len = (buf[73] as usize)
        | ((buf[74] as usize) << 8)
        | ((buf[75] as usize) << 16)
        | ((buf[76] as usize) << 24);
    let data_len = data_len.min(MAX_DATA);
    let data = buf[77..77 + data_len].to_vec();

    Node { name, node_type, data, parent }
}

// Save filesystem to disk

pub unsafe fn save() -> Result<(), &'static str> {
    let ns = nodes();

    // Write header
    let mut header = [0u8; ata::SECTOR_SIZE];
    header[0..4].copy_from_slice(&MAGIC);
    let count = ns.len() as u32;
    header[4] = (count & 0xFF) as u8;
    header[5] = ((count >> 8)  & 0xFF) as u8;
    header[6] = ((count >> 16) & 0xFF) as u8;
    header[7] = ((count >> 24) & 0xFF) as u8;
    ata::write_sector(HEADER_SECTOR, &header)?;

    // Write nodes
    for (i, node) in ns.iter().enumerate() {
        let mut buf = [0u8; ata::SECTOR_SIZE];
        serialize_node(node, &mut buf);
        ata::write_sector(DATA_SECTOR + i as u32, &buf)?;
    }

    Ok(())
}

// Load filesystem from disk

pub unsafe fn load() -> bool {
    //  Read header
    let mut header = [0u8; ata::SECTOR_SIZE];
    if ata::read_sector(HEADER_SECTOR, &mut header).is_err() {
        return false;
    }

    // Check magic
    if &header[0..4] != &MAGIC {
        return false; // Disk not formatted
    }

    // Read node count
    let count = (header[4] as usize)
        | ((header[5] as usize) << 8)
        | ((header[6] as usize) << 16)
        | ((header[7] as usize) << 24);

    // Read nodes
    let mut loaded = Vec::new();
    for i in 0..count {
        let mut buf = [0u8; ata::SECTOR_SIZE];
        if ata::read_sector(DATA_SECTOR + i as u32, &mut buf).is_err() {
            return false;
        }
        loaded.push(deserialize_node(&buf));
    }

    NODES = Some(loaded);
    CWD = 0;
    true
}

// Init

pub unsafe fn init() {
    NODES = Some(Vec::new());
    CWD = 0;

    if ata::detect_drive(0xB0) {
        ata::set_active_drive(0xB0);
    } else if ata::detect_drive(0xA0) {
        ata::set_active_drive(0xA0);
    }

    if load() {
        return;
    }

    // Disk found but not formatted
    nodes().push(Node {
        name:      String::from("/"),
        node_type: NodeType::Dir,
        data:      Vec::new(),
        parent:    0,
    });
    let _ = save();
    // Create root directory
    nodes().push(Node {
        name:      String::from("/"),
        node_type: NodeType::Dir,
        data:      Vec::new(),
        parent:    0,
    });

    // Save if disk available
    let _ = save();
}

// Public API

pub unsafe fn cwd() -> usize { CWD }

pub unsafe fn cwd_name() -> &'static str { &nodes()[CWD].name }

pub unsafe fn find(name: &str) -> Option<usize> {
    let ns = nodes();
    for (i, node) in ns.iter().enumerate() {
        if node.parent == CWD && node.name == name {
            return Some(i);
        }
    }
    None
}

pub unsafe fn create_file(name: &str) -> Result<(), &'static str> {
    if find(name).is_some() { return Err("File already exists"); }
    nodes().push(Node {
        name:      String::from(name),
        node_type: NodeType::File,
        data:      Vec::new(),
        parent:    CWD,
    });
    save()
}

pub unsafe fn create_dir(name: &str) -> Result<(), &'static str> {
    if find(name).is_some() { return Err("Directory already exists"); }
    nodes().push(Node {
        name:      String::from(name),
        node_type: NodeType::Dir,
        data:      Vec::new(),
        parent:    CWD,
    });
    save()
}

pub unsafe fn remove(name: &str) -> Result<(), &'static str> {
    let idx = find(name).ok_or("File not found")?;
    if nodes()[idx].node_type == NodeType::Dir {
        return Err("Use rmdir to remove directories");
    }
    nodes().remove(idx);
    save()
}

pub unsafe fn write(name: &str, data: &[u8]) -> Result<(), &'static str> {
    let idx = find(name).ok_or("File not found")?;
    if nodes()[idx].node_type == NodeType::Dir {
        return Err("Cannot write to directory");
    }
    nodes()[idx].data = Vec::from(data);
    save()
}

pub unsafe fn read(name: &str) -> Result<&'static [u8], &'static str> {
    let idx = find(name).ok_or("File not found")?;
    if nodes()[idx].node_type == NodeType::Dir {
        return Err("Cannot read directory");
    }
    Ok(&nodes()[idx].data)
}

pub unsafe fn chdir(name: &str) -> Result<(), &'static str> {
    if name == ".." {
        if CWD != 0 { CWD = nodes()[CWD].parent; }
        return Ok(());
    }
    let idx = find(name).ok_or("Directory not found")?;
    if nodes()[idx].node_type != NodeType::Dir {
        return Err("Not a directory");
    }
    CWD = idx;
    Ok(())
}

pub unsafe fn list() -> Vec<(&'static str, bool)> {
    let ns = nodes();
    let mut result = Vec::new();
    for node in ns.iter() {
        if node.parent == CWD && node.name != "/" {
            result.push((node.name.as_str(), node.node_type == NodeType::Dir));
        }
    }
    result
}
