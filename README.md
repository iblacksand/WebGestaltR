# WebGestaltR

> [!IMPORTANT]
> The new version of WebGesaltR requires Rust, which must be installed on your device prior to installing or updating the package from CRAN. See the installation section for more information.

WebGestalt R package is the R version of our well-known web application tool WebGestalt (www.webgestalt.org) that has on average 27,000 users from 140 countries and territories per year and has also been cited 371 in 2016. The advantage of this R package is that it can be easily integrated to other pipelines or simultaneously analyze multiple gene lists.

WebGestaltR function can perform popular enrichment analyses: ORA (Over-Representation Analysis), GSEA (Gene Set Enrichment Analysis) and NTA (Network Topology Analysis). Based on the user-uploaded gene list or gene list with scores (for GSEA method), WebGestaltR function will first map the gene list to entrez gene IDs and then summarize the gene list based on the GO (Gene Ontology) Slim data. After performing the enrichment analysis, WebGestaltR function also returns an user-friendly HTML report containing GO Slim summary and enrichment analysis result. If the functional categories have the DAG (directed acyclic graph) structure, the structure of the enriched categories can also be visualized in the report.

## New Changes

> [!INFO]
> Besides the change in installation, there should be no difference in how the R package performs for existing use-cases. If you experience any difference in results that are not due to the data-update, that is considered a bug. [Please report the changes you experience in a new issue](https://github.com/bzhanglab/WebGestaltR/issues/new/choose).

WebGestaltR's core was re-written in Rust, which dramatically increased performance, with up to 15x the speed of previous versions.

## Installation

Since WebGestaltR v1.0.0, Rust is used for core computations in the R package. Therefore, to install WebGestaltR, please download and install Rust from [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started). For Mac, Linux, or Unix users, Rust can be installed from the command line, and Windows users can download a GUI installer.

After installing Rust, you can install WebGestaltR with the following command:

```R
# install.packages("devtools") # run if needed
devtools::install_github("bzhanglab/WebGestaltR")
```

During installation, the Rust compiler will be called to build the computation library used by WebGestaltR. If you run into problems with installation of the new version, please [open a new issue](https://github.com/bzhanglab/WebGestaltR/issues/new/choose).
