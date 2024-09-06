use bounce::Atom;


#[derive(Atom, PartialEq, Clone)]
pub struct Table {
    pub name: String,
    pub data: Vec<Vec<String>>, // current selected table
}

impl Default for Table {
    fn default() -> Self {
        Table {
            name: "Table 1".to_string(),
            data: vec![
                vec!["Header 1".to_string(), "Header 2".to_string(), "Header 3".to_string()],
                vec!["Row 1, Cell 1".to_string(), "Row 1, Cell 2".to_string(), "Row 1, Cell 3".to_string()],
                vec!["Row 2, Cell 1".to_string(), "Row 2, Cell 2".to_string(), "Row 2, Cell 3".to_string()],
            ],
        }
    }
}

#[derive(Atom, PartialEq)]
pub struct Tables {
    pub tables: Vec<Table>
}

impl Default for Tables {
    fn default() -> Self {
        Tables {
            tables: vec![
                Table {
                    name: "Table 1".to_string(),
                    data: vec![
                        vec!["Header 1".to_string(), "Header 2".to_string(), "Header 3".to_string()],
                        vec!["Row 1, Cell 1".to_string(), "Row 1, Cell 2".to_string(), "Row 1, Cell 3".to_string()],
                        vec!["Row 2, Cell 1".to_string(), "Row 2, Cell 2".to_string(), "Row 2, Cell 3".to_string()],
                    ],
                },
                Table {
                    name: "Table 2".to_string(),
                    data: vec![
                        vec!["Header A".to_string(), "Header B".to_string(), "Header C".to_string()],
                        vec!["Row 1, Cell A".to_string(), "Row 1, Cell B".to_string(), "Row 1, Cell C".to_string()],
                        vec!["Row 2, Cell A".to_string(), "Row 2, Cell B".to_string(), "Row 2, Cell C".to_string()],
                    ],
                },
            ]
        }
    }
}