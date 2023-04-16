Extractor
=========

This is a extractor for the photos inside the doc files.

## Usage

fn main() {
    extr_photo("./test.doc", "./test").unwrap();
}

## Panics 
If Path to doc file is incorrect or path to photo folder is incorrect or
path to photo folder is occupied by another file or folder
