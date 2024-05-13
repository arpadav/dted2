# DTED Reader for Rust

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/dted2.svg)](https://crates.io/crates/dted2)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.56.0+-lightgray.svg)](#rust-version-requirements-msrv)
<!-- [![Latest Release](https://img.shields.io/github/v/release/arpadav/dted2)](https://github.com/arpadav/dted2) -->
<!-- [![Coverage Status](https://coveralls.io/repos/github/arpadav/dted2/badge.svg?branch=main)](https://coveralls.io/github/arpadav/dted2?branch=main) -->

<p align="center">
    <img width="500" src="https://arpadvoros.com/public/dted2.png" alt="dted2 surface" title="dted2 surface">
</p>

Refactor of [`dted`](https://github.com/fizyk20/dted), with updated version of [`nom`](https://crates.io/crates/nom), improved functionality, added features, fixes, and optimizations!

## Usage

```rust
use dted2::{ DTEDData, DTEDMetadata };

let data = DTEDData::read("dted_file.dt2").unwrap();
let metadata: DTEDMetadata = data.metadata;
// or can read just the header without the rest of the data
let metadata: DTEDMetadata = DTEDData::read_header("dted_file.dt2").unwrap();

// query elevation, returns None if out of bounds
let elevation = data.get_elevation(50.0, 10.0).unwrap();
```

## Description

The `dted2` crate is a Rust library designed to parse and handle [DTED (Digital Terrain Elevation Data)](https://www.dlr.de/de/eoc/Portaldata/60/Resources/dokumente/7_sat_miss/SRTM-XSAR-DEM-DTED-1.1.pdf) files. DTED files are a standard format used for storing raster elevation data, particularly for military and simulation applications. The data in DTED files is stored in a matrix of elevation points, representing the terrain's height above a given datum. This format supports several military and simulation applications including line-of-sight analysis, 3D visualization, and mission planning.

DTED data is organized into three levels of resolution:

* _Level 0_: Approximately 900 meters between data points.
* _Level 1_: Approximately 90 meters between data points.
* _Level 2_: Approximately 30 meters between data points.
Each level of DTED provides different details suitable for various precision requirements in applications.

## Features

* __Data Handling__: Efficient handling of large datasets with options to process only required sections of data for memory management.
* __Read Functionality__: Parse DTED files (`.dt0`, `.dt1`, `.dt2`) into usable data structures. ***Currently only `.dt2` files have been tested. `dt1` and `dt0` files should in theory work.***

## TODO

* __Geographic Processing__: Convert DTED raster data into geographic coordinates based on the WGS84 datum.
* __Additional DTED Header parsing__: Add support for additional header records. Currently both `DSI` and `ACC` records are being worked on, and only the standard `UHL` header is being read alongside the data.
