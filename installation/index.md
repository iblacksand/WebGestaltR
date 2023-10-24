---
layout: default
title: Installation
nav_order: 2
---

# Installation

With the new version of WebGestaltR, we now require Rust to be installed on your system, as the core computations are written in a new Rust Library. This allows for up to a 15x speedup in GSEA. We can also process more datasets for ORA with less memory usage.

## Requirements

- Rust 1.66.0 or later
- R 4.0.0 or later (Windows requires 4.2+)

{: .note}

Apple silicon users (M1, M2, etc) need to have the arm64 version of R installed. You can check what platform you have installed by running `R --version` in your terminal. If you see `x86_64` in the output, please download the arm64 version from [the R-Project website](https://cran.r-project.org/index.html).

## Steps

1. Install Rust according to the instructions here: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Install the R package from CRAN for the latest version of WebGestaltR

```R
install.packages("WebGestaltR")
```

If you have problems installing the package, please [create an issue](https://github.com/iblacksand/WebGestaltR/issues/new?assignees=iblacksand&labels=Installation&projects=&template=installation-issue.md&title=).
