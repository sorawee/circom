use super::{Tree, DAG};
use circom_algebra::num_traits::AsPrimitive;
use constraint_writers::info_writer::*;
// use std::collections::HashMap;
// use std::collections::HashSet;

pub fn write(dag: &DAG, file_name: &str) -> Result<(), ()> {
    let tree = Tree::new(dag);
    let mut dot_sym = InfoFile::new(file_name)?;
    visit_tree(&tree, &mut dot_sym)?;
    InfoFile::finish_writing(dot_sym)?;
    //SymFile::close(dot_sym);
    Ok(())
}

fn visit_tree(tree: &Tree, dot_sym: &mut InfoFile) -> Result<(), ()> {
    use InfoEntry::*;

    let node_id = tree.node_id.as_();

    // for signal in &tree.signals {
    //     let name = HashMap::get(&tree.id_to_name, signal).unwrap();
    //     let symbol = format!("{}.{}", tree.path, name);
    //     let original = signal.as_();
    //     let witness = original;
    //     let mut status = "internal";
    //     if HashSet::contains(&tree.signal_inputs, signal) {
    //         status = "input";
    //     } else if HashSet::contains(&tree.signal_outputs, signal) {
    //         status = "output";
    //     }
    //     InfoFile::write_sym_elem(dot_sym, InfoElem { original, witness, node_id, symbol, status: status.to_string() })?;
    // }
    let size = tree.constraints.len();
    let num_inputs = tree.signal_inputs.len();
    let num_outputs = tree.signal_outputs.len();
    let num_internals = tree.signals.len() - num_inputs - num_outputs;
    InfoFile::write_sym_elem(dot_sym, InfoHeader { size, node_id, num_inputs, num_outputs, num_internals })?;

    for edge in Tree::get_edges(tree) {
        let subtree = Tree::go_to_subtree(tree, edge);
        visit_tree(&subtree, dot_sym)?;
    }
    Ok(())
}
