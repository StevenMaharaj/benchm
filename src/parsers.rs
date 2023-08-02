use serde::Deserialize;
use serde::Serialize;
use std::str::Lines;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub arg: Arg,
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arg {
    pub channel: String,
    pub inst_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub asks: [[String;4];1],
    pub bids: [[String;4];1],
    pub ts: String,
    pub seq_id: i64,
}

pub fn serde_parse(lines: Lines){
    for line in lines {
        let _root: Root = serde_json::from_str(line).unwrap();
    }
    // let root:Vec<Root> = lines.for_each(|line| serde_json::from_str(line).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_serde_parse() {
        let file_path = "data.txt";

        let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

        let lines = contents.lines();
        serde_parse(lines);
    }
}

