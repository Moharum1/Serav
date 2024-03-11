use std::collections::HashMap;
use std::path::PathBuf;

pub type TermFreq = HashMap::<String, usize>;
pub type TermFreqIndex = HashMap<PathBuf, TermFreq>;

pub mod indexer
{
    use std::fs::File;
    use std::{fs, io};
    use std::path::{Path, PathBuf};
    use xml::EventReader;
    use xml::reader::XmlEvent::Characters;
    use crate::indexer::{TermFreq, TermFreqIndex};
    use crate::token::Lexer;

    /** read_xml_file : A function that takes a file path to xml file and return String content
    * file_path: The path of the file
    */
    pub(super) fn read_xml_file(file_path : &PathBuf) -> io::Result<String>{
        let file = File::open(file_path)?;

        // Apply Parser
        let parser = EventReader::new(file);
        let mut content_buffer = String::new();
        for event in parser.into_iter()
        {
            match event {
                Ok(Characters(content)) => {
                    content_buffer.push_str(&content);
                    content_buffer.push_str(" ")
                },
                _ => continue,
            }
        }
        Ok(content_buffer)
    }

    pub fn get_dir_freq_table(dir : &str) -> TermFreqIndex
    {
        let mut freq_table_indexer = TermFreqIndex::new();
        let all_files = fs::read_dir(dir).unwrap();

        for file in all_files.map(|content| {content.unwrap().path()})
        {
            let content: Vec<char> = read_xml_file(&file).expect("ERROR").chars().collect();
            let lexer = Lexer::new(content.as_slice());
            let mut freq_table = TermFreq::new();

            for token in lexer.into_iter()
            {
                let token_content = token.iter().collect::<String>();

                if let Some(count) = freq_table.get_mut(&token_content)
                {
                    *count += 1;
                }else {
                    freq_table.insert(token_content,1);
                }
            }

            freq_table_indexer.insert(file, freq_table);
        }
        return freq_table_indexer;
    }

    pub fn serialize_freq_table_to_json(object: &TermFreqIndex)
    {
        let index_path = Path::new("src/engine/storage.json");
        if index_path.exists()
        {
            println!("Exitst");
        } else {
            println!("Saving .............");
            let index_file = File::create(index_path).unwrap();
            serde_json::to_writer(index_file, object).expect("File Saved");
        }

    }

    pub fn deserialize_frequency_table() -> TermFreqIndex
    {
        let index_path = Path::new("src/engine/storage.json");
        return if index_path.exists()
        {
            println!("Reading Index File ..................");
            let index_file = File::open(index_path).unwrap();
            let tf_index: TermFreqIndex = serde_json::from_reader(index_file).unwrap();
            println!("TF index contains {:?}", tf_index.len());
            tf_index
        } else {
            TermFreqIndex::new()
        }
    }
}
