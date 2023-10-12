use std::vec;

use extendr_api::prelude::*;
use ndarray::Array2;
use rustc_hash::FxHashMap;
use webgestalt_lib::{
    methods::{
        gsea::{GSEAConfig, RankListItem},
        *,
    },
    readers::utils::Item,
};
/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn rust_hello_world() -> &'static str {
    "Hello world!"
}

/// Run GSEA using rust library
/// @return List of the results of GSEA
/// @export
#[extendr]
fn gsea_rust(
    min_overlap: Robj,
    max_overlap: Robj,
    sets: Robj,
    parts: Robj,
    analytes: Robj,
    ranks: Robj,
) -> List {
    // webgestalt_lib::methods::gsea::
    let config = GSEAConfig {
        min_overlap: 15,
        max_overlap: 500,
        ..Default::default()
    };
    let mut gmt: Vec<Item> = Vec::new();
    let set_vec = sets.as_str_vector().unwrap();
    let parts_vec: Vec<Vec<String>> = parts
        .as_list()
        .unwrap()
        .iter()
        .map(|(_, x)| x.as_string_vector().unwrap())
        .collect();
    for (i, set) in set_vec.iter().enumerate() {
        gmt.push(Item {
            id: set.to_string(),
            url: String::default(),
            parts: parts_vec[i].clone(),
        })
    }
    let mut analyte_list: Vec<RankListItem> = Vec::new();
    let analyte_vec: Vec<&str> = analytes.as_str_vector().unwrap();
    let ranks_vec: Vec<f64> = ranks.as_real_vector().unwrap();
    for (i, analyte) in analyte_vec.iter().enumerate() {
        analyte_list.push(RankListItem {
            analyte: analyte.to_string(),
            rank: ranks_vec[i],
        })
    }
    let res = webgestalt_lib::methods::gsea::gsea(analyte_list, gmt, config); // TODO: Convert
                                                                              // dataframe to GMT
    let mut fdr: Vec<f64> = Vec::new();
    let mut p: Vec<f64> = Vec::new();
    let mut leading_edge: Vec<i32> = Vec::new();
    let mut gene_sets: Vec<String> = Vec::new();
    let mut es: Vec<f64> = Vec::new();
    let mut nes: Vec<f64> = Vec::new();
    let mut running_sum: Vec<Robj> = Vec::new();
    for row in res {
        fdr.push(row.fdr);
        p.push(row.p);
        leading_edge.push(row.leading_edge);
        gene_sets.push(row.set);
        es.push(row.es);
        nes.push(row.nes);
        running_sum.push(Robj::from(row.running_sum));
    }
    list!(
        fdr = fdr,
        p_val = p,
        ES = es,
        NES = nes,
        leading_edge = leading_edge,
        gene_sets = gene_sets,
        running_sum = running_sum,
    )
}

/// Fill relation data frame for GSEA input
///
/// Fill 1 for gene in gene set
///
/// See https://github.com/extendr/extendr/issues/612 for how to export DataFrame
///
/// ## Diagram
/// ```shell
///         Gene Sets
///       ┌───────────┐  First column named 'gene' containing gene name
///       │A0100110100│  1 = in set
/// Genes │B0100101000│  0 = not in set
///       │C1011101001│  Due to limitiations with extendr-api v 0.6.0,
///       └───────────┘  function returns a list, and the R package will
///                      add the first 'gene' column
/// ```
/// @param gmt A Data Frame with geneSet and gene columns from the GMT file
/// @param genes A vector of genes
/// @param gene_sets A vector of gene sets
///
/// @return A Data Frame with the first column of gene and 1 or 0 for other columns of gene sets.
/// @author John Elizarraras
/// @keywords internal
/// @export
#[extendr]
pub fn fill_input_data_frame(gmt: Robj, genes: Robj, gene_sets: Robj) -> List {
    let genes_vec = genes.as_string_vector().unwrap();
    let gene_set_vec = gene_sets.as_string_vector().unwrap();
    let mut value_array = Array2::zeros((genes_vec.len(), gene_set_vec.len()));
    let mut gene_index: FxHashMap<&String, usize> = FxHashMap::default();
    let mut set_index: FxHashMap<&String, usize> = FxHashMap::default();
    let gmt_set: Vec<String> = gmt.index("geneSet").unwrap().as_string_vector().unwrap();
    let gmt_gene: Vec<String> = gmt.index("gene").unwrap().as_string_vector().unwrap();
    for (i, val) in genes_vec.iter().enumerate() {
        gene_index.insert(val, i);
    }
    for (i, val) in gene_set_vec.iter().enumerate() {
        set_index.insert(val, i);
    }
    for i in 0..gmt_set.len() {
        value_array[[gene_index[&gmt_gene[i]], set_index[&gmt_set[i]]]] = 1;
    }
    let mut gene_set_val: Vec<Vec<i32>> = Vec::new();
    // gene_set_val.push(genes_vec.into_iter().map(|x| SafeTypes::String(x)).collect());
    for i in 0..value_array.len_of(ndarray::Axis(1)) {
        gene_set_val.push(value_array.column(i).to_vec())
    }
    // gene_set_vec.insert(0, String::from("gene"));
    // Construct DataFrame in R. Create list for now.
    List::from_names_and_values(gene_set_vec, gene_set_val).unwrap()
    // data_frame!(x = 1)
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod WebGestaltR;
    fn rust_hello_world;
    fn fill_input_data_frame;
    fn gsea_rust;
}
