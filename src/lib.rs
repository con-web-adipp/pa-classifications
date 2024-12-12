use calamine::{open_workbook, HeaderRow, RangeDeserializerBuilder, Reader, Xlsx};
use regex::Regex;
use std::collections::HashMap;

pub mod analyze;

pub fn find_xlsx_files(folder_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut xlsx_files = vec![];
    let entries = std::fs::read_dir(folder_path)?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "xlsx" {
                    xlsx_files.push(path.to_string_lossy().into_owned());
                }
            }
        }
    }
    Ok(xlsx_files)
}

pub fn handle_xlsx_file(
    path: &str,
    hashmap: &mut HashMap<String, Vec<(String, String)>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut excel: Xlsx<_> = open_workbook(path)?;
    let sheet = match excel
        .with_header_row(HeaderRow::Row(1))
        .worksheet_range("Bilder")
    {
        Ok(sheet) => sheet,
        Err(_) => match excel.worksheet_range("Images") {
            Ok(sheet) => sheet,
            Err(_) => return Err("Failed to find sheet 'Images' or 'Bilder'.".into()),
        },
    };

    let iter =
        RangeDeserializerBuilder::with_headers(&["MD5", "Classifications"]).from_range(&sheet)?;

    for result in iter {
        let (md5, classifications): (String, String) = result?;
        if !classifications.is_empty() {
            let classifications = split_classifications(&classifications);
            hashmap.insert(md5, classifications);
        }
    }

    Ok(())
}

fn split_classifications(classifications: &str) -> Vec<(String, String)> {
    classifications
        .split("\r\n")
        .map(|s| s.replace("_x000D_", ""))
        .filter(|s| !s.is_empty())
        .map(|s| split_class_percentage(s.as_str()))
        .collect()
}

fn split_class_percentage(classification: &str) -> (String, String) {
    eprintln!("{:?}", classification);
    let re = Regex::new(r"(.*)\s\((\d+%)\)").unwrap();
    if let Some(captures) = re.captures(classification) {
        let class = captures.get(1).map_or("", |m| m.as_str());
        let percentage = captures.get(2).map_or("", |m| m.as_str());
        (class.to_string(), percentage.to_string())
    } else {
        ("".to_string(), "".to_string())
    }
}
