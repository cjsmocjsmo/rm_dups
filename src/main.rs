use std::fs;
// use std::fs::rename;
use std::path::Path;
// use std::sync::mpsc::channel;
// use std::time::Instant;
// use threadpool::ThreadPool;
use serde::Deserialize;
use serde::Serialize;
pub mod walk_dirs;
// pub mod dedup;


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransDupsEntry {
    pub jsonfilename: String,
    pub filename: String,
    pub httpfilename: String,
    pub duplicates: Vec<DupStruct>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DupStruct {
    pub strdups: String,
    pub httpdups: String,
}

fn main() {

    let url2 = "/media/pipi/0123-4567/oldjpg_backup/".to_string();
    let url3 = "/media/pipi/0123-4567/ToRemove/".to_string();
    let url4 = "/media/pipi/USB01/DeDuped1/".to_string();

    let json_list = walk_dirs::walk_dir(url3.clone());
    for json_file in json_list.clone() {
        let json_content = fs::read_to_string(json_file.clone()).expect("Unable to read file");
        let dups_entry: TransDupsEntry = serde_json::from_str(&json_content).unwrap();
        // println!("dups_entry: {:#?}", dups_entry);
        let keep_file = dups_entry.filename.clone();
        println!("keep_file: {:#?}", keep_file);
        let keep_file_exists = Path::new(&keep_file).exists();
        if keep_file_exists {
            let keep_file_parts = keep_file.split("/").collect::<Vec<&str>>();
            let fname = keep_file_parts.last().unwrap().to_string();
            let newfilename = url4.clone() + &fname;
            println!("newfilename: {:#?}", newfilename);
            fs::copy(keep_file.clone(), newfilename.clone()).expect("Unable to copy file");
            let dups = dups_entry.duplicates.clone();
            for dup in dups {
                let dup_url = url2.clone() + &dup.strdups.to_string();
                let dup_url_exists = Path::new(&dup_url).exists();
                if dup_url_exists {
                    let _rm_dup = fs::remove_file(dup_url.clone()).expect("Unable to delete file");
                    println!("Deleted: \n\t{}", dup_url.clone());
                } else {
                    println!("File does not exist: \n\t{}", dup_url.clone());
                }
            }
            fs::remove_file(keep_file.clone()).expect("Unable to delete keep file");
        } else {
            println!("File does not exist: \n\t{}", keep_file.clone());
        }
    }
}
