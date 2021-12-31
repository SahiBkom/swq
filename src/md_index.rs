use crate::model::Model;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Eq, PartialEq, Hash)]
struct IdName(u32, String);

impl IdName {
    pub fn new(s: String) -> Result<IdName, ()> {
        if let Some((id, name)) = s.split_once("-") {
            Ok(IdName(id.parse::<u32>().map_err(|_| ())?, name.to_string()))
        } else {
            Err(())
        }
    }
}

pub struct MdIndex {
    map: HashMap<IdName, Vec<IdName>>,
}

impl MdIndex {
    pub fn create(data: &Vec<Model>) -> MdIndex {
        let mut map: HashMap<IdName, Vec<IdName>> = HashMap::new();
        for d in data {
            if d.path.is_empty() {
                continue;
            }

            if let Some((a, b)) = d.path.split_once('/') {
                if let (Ok(a), Ok(b)) = (IdName::new(a.to_string()), IdName::new(b.to_string())) {
                    map.entry(a).or_insert(Vec::new()).push(b);
                }
            }
        }

        MdIndex { map }
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, filename: P) -> Result<(), ()> {
        let mut md = String::new();
        md.push_str("# Scouting Wetenschap Quiz\n");
        md.push_str("Deze index is automaties gegenereerd\n");
        for (key, values) in &self.map {
            md.push_str(&format!("## {}\n", Self::uppercase_first_letter(&key.1)));
            for v in values {
                md.push_str(&format!(
                    "- [{} ({})]({:03}-{}/{:03}-{}.md)\n",
                    Self::uppercase_first_letter(&v.1),
                    key.0 * 100 + v.0,
                    key.0,
                    key.1,
                    v.0,
                    v.1
                ));
            }
        }

        let mut file = File::create(filename).map_err(|_| ())?;
        write!(file, "{}", md);
        Ok(())
    }

    fn uppercase_first_letter(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}
