use std::fs::File;
use std::io::{BufWriter, Write};

pub enum InfoEntry {
    InfoElem {
        original: i64,
        witness: i64,
        node_id: i64,
        status: String,
        symbol: String,
    },
    InfoHeader {
        size: usize,
        node_id: i64,
        num_inputs: usize,
        num_outputs: usize,
        num_internals: usize,
    },
}

impl ToString for InfoEntry {
    fn to_string(&self) -> String {
        use InfoEntry::*;
        match self {
            InfoElem { original, witness, node_id, status, symbol } =>
                format!("{},{},{},{},{}", original, witness, node_id, status, symbol),
            InfoHeader { size, node_id, num_inputs, num_outputs, num_internals } => format!("{},{},{},{},{}", size, node_id,  num_outputs, num_inputs, num_internals)
        }
    }
}

pub struct InfoFile {
    writer: BufWriter<File>,
}

impl InfoFile {
    pub fn new(file: &str) -> Result<InfoFile, ()> {
        let file = File::create(file).map_err(|_err| {})?;
        let writer = BufWriter::new(file);
        Result::Ok(InfoFile { writer })
    }

    pub fn write_sym_elem(sym: &mut InfoFile, elem: InfoEntry) -> Result<(), ()> {
        sym.writer.write_all(elem.to_string().as_bytes()).map_err(|_err| {})?;
        sym.writer.write_all(b"\n").map_err(|_err| {}) //?;
        //sym.writer.flush().map_err(|_err| {})
    }

    pub fn finish_writing(mut sym: InfoFile) -> Result<(), ()> {
	sym.writer.flush().map_err(|_err| {})
    }

    // pub fn close(_sym: SymFile) {}
}
