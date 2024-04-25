use std::{collections::HashMap, fmt};

use byte_code::Instruction;

#[derive(Debug, Clone, Copy)]
pub struct Data_Layout {
    pub address: usize,
    pub size:    usize,
}

#[derive(Debug, Clone)]
pub struct Object {
    pub data_layout: HashMap<String, Data_Layout>,
    pub data_bytes:  Vec<u8>,

    pub code_labels: HashMap<String, usize>,
    pub code_instrs: Vec<Instruction>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            data_layout: HashMap::new(),
            data_bytes:  Vec::new(),
            code_labels: HashMap::new(),
            code_instrs: Vec::new(),
        }
    }

    pub fn add_label(&mut self, label: impl AsRef<str>) {
        self.code_labels
            .insert(label.as_ref().to_string(), self.code_instrs.len());
    }

    pub fn add_data_entry(&mut self, name: impl AsRef<str>, size: usize, bytes: &[u8]) {
        let address = self.data_bytes.len();
        self.data_layout
            .insert(name.as_ref().to_string(), Data_Layout { address, size });
        self.data_bytes.extend_from_slice(bytes);
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "---------------------------")?;
        writeln!(f, "Code:    {:?}", self.code_instrs)?;
        writeln!(f, "Labels:  {:?}", self.code_labels)?;
        writeln!(f, "Data:    {:?}", self.data_bytes)?;
        writeln!(f, "Layouts: {:?}", self.data_layout)
    }
}
