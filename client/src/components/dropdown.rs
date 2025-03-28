use yew::prelude::*;
use std::rc::Rc;

#[derive(Properties, PartialEq)]
pub struct DropdownProps {
    pub table_id: String,
    pub is_open: bool,
    pub delete_table: Callback<MouseEvent>,
    pub set_renaming_id: Callback<Option<String>>,
}

#[function_component]
pub fn Dropdown(props: &DropdownProps) -> Html {
    let DropdownProps { table_id, is_open, delete_table, set_renaming_id } = props;

    
    let on_rename_click = {
        let set_renaming_id = set_renaming_id.clone();
        let table_id = table_id.clone();
        Callback::from(move |_| set_renaming_id.emit(Some(table_id.clone())))
    };

    html! {
        if *is_open {
            <div id={format!("dropdown-{}", table_id)} class="absolute flex-col justify-end py-1 mb-2 space-y-2 bg-white border border-gray-100 rounded-lg shadow-sm dark:bg-gray-700 dark:border-gray-600 top-4 right-1 z-50">
                <ul class="text-sm text-gray-500 dark:text-gray-300">
                    <li>
                        <div class="flex space-x-2 items-center px-5 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white"
                            onclick={on_rename_click}
                            
                        >
                            <i class="fas fa-edit"></i>
                            <span class="text-sm font-medium">{"Rename"}</span>
                        </div>
                    </li>
                    <li>
                        <div 
                            class="cursor-pointer flex space-x-2 items-center px-5 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white"
                        >
                            <i class="fa fa-clone"></i>
                            <span class="text-sm font-medium">{"Duplicate"}</span>
                        </div>
                    </li>
                    <li>
                        <div onclick={delete_table.clone()} class="flex space-x-2 items-center px-5 py-2 cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white">
                            <i class="fas fa-trash text-red-500"></i>
                            <span class="text-sm font-medium text-red-500">{"Delete"}</span>
                        </div>
                    </li>
                </ul>
            </div>
        }
    }
}


#[derive(Properties)]
pub struct ColumnDropdownProps {
    pub column_index: usize,
    pub is_open: bool,
    pub set_col_id: Callback<Option<String>>,
    pub insert_column: Rc<dyn Fn(usize, &str)>,
    pub delete_column: Rc<dyn Fn(usize)>,
}

impl PartialEq for ColumnDropdownProps {
    fn eq(&self, other: &Self) -> bool {
        self.column_index == other.column_index &&
        self.is_open == other.is_open &&
        self.set_col_id == other.set_col_id
    }
}

#[function_component(ColumnDropdown)]
pub fn column_dropdown(props: &ColumnDropdownProps) -> Html {
    let ColumnDropdownProps { column_index, is_open, set_col_id, insert_column, delete_column } = props;

    let on_rename_click = {
        let set_col_id = set_col_id.clone();
        let col_id = column_index.clone();
        Callback::from(move |_| set_col_id.emit(Some(col_id.clone().to_string())))
    };
    
    let on_insert_left_click = {
        let insert_column = insert_column.clone();
        let col_id = column_index.clone();
        Callback::from(move |_| insert_column(col_id, "left"))
    };

    let on_insert_right_click = {
        let insert_column = insert_column.clone();
        let col_id = column_index.clone();
        Callback::from(move |_| insert_column(col_id, "right"))
    };

    let on_delete_click = {
        let delete_column = delete_column.clone();
        let col_id = column_index.clone();
        Callback::from(move |_| delete_column(col_id))
    };

    html! {
        if *is_open {
            <div id={format!("speed-dial-menu-dropdown-alternative-square-{}", column_index)} class="flex flex-col w-40 justify-end py-1 mb-4 space-y-2 bg-white border border-gray-100 rounded-lg shadow-sm dark:bg-gray-700 dark:border-gray-600 absolute top-8 left-4 z-50">
                <ul class="text-sm text-gray-500 dark:text-gray-300">
                    <li>
                        <div onclick={on_rename_click} class="flex space-x-2 items-center px-5 py-2 border-b border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white">
                            <i class="fas fa-pencil"></i>
                            <span style="text-transform: none;" class="text-sm font-medium">{"Edit field"}</span>
                        </div>
                    </li>
                    <li>
                        <div onclick={on_insert_left_click} class="flex space-x-2 items-center px-5 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white">
                            <i class="fas fa-arrow-left"></i>
                            <span style="text-transform: none;" class="text-sm font-medium">{"Insert left"}</span>
                        </div>
                    </li>
                    <li>
                        <div onclick={on_insert_right_click} class="flex space-x-2 items-center px-5 py-2 border-b border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white">
                            <i class="fas fa-arrow-right"></i>
                            <span style="text-transform: none;" class="text-sm font-medium">{"Insert right"}</span>
                        </div>
                    </li>
                    <li>
                        <div onclick={on_delete_click} class="flex space-x-2 items-center px-5 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white">
                            <i class="fas fa-trash text-red-500"></i>
                            <span style="text-transform: none;" class="text-sm font-medium text-red-500">{"Delete"}</span>
                        </div>
                    </li>
                </ul>
            </div>
        }
    }
}

