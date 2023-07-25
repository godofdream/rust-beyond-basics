use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

// Struct to represent a section in the INI file
#[derive(Debug, PartialEq)]
struct Section {
    name: String,
    properties: HashMap<String, String>,
}


impl Section {
    fn new(name: String) -> Self {
        Section {
            name,
            properties: HashMap::new(),
        }
    }


    fn insert_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }


    fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }


    fn iter(&self) -> std::collections::hash_map::Iter<String, String> {
        self.properties.iter()
    }
}


// Struct to represent the INI file
#[derive(Debug)]
struct IniFile {
    sections: Vec<Section>,
}


impl IniFile {
    fn new() -> Self {
        IniFile {
            sections: Vec::new(),
        }
    }


    fn add_section(&mut self, section: Section) {
        self.sections.push(section);
    }


    fn section(&self, section_name: &str) -> Option<&Section> {
        self.sections.iter().find(|s| s.name == section_name)
    }


    fn iter_sections(&self) -> std::slice::Iter<Section> {
        self.sections.iter()
    }
}


// Parse the INI file
fn parse_ini_file(file_path: &Path) -> io::Result<IniFile> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let ini_file = reader.lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .fold(IniFile::new(), |mut ini_file, line| {
            if line.starts_with('[') && line.ends_with(']') {
                ini_file.add_section(Section::new(line[1..line.len() - 1].to_string()));
            } else {
                match ini_file.sections.last_mut() {
                    Some(section) => {
                        if let Some(index) = line.find('=') {
                            let (key, value) = line.split_at(index);
                            let value = value[1..].trim().to_string();
                            section.insert_property(key.trim().to_string(), value);
                        }
                    }
                    None => panic!()
                }
            }
            ini_file
        });
    Ok(ini_file)

    /* First approach with scan() was buggy, as last section will not be returned
     * -> See implementation with fold() above

            .scan(None, |current_section, line| {
            if line.starts_with('[') && line.ends_with(']') {
                let result = current_section.clone();
                *current_section = Some(Section::new(line[1..line.len() - 1].to_string()));
                return result;
            } else {
                if let Some(section) = current_section {
                    // Parse key-value pairs within the current section
                    if let Some(index) = line.find('=') {
                        let (key, value) = line.split_at(index);
                        // println!("{} = {} ", key, value);
                        let value = value[1..].trim().to_string();
                        section.insert_property(key.trim().to_string(), value);
                    }
                }
                None
            }
        }).collect();

    */



    /* Original implementation without iterators

    for line in reader.lines() {
        if let Ok(line) = line {
            let line = line.trim();


            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }


            if line.starts_with('[') && line.ends_with(']') {
                // Found a new section
                let section_name = line[1..line.len() - 1].to_string();
                // println!("found: {}", &section_name);
                if let Some(section) = current_section {
                    ini_file.add_section(section);
                }
                current_section = Some(Section::new(section_name));
            } else if let Some(section) = current_section.as_mut() {
                // Parse key-value pairs within the current section
                if let Some(index) = line.find('=') {
                    let (key, value) = line.split_at(index);
                    // println!("{} = {} ", key, value);
                    let value = value[1..].trim().to_string();
                    section.insert_property(key.trim().to_string(), value);
                }
            }
        }
    }
    if let Some(section) = current_section {
        ini_file.add_section(section);
    }

    */
}


fn main() {
    let file_path = Path::new("/home/hoth/.dnsverw.cfg");
    let ini_file = parse_ini_file(file_path).expect("No valid ini file");


    // Accessing the key-value pairs in a section
    if let Some(section) = ini_file.section("hostdb") {
        for (key, value) in section.iter() {
            println!("{} = {}", key, value);
        }
    }


    // Accessing all sections
    for section in ini_file.iter_sections() {
        println!("[{}]", section.name);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let file_path = Path::new("./tests/test1.ini");
        let ini_file = parse_ini_file(file_path).unwrap();
        let mut s = Section::new("section".to_string());
        s.insert_property("key".to_string(), "value".to_string());
        if let Some(s2) = ini_file.section("section") {
            assert_eq!(&s, s2);
        }
    }
}