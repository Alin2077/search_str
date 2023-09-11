use std::fs;
use std::path::{Path,PathBuf};

fn main() {
    let path: String = String::from("C:\\data_work\\studyWork\\testData\\dist");
    // let STR: String = String::from("智慧");

    let path_list = read_dir_fun(&Path::new(&path));
    // println!("{:#?}",path_list);
    for path in path_list {
        let line_list = find_str(&String::from("智慧"), &path);
        print!("{:#?}\t",path);
        println!("{:#?}",line_list);
    }

}

fn read_dir_fun(dir: &Path) -> Vec<PathBuf> {

    let mut files = vec![];

    //先判断是否是UTF-8流,不是则直接返回None

    let read_dir_result = match fs::read_dir(dir) {
        Ok(read_dir) => read_dir,
        Err(e) => panic!("read dir error: {}", e),
    };

    for entry in read_dir_result {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => panic!("list dir error: {}", e),
        };
        let path = entry.path();
        if path.is_dir() {
            files.extend(read_dir_fun(&path));
        } else {
            files.push(path.into());
        }

    }
    files
}

fn find_str(find_str: &str, path: &PathBuf) -> Option<Vec<u32>> {

    let mut result_vec = vec![];

    let read_result = match fs::read_to_string(path) {
        Ok(read_result) => read_result,
        Err(e) => {
            if e.is_invalid_utf8()  {
                   return None;
            } else {
                panic!("Unexpected error: {}", e);
            }
        },
    };

    for (index,value) in read_result.split(find_str).enumerate() {
        if value.contains(find_str) {
            result_vec.push(index as u32);
        }
    }

    if result_vec.is_empty() {
        return None;
    }
    Some(result_vec)
}

