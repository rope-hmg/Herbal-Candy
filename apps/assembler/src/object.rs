use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, Copy)]
struct Data_Layout {
    address: usize,
    size: usize,
}

#[derive(Debug, Clone)]
pub struct Object {
    data_layout: HashMap<String, Data_Layout>,
    data_bytes: Vec<u8>,

    code_labels: HashMap<String, usize>,
    code_bytes: Vec<u8>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            data_layout: HashMap::new(),
            data_bytes: Vec::new(),
            code_labels: HashMap::new(),
            code_bytes: Vec::new(),
        }
    }

    pub fn add_label(&mut self, label: impl AsRef<str>) {
        self.code_labels
            .insert(label.as_ref().to_string(), self.code_bytes.len());
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
        writeln!(f, "Code:    {:?}", self.code_bytes)?;
        writeln!(f, "Labels:  {:?}", self.code_labels)?;
        writeln!(f, "Data:    {:?}", self.data_bytes)?;
        writeln!(f, "Layouts: {:?}", self.data_layout)
    }
}
