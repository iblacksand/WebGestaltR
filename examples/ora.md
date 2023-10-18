---
layout: default
title: ORA
parent: Examples
nav_order: 1
---

# Over-Representation Analysis (ORA) Examples

This is some examples of Over-Representation Analysis (ORA) using WebGestaltR.

```R
library("WebGestaltR")
geneFile <- "genelist.txt"
refFile <- "reference.txt"
gmtFile <- "combined_renamed.gmt"
outputDirectory <- getwd()
enrichResult <- WebGestaltR(
  enrichMethod = "ORA", organism = "hsapiens",
  enrichDatabaseFile = gmtFile, interestGeneFile = geneFile,
  enrichDatabaseType = "entrezgene",
  interestGeneType = "entrezgene", referenceGeneFile = refFile,
  referenceGeneType = "entrezgene", isOutput = TRUE,
  topThr = 100, sigMethod = "top",
  outputDirectory = outputDirectory, projectName = NULL, usekMedoid = TRUE, kMedoid_k = 50, useWeightedSetCover = FALSE, useAffinityPropagation = TRUE
)
```
