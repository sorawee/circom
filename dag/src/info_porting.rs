use super::{Tree, DAG};
use circom_algebra::num_traits::AsPrimitive;
use constraint_writers::info_writer::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn write(dag: &DAG, file_name: &str) -> Result<(), ()> {
    let tree = Tree::new(dag);
    let mut dot_sym = InfoFile::new(file_name)?;
    visit_tree(&tree, &mut dot_sym)?;
    InfoFile::finish_writing(dot_sym)?;
    //SymFile::close(dot_sym);
    Ok(())
}

fn visit_tree(tree: &Tree, dot_sym: &mut InfoFile) -> Result<(), ()> {
    for signal in &tree.signals {
        let name = HashMap::get(&tree.id_to_name, signal).unwrap();
        let symbol = format!("{}.{}", tree.path, name);
        let original = signal.as_();
        let witness = original;
        let node_id = tree.node_id.as_();
        let mut status = "internal";
        if HashSet::contains(&tree.signal_inputs, signal) {
            status = "input";
        } else if HashSet::contains(&tree.signal_outputs, signal) {
            status = "output";
        }
        let sym_elem = InfoElem { original, witness, node_id, symbol, status: status.to_string() };
        InfoFile::write_sym_elem(dot_sym, sym_elem)?;
    }
    for edge in Tree::get_edges(tree) {
        let subtree = Tree::go_to_subtree(tree, edge);
        visit_tree(&subtree, dot_sym)?;
    }
    Ok(())
}
