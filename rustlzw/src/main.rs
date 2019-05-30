/* LZW - Compression
 * Program to compress file with TXT , TIF/TIFF, and GIF extension
 * using LZW Compression algorithm
 * 
 * Pandyaka Aptanagi / 13517003
 */

// ========= Importing some utils ========= 
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
extern crate image_base64;

// ========== Read External File =========
fn read_file(v: &Vec<&str>) -> String {

    let mut temp: String = "".to_string();
    if v[1] == "txt" {
        temp = fs::read_to_string(v[0].to_owned()+".txt").expect("File not found");
    } else {
        temp = image_base64::to_base64(&(v[0].to_owned()+"."+&(v[1].to_owned())));
    }

    return temp.to_string()
}

// =========== Write External File after Compression =========
fn write_compress(text: String, filename: &Vec<&str>) -> std::io::Result<()> {

    let mut file = File::create(filename[0].to_owned()+".txt")?;
    let contents = filename[1].to_owned() + " " + &text;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

// =========== Write External File after deCompression =========
fn write_decompress(text: String, filename: Vec<&str>, ex: String) {

}

// =========== Make the dictionary (array of char) ==========
fn make_dict(opt: String) {

}

// ========== Function to Compress =========
fn do_compress(filename: Vec<&str>) {

}

// ========== Function to deCompress ==========
fn do_decompress(filename: Vec<&str>) {
    
}

// ========== Input validation =========
fn is_validquery(query: String) -> bool {
    return query == "compress" || query == "decompress";
}

fn is_validformat(format: String) -> bool {
    return format == "txt" || format == "gif" || format == "tif" || format == "tiff";
}

// ========== Main Function ==========
fn main() {

    let args: Vec<String> = env::args().collect();

    // Arguments validation
    if args.len() == 3 {
        let v: Vec<&str> = args[2].split(".").collect();
        let query = &args[1];
        let filename = &v[0];
        let formatfile = &v[1];
        if is_validquery(query.to_string()) && is_validformat(formatfile.to_string())  {
            if query == "compress" {
                let temp = read_file(&v);
                write_compress(temp, &v);
            } else {
                do_decompress(v)
            }
        } else {
            println!("Wrong format and or query : (")
        }
    } else {
        println!("Wrong number of argument(s) :(")
    }
}