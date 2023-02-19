use itertools::free::join;
use itertools::Itertools;
use phf::phf_map;
use rayon::prelude::*;
// use std::fs::File;
// use std::io::{BufWriter, Error, Write};
use compress_io::{
    compress::CompressIo,
    compress_type::{CompressThreads, CompressType},
};
use std::io::{Error, Write};

// https://github.com/WhyNotHugo/python-barcode/blob/main/barcode/codex.py
const CODES: phf::Map<&str, &str> = phf_map! {
 "0"=>"101000111011101",
 "1"=>"111010001010111",
 "2"=>"101110001010111",
 "3"=>"111011100010101",
 "4"=>"101000111010111",
 "5"=>"111010001110101",
 "6"=>"101110001110101",
 "7"=>"101000101110111",
 "8"=>"111010001011101",
 "9"=>"101110001011101",
 "A"=>"111010100010111",
 "B"=>"101110100010111",
 "C"=>"111011101000101",
 "D"=>"101011100010111",
 "E"=>"111010111000101",
 "F"=>"101110111000101",
 "G"=>"101010001110111",
 "H"=>"111010100011101",
 "I"=>"101110100011101",
 "J"=>"101011100011101",
 "K"=>"111010101000111",
 "L"=>"101110101000111",
 "M"=>"111011101010001",
 "N"=>"101011101000111",
 "O"=>"111010111010001",
 "P"=>"101110111010001",
 "Q"=>"101010111000111",
 "R"=>"111010101110001",
 "S"=>"101110101110001",
 "T"=>"101011101110001",
 "U"=>"111000101010111",
 "V"=>"100011101010111",
 "W"=>"111000111010101",
 "X"=>"100010111010111",
 "Y"=>"111000101110101",
 "Z"=>"100011101110101",
 "-"=>"100010101110111",
 "."=>"111000101011101",
 " "=>"100011101011101",
 "$"=>"100010001000101",
 "/"=>"100010001010001",
 "+"=>"100010100010001",
 "%"=>"101000100010001",
};

const EDGE: &str = "100010111011101";

fn main() -> Result<(), Error> {
    (1..7).into_par_iter().for_each(|idx| {
        // let path = format!("lines{idx}.txt");
        // let mut output = BufWriter::new(File::create(path).unwrap());

        let mut output = CompressIo::new()
            .path(format!("barcodes_{idx}"))
            .ctype(CompressType::Zstd)
            .cthreads(CompressThreads::Set(4))
            .bufwriter()
            .unwrap();

        // let dataset = CODES.keys().permutations(idx).unique().map(|v| {
        //     format!(
        //         "{},{}0{}0{}",
        //         join(&v, ""),
        //         EDGE,
        //         join(
        //             v.iter()
        //                 .map(|x| CODES.get(x).unwrap())
        //                 .collect::<Vec<&&str>>(),
        //             "0"
        //         ),
        //         EDGE
        //     )
        // });
        // for d in dataset {
        //     writeln!(output, "{}", d)?;
        // }

        for perm in CODES.keys().permutations(idx) {
            writeln!(
                output,
                "{},{}0{}0{}",
                join(&perm, ""),
                EDGE,
                join(
                    perm.iter()
                        .map(|x| CODES.get(x).unwrap())
                        .collect::<Vec<&&str>>(),
                    "0"
                ),
                EDGE
            )
            .unwrap();
        }
    });

    Ok(())
}
