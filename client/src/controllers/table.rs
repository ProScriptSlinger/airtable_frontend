use bounce::Atom;
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::apis::table_api::SimpleTableItem;
use yew::functional::UseStateHandle;

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

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::Text => "text".to_string(),
            ContentType::Number => "number".to_string(),
            ContentType::SingleSelect => "single_select".to_string(),
            ContentType::MultipleSelect => "multiple_select".to_string(),
            ContentType::Date => "date".to_string(),
            ContentType::Url => "url".to_string(),
            ContentType::Checkbox => "checkbox".to_string(),
            ContentType::Attachment => "attachment".to_string(),
            ContentType::Subtask => "subtask".to_string(),
            // Handle other variants as strings...
        }
    }
}

impl From<UseStateHandle<String>> for ContentType {
    fn from(state: UseStateHandle<String>) -> Self {
        match &**state {
            "text" => ContentType::Text,
            "number" => ContentType::Number,
            "single_select" => ContentType::SingleSelect,
            "multiple_select" => ContentType::MultipleSelect,
            "date" => ContentType::Date,
            "url" => ContentType::Url,
            "checkbox" => ContentType::Checkbox,
            "attachment" => ContentType::Attachment,
            "subtask" => ContentType::Subtask,
            _ => ContentType::Text, // Default case (or consider returning an error)
        }
    }
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
            title: "Sample Table".to_string(),
            data: vec![
                vec![
                    Cell { content: vec!["Label".to_string()], content_type: ContentType::Text },
                    Cell { content: vec!["Number".to_string()], content_type: ContentType::Number },
                    Cell { content: vec!["Status".to_string()], content_type: ContentType::SingleSelect },
                    
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
                    title: "Sample Table".to_string(),
                    data: vec![
                        vec![
                            Cell { content: vec!["Label".to_string()], content_type: ContentType::Text },
                            Cell { content: vec!["Number".to_string()], content_type: ContentType::Number },
                            Cell { content: vec!["Status".to_string()], content_type: ContentType::SingleSelect },
                            
                        ],
                    ],
                }
            ]
        }
    }
}


