use csv::{ReaderBuilder, StringRecord};
use std::collections::{HashMap};
use std::{fs};

const FILENAME: &str = "history.csv";
const FIRST_TAG: &str = "START";

#[derive(Debug)]
struct HistoryData {
    data_type: String,
    tag: String,
    text: String,
    life: i32,
    options: Vec<HistoryData>,
}

impl HistoryData {
    fn new(row: StringRecord) -> HistoryData{
        let life = row.get(3).unwrap().trim();
        let life : i32 = life.parse().unwrap_or(0);
        return HistoryData {
            data_type:row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            life: life,
            options: vec![],
        };
    }
}



fn main() {
    let mut life = 100;
    let mut actual_tag = FIRST_TAG;

    let mut last_record: String = "".to_string();

    let mut history_data: HashMap<String, HistoryData> = HashMap::new();

    let content = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        let dato = HistoryData::new(result);
        if dato.data_type == "SITUATION" {
            let record_tag = dato.tag.clone();
            history_data.insert(record_tag.clone(), dato);
            last_record = record_tag;
        }else if dato.data_type == "OPTION" {
            if let Some(data) = history_data.get_mut(&last_record) {
                (*data).options.push(dato);
            }
        }
    }

    // Game Loop
    loop {
        println!("You have {} life points", life);
        
        if let Some(data) = history_data.get(actual_tag){
            println!("{}", data.text);

            for (index, option) in data.options.iter().enumerate() {
                println!("[{}] {}", index, option.text);
            }

            let mut selection = String::new();
            std::io::stdin().read_line(&mut selection).unwrap();
            let selection = selection.trim().parse().unwrap_or(99);

            if let Some(option_selected) = &data.options.get(selection){
                actual_tag = &option_selected.tag;
            }else{
                println!("Invalid Option!");
            }


            life += data.life;
            println!("");
        }else{
            break;
        }

        if life <= 0{
            println!("You have died!");
            break;
        }

    }
}
