# Barcode Generator

generates a dataset containing all permutations of valid characters in Code39 barcode format and its corresponding barcode in binary.

- phf: Used to create a static map between character and its corresponding barcode
- itertools: used to create permutations of all keys in the char:barcode map
- rayon: parallelize generation of barcodes 
- compress_io: Buffered write to a compressed file since a size 6 barcode is over 200gb
