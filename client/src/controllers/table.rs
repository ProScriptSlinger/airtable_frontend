use bounce::Atom;
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::apis::table_api::SimpleTableItem;

// Assuming this is how your Attachment struct looks like
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attachment {
    // Define fields for Attachment here
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ContentType {
    Text,
    Number,
    SingleSelect,
    MultipleSelect,
    Date,
    Url,
    Checkbox,
    Attachment,
    Subtask,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Cell {
    pub content: Vec<String>,
    pub content_type: ContentType,
}

#[derive(Atom, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub id: String,
    pub title: String,
    pub data: Vec<Vec<Cell>>,
}

impl Default for Table {
    fn default() -> Self {
        Table {
            id: "abc".to_string(),
            title: "Table 1".to_string(),
            data: vec![
                vec![
                    Cell { content: vec!["Name".to_string()], content_type: ContentType::Text },
                    Cell { content: vec!["Age".to_string()], content_type: ContentType::Number },
                    Cell { content: vec!["Device Type".to_string()], content_type: ContentType::SingleSelect },
                    Cell { content: vec!["Current Owner".to_string()], content_type: ContentType::MultipleSelect },
                    Cell { content: vec!["Date".to_string()], content_type: ContentType::Date },
                    Cell { content: vec!["Url".to_string()], content_type: ContentType::Url },
                    Cell { content: vec!["Checkbox".to_string()], content_type: ContentType::Checkbox },
                    Cell { content: vec!["Attachments".to_string()], content_type: ContentType::Attachment },
                    Cell { content: vec!["Subtask".to_string()], content_type: ContentType::Subtask },
                ],
                vec![
                    Cell { content: vec!["Jose".to_string()], content_type: ContentType::Text },
                    Cell { content: vec!["24".to_string()], content_type: ContentType::Number },
                    Cell { content: vec!["Mac".to_string()], content_type: ContentType::SingleSelect },
                    Cell { content: vec!["Jose".to_string(), "Abel".to_string(), "Hardy".to_string()], content_type: ContentType::MultipleSelect },
                    Cell { content: vec!["2024-09-06".to_string()], content_type: ContentType::Date },
                    Cell { content: vec!["https://airtable.io".to_string()], content_type: ContentType::Url },
                    Cell { content: vec!["true".to_string()], content_type: ContentType::Checkbox },
                    Cell { content: vec!["a.jpg".to_string(), "b.mp4".to_string(), "c.pdf".to_string()], content_type: ContentType::Attachment },
                    Cell { content: vec!["task1".to_string(), "task2".to_string()], content_type: ContentType::Subtask },
                ],
            ],
        }
    }
}

// Serialization Function
pub fn serialize_table(table: &Table) -> SimpleTableItem {
    let data_str = serde_json::to_string(&table.data).unwrap();

    SimpleTableItem {
        id: Some(table.id.clone()),
        title: table.title.clone(),
        body: data_str,
    }
}

// Deserialization Function
pub fn deserialize_table(item: &SimpleTableItem) -> Table {
    let data: Vec<Vec<Cell>> = serde_json::from_str(&item.body).unwrap();

    Table {
        id: item.id.clone().expect("REASON"),
        title: item.title.clone(),
        data,
    }
}

#[derive(Atom, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tables {
    pub tables: Vec<Table>
}

impl Default for Tables {
    fn default() -> Self {
        Tables {
            tables: vec![
                Table {
                    id: "abcd".to_string(),
                    title: "Table 1".to_string(),
                    data: vec![
                        vec![
                            Cell { content: vec!["Name".to_string()], content_type: ContentType::Text },
                            Cell { content: vec!["Age".to_string()], content_type: ContentType::Number },
                            Cell { content: vec!["Device Type".to_string()], content_type: ContentType::SingleSelect },
                            Cell { content: vec!["Current Owner".to_string()], content_type: ContentType::MultipleSelect },
                            Cell { content: vec!["Date".to_string()], content_type: ContentType::Date },
                            Cell { content: vec!["Url".to_string()], content_type: ContentType::Url },
                            Cell { content: vec!["Checkbox".to_string()], content_type: ContentType::Checkbox },
                            Cell { content: vec!["Attachment".to_string()], content_type: ContentType::Attachment },
                            Cell { content: vec!["Subtask".to_string()], content_type: ContentType::Subtask },
                        ],
                        vec![
                            Cell { content: vec!["Jose".to_string()], content_type: ContentType::Text },
                            Cell { content: vec!["24".to_string()], content_type: ContentType::Number },
                            Cell { content: vec!["Mac".to_string()], content_type: ContentType::SingleSelect },
                            Cell { content: vec!["Jose".to_string(), "Abel".to_string(), "Hardy".to_string()], content_type: ContentType::MultipleSelect },
                            Cell { content: vec!["2024-09-06".to_string()], content_type: ContentType::Date },
                            Cell { content: vec!["https://airtable.io".to_string()], content_type: ContentType::Url },
                            Cell { content: vec!["true".to_string()], content_type: ContentType::Checkbox },
                            Cell { content: vec!["a.jpg".to_string(), "b.mp4".to_string(), "c.pdf".to_string()], content_type: ContentType::Attachment },
                            Cell { content: vec!["task1".to_string(), "task2".to_string()], content_type: ContentType::Subtask },
                        ],
                    ],
                }
            ]
        }
    }
}


