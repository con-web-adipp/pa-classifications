use crate::analyze::input::File;
use crate::analyze::output::{Bookmark, Output};
use std::collections::HashMap;

use pa_classifications::{find_xlsx_files, handle_xlsx_file};
use std::io::Write;

mod analyze;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = skip_bom::SkipEncodingBom::new(skip_bom::BomType::all(), std::io::stdin());
    let input: Vec<File> = serde_json::de::from_reader(reader)?;

    let mut bookmarks: Vec<Bookmark> = vec![];

    let plugin_dir = match input.first() {
        Some(f) => f.output_directory.as_str(),
        None => return Err("No input data provided by Analyze".into()),
    };

    let mut hashmap: HashMap<String, Vec<(String, String)>> = HashMap::new();

    let xlsx_files = find_xlsx_files(plugin_dir)?;
    if xlsx_files.is_empty() {
        return Err(format!("Failed to find .xlsx file in {:?}", plugin_dir).into());
    }

    for xlsx_file in xlsx_files {
        match handle_xlsx_file(xlsx_file.as_str(), &mut hashmap) {
            Ok(_) => {}
            Err(e) => eprintln!("Error while handling xlsx file {}: {}", xlsx_file, e),
        };
    }
    if hashmap.is_empty() {
        return Err(format!("No classification data found in {:?}", plugin_dir).into());
    }

    for file in input {
        if let Some(classifications) = hashmap.get(file.md5.as_str()) {
            for (class, percentage) in classifications {
                let path = format!("pa-classifications/{}", *class);
                let bm = Bookmark::new(file.sha1.as_str(), path.as_str()).with_comment(percentage);
                bookmarks.push(bm);
            }
        };
    }

    let output = Output {
        custom_properties: vec![],
        bookmarks,
    };
    serde_json::to_writer(std::io::stdout(), &output)?;
    std::io::stdout().write_all("\n".as_bytes())?;

    Ok(())
}
