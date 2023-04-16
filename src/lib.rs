//! # Photo Extractor Crate
//! 
//! This crate extracts photo from doc files.


use std::fs;
use std::io::prelude::*;

/// Extracts photo from doc file
/// 
/// # Panics 
/// If Path to doc file is incorrect or path to photo folder is incorrect or
/// path to photo folder is occupied by another file or folder
/// 
/// # Arguments
/// 
/// * `path_doc` - Path to doc file
/// * `path_photo` - Path to photo folder where photos will be saved
/// 
/// # Examples 
///  
/// ```
/// use extractor_p::extr_photo;
/// 
///fn main() {
///     extr_photo("./test.doc", "./test").unwrap();
/// }
/// ```
/// 
///

pub fn extr_photo(path_doc: &str, path_photo: &str) -> std::io::Result<()> {
    let pat_jpg = b"\xff\xd8";
    let eof_jpg = b"\xff\xd9";
    let mut start = 0;
    let mut dir = false;
    let mut f_jpg = false;
    let mut f_png = false;
    let pat_png = b"\x89\x50\x4e\x47\x0d\x0a\x1a\x0a";
    let eof_png = b"\x49\x45\x4e\x44\xae\x42\x60\x82";
    let mut f = fs::File::open(path_doc)?;
    let medata = f.metadata()?;
    let length = medata.len();
    let mut num = 1;
    let mut byte_vec = vec![0; length as usize];
    f.read(&mut byte_vec)?;
    for (i, v) in byte_vec.iter().enumerate() {
        if i+1 == length as usize{
            break;
        }
        if !f_png && *v == pat_jpg[0] && byte_vec[i+1] == pat_jpg[1]{
            start = i;
            f_jpg = true;
        }
        if !f_jpg && *v == pat_png[0] && byte_vec[i+1] == pat_png[1] && byte_vec[i+2] == pat_png[2] && byte_vec[i+3] == pat_png[3] && byte_vec[i+4] == pat_png[4] && byte_vec[i+5] == pat_png[5] && byte_vec[i+6] == pat_png[6] && byte_vec[i+7] == pat_png[7]{
            start = i;
            f_png = true;
        }
        if *v == eof_jpg[0] && byte_vec[i+1] == eof_jpg[1] && f_jpg{
            if !dir{
                fs::create_dir(path_photo)?;
                dir = true;
            }
            let mut f = fs::File::create(format!("{}\\{}.jpg", path_photo, num))?;
            f.write(&byte_vec[start..i+2])?;
            num += 1;
            f_jpg = false;
        }  
        if f_png && eof_png[0] == *v && eof_png[1] == byte_vec[i+1] && eof_png[2] == byte_vec[i+2] && eof_png[3] == byte_vec[i+3] && eof_png[4] == byte_vec[i+4] && eof_png[5] == byte_vec[i+5] && eof_png[6] == byte_vec[i+6] && eof_png[7] == byte_vec[i+7] && f_png{
            if !dir{
                fs::create_dir(path_photo)?;
                dir = true;
            }
            let mut f = fs::File::create(format!("{}\\{}.png", path_photo, num))?;
            f.write(&byte_vec[start..i+8])?;
            num += 1;
            f_png = false;
        }
    }
    Ok(())
}

