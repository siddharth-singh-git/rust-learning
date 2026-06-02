use std:fs; // for file handling
use std::io; // for input/output handling


fn main() {
    std::process::exit(decompress()); // call the decompress function and exit with its return value
}


fn decompress() -> i32 {

    let args : Vec<_> = std::env::args().collect(); // collect command line arguments into a vector

    if args.len() < 2 { // check if there are more than 2 arguments (program name and file name)
        println!("Usage: {} <filename>", args[0]); // print usage message to standard error
        return 1; // return with error code
    }

    let fname = std::path::Path::new(&args[1]); // create a Path object from the second argument (file name)
    let file = fs::File::open(&fname).unwrap(); // open the file and unwrap the result (panic if it fails)

    let mut archive = zip::ZipArchive::new(file).unwrap(); // create a ZipArchive object from the file and unwrap the result (panic if it fails)

    for i in 0..archive.len() { // iterate over the files in the archive
        let mut file= archive.by_index(i).unwrap(); // get the file by index and unwrap the result (panic if it fails)
        
        let mut outpath = match file.enclosed_name(){ // get the enclosed name of the file (the path inside the archive)
           Some(path)=> path.to_owned(), // if it exists, clone it
           None => continue, // if it doesn't exist, skip this file
        };

        {
            let comment =file.comment(); // get the comment of the file (not used in this code, but could be printed or logged)
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment); // print the file index and its comment if it's not empty
            }

        }


















        //*println!("Extracting {} to {}", file.name(), outpath.display()); // print a message about extracting the file

        if (&*file.name()).ends_with('/') { // check if the file is a directory (name ends with '/')
            fs::create_dir_all(&outpath).unwrap(); // create the directory and unwrap the result (panic if it fails)
        } else { // if it's not a directory
            if let Some(p) = outpath.parent() { // get the parent directory of the output path
                if !p.exists() { // check if the parent directory does not exist
                    fs::create_dir_all(&p).unwrap(); // create the parent directory and unwrap the result (panic if it fails)
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap(); // create a new file at the output path and unwrap the result (panic if it fails)
            io::copy(&mut file, &mut outfile).unwrap(); // copy the contents of the file from the archive to the new file and unwrap the result (panic if it fails)
        }
    }
    



}