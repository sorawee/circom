use std::fs::File;
use std::io::{BufWriter, Write};

pub struct InfoElem {
    pub original: i64,
    pub witness: i64,
    pub node_id: i64,
    pub status: String,
    pub symbol: String,
}
impl ToString for InfoElem {
    fn to_string(&self) -> String {
        format!("{},{},{},{},{}", self.original, self.witness, self.node_id, self.status, self.symbol)
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

    pub fn write_sym_elem(sym: &mut InfoFile, elem: InfoElem) -> Result<(), ()> {
        sym.writer.write_all(elem.to_string().as_bytes()).map_err(|_err| {})?;
        sym.writer.write_all(b"\n").map_err(|_err| {}) //?;
        //sym.writer.flush().map_err(|_err| {})
    }

    pub fn finish_writing(mut sym: InfoFile) -> Result<(), ()> {
	sym.writer.flush().map_err(|_err| {})
    }

    // pub fn close(_sym: SymFile) {}
}
