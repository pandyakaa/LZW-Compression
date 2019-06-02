/* LZW - Compression
 * Program to compress file with TXT , TIF/TIFF, and GIF extension
 * using LZW Compression algorithm
 * 
 * Pandyaka Aptanagi / 13517003
 */

// ========= Importing some utils ========= 
use std::env;
use std::fs;
use std::char;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

extern crate image_base64;

// ========== Read External File =========
fn read_file(v: &Vec<&str>) -> String {

    let mut temp;
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
fn make_compressdict() -> HashMap<String,u32> {
    
    let mut h: HashMap<String, u32> = HashMap::new();
    for i in 0..256 {
        let c = char::from_u32(i).unwrap().to_string();
        h.insert(c,i);
    }
    
    return h
}

fn make_decompressdict() -> HashMap<u32,String> {

    let mut h: HashMap<u32, String> = HashMap::new();
    for i in 0..256 {
        let c = char::from_u32(i).unwrap().to_string();
        h.insert(i, c);
    }

    return h
}

// ========== Function to Compress =========
fn do_compress(filename: &Vec<&str>) {

    let mut dict = make_compressdict();
    let data = read_file(filename);

    let mut word = String::new();
    let mut res = String::new();
    for c in data.chars() {
        let wc = word.to_owned() + &c.to_string();
        if dict.contains_key(&wc) {
            word = wc
        } else {
            res = res + &dict.get(&word).unwrap().to_string() + "-";
            dict.insert(wc, dict.len() as u32);
            word = c.to_string();
        }
    }

    res = res + &dict.get(&word).unwrap().to_string();
    write_compress(res, &filename);
}

// ========== Function to deCompress ==========
fn do_decompress(filename: &Vec<&str>) {
    
    let mut dict = make_decompressdict();
    let data = read_file(filename);

    let mut v: Vec<&str> = data.split(" ").collect();
    let filetype = v[0];
    v = v[1..].to_vec();
    v = v[0].split("-").collect();
    let mut tempv: Vec<u32> = Vec::new();
    for val in &v {
        let myint: u32 = val.parse().unwrap();
        tempv.push(myint);
    }

    let mut res = String::new();
    let clonedict = dict.clone();
    let mut word = clonedict.get(&tempv[0]).unwrap();
    res = res + word;

    let mut entry = String::new();
    let tempv2: Vec<u32> = tempv[1..].to_vec();
    for k in &tempv2 {
        if dict.contains_key(&k) {
            entry = dict.get(&k).unwrap().to_string();
        } else if k == &(dict.len() as u32) {
            let c = word.chars().nth(0).unwrap().to_string();
            entry = word.to_string() + &c;
        }

        res = res + &entry.clone();

        let ch = entry.chars().nth(0).unwrap().to_string();
        let tempw = word.to_string() + &ch;

        dict.insert(dict.len() as u32, tempw);
        
        word = &entry.clone();
    }

    println!("{}",res );
    //write_decompress(res, filename.to_vec(), filetype.to_string());
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
        let formatfile = &v[1];
        if is_validquery(query.to_string()) && is_validformat(formatfile.to_string()) {
            if query == "compress" {
                do_compress(&v);
            } else {
                do_decompress(&v);
            }
        } else {
            println!("Wrong format and or query : (")
        }
    } else {
        println!("Wrong number of argument(s) :(")
    }
}