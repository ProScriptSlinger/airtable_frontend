use yew::prelude::*;
use bounce::{use_atom_value, use_atom, UseAtomHandle};
use crate::controllers::table::{Table, Tables, Cell, ContentType, serialize_table, deserialize_table};
use crate::components::dropdown::ColumnDropdown;
use crate::components::drawer::Drawer;
use crate::components::multi_select::CustomMultiSelect;
use crate::components::subtask::{SubtaskComponent, Subtask, Status};
use crate::components::attachment::{AttachmentComponent, Attachment, FileType};
use crate::components::file_modal::{AddFileModal};
use crate::components::subtask_modal::{SubtaskModal};
use crate::apis::table_api::{fetch_table, post_table, put_table, delete_table, SimpleTableItem};
use wasm_bindgen::JsCast;
use web_sys;
use yew::functional::use_effect_with;
use serde::Deserialize;
use yew::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsError;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement,HtmlSelectElement , MouseEvent};
use std::collections::HashSet;

#[derive(Properties, PartialEq)]
pub struct DisplayCellIconProps {
    pub content_type: ContentType,
}

#[function_component(DisplayCellIcon)]
fn display_cell_icon(props: &DisplayCellIconProps) -> Html {
    let DisplayCellIconProps { content_type } = props;
    
    fn get_icon(content_type: &ContentType) -> &'static str {
        match content_type {
            ContentType::Text => "<i class=\"fas fa-font\" style=\"color: white; font-size: 15px;\"></i>",
            ContentType::Number => "<i class=\"fas fa-hashtag\" style=\"color: white; font-size: 15px;\"></i>",
            ContentType::SingleSelect => "<i class=\"fas fa-list\" style=\"color: white; font-size: 15px;\"></i>",
            ContentType::MultipleSelect => "<i class=\"fas fa-tasks\" style=\"color: white; font-size: 15px;\"></i>",
            ContentType::Date => "<i class=\"fas fa-calendar-alt\" style=\"color: white; font-size: 15px;\"></i>",
            ContentType::Url => "<i class=\"fas fa-link\" style=\"color: white; font-size: 15px;\"></i>",
            ContentType::Checkbox => "<i class=\"fas fa-check-square\" style=\"color: white; font-size: 15px;\"></i>",
            ContentType::Attachment => "<i class=\"fas fa-paperclip\" style=\"color: white; font-size: 15px;\"></i>",
            ContentType::Subtask => "<i class=\"fas fa-project-diagram\" style=\"color: white; font-size: 15px;\"></i>",
        }
    }
    
    let icon_html = get_icon(&content_type);

    html! {
        <div class="flex items-center gap-2">
            { Html::from_html_unchecked(AttrValue::from(icon_html)) }
        </div>
    }
}

// fetch tables
pub async fn fetch_tables() -> Result<Vec<Table>, JsError> {
    
    match fetch_table().await {
        Ok(fetched_data) => {
            let tables: Vec<Table> = fetched_data.iter()
            .map(|item| deserialize_table(item))
            .collect();
            // Log fetched data to the console (for debugging)
            // web_sys::console::log_1(&format!("Deserialized Tables: {:?}", tables).into());
            Ok(tables)
        },
        Err(e) => {
            // web_sys::console::error_1(&format!("Failed to save table: {:?}", e).into());
            Err(e)
        }
    }
}

// Create new table function
pub async fn create_table(table_title: String) -> Result<(), JsError> {
    let new_table =  Table {
        id: "default_id".to_string(),
        title: table_title,
        data: vec![
            vec![
                Cell { content: vec!["Label".to_string()], content_type: ContentType::Text },
                Cell { content: vec!["Number".to_string()], content_type: ContentType::Number },
                Cell { content: vec!["Status".to_string()], content_type: ContentType::SingleSelect },
            ],
            vec![
                Cell { content: vec!["".to_string()], content_type: ContentType::Text },
                Cell { content: vec!["".to_string()], content_type: ContentType::Number },
                Cell { content: vec!["".to_string()], content_type: ContentType::SingleSelect }
            ],
        ],
    };

    let table_item = serialize_table(&new_table);

    match post_table(table_item).await {
        Ok(_) => {
            web_sys::console::log_1(&"Table created successfully".into());
            Ok(())
        },
        Err(e) => {
            // web_sys::console::error_1(&format!("Failed to save table: {:?}", e).into());
            Err(e)
        }
    }
}

// Update table function by id
pub async fn update_table(new_table: Table) -> Result<(), JsError> {
    let table_item = serialize_table(&new_table);

    match put_table(table_item).await {
        Ok(_) => {
            web_sys::console::log_1(&"Table updated successfully".into());
            Ok(())
        },
        Err(e) => {
            // web_sys::console::error_1(&format!("Failed to save table: {:?}", e).into());
            Err(e)
        }
    }
}

// Delete table function by id
pub async fn delete_table_func(table_id: String) -> Result<(), JsError> {

    match delete_table(table_id).await {
        Ok(_) => {
            web_sys::console::log_1(&"Table deleted successfully".into());
            Ok(())
        },
        Err(e) => {
            // web_sys::console::error_1(&format!("Failed to save table: {:?}", e).into());
            Err(e)
        }
    }
}


#[derive(PartialEq, Properties, Clone)]
pub struct EditableCellProps {
    pub content_type: ContentType,
    pub content: Vec<String>,
    pub on_save: Callback<Vec<String>>,
    pub toggle_modal: Callback<String>
}


#[function_component(EditableCell)]
fn editable_cell(props: &EditableCellProps) -> Html {
    let content_state = use_state(|| props.content.clone());

    // Use effect to update content_state when props.content changes
    {
        let content_state = content_state.clone();
        let new_content = props.content.clone();
    
        use_effect_with(props.content.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                content_state.set(new_content);
            });
    
            || ()
        });
    }

    let is_editing = use_state(|| false);
    let props = props.clone();
    let EditableCellProps { content_type, content, on_save, toggle_modal } = props.clone();
    let is_editing = use_state(|| false);
    
    let toggle_editing = {
        let is_editing = is_editing.clone();
        move |_: MouseEvent| {
            is_editing.set(!*is_editing);
        }
    };
    
    let handle_change = {
        let content_state = content_state.clone();
        move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            content_state.set(vec![input.value()]);
        }
    };

    let handle_checkbox_change = {
        let content_state = content_state.clone();
        move |e: web_sys::Event| {
            let input: web_sys::HtmlInputElement = e.target().unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap();
            content_state.set(vec![if input.checked() { "true".to_string() } else { "false".to_string() }]);
        }
    };
    
    let handle_save = {
        let is_editing = is_editing.clone();
        let content_state = content_state.clone();
        let on_save = on_save.clone();
        // console::log_1(&"handle_save ----->".into());
        move |_: MouseEvent| {
            on_save.emit(content_state.to_vec());
            is_editing.set(false);
        }
    };
    let badge_classes = [
                            ("bg-blue-100", "text-blue-800", "dark:bg-blue-900", "dark:text-blue-300"),
                            ("bg-gray-100", "text-gray-800", "dark:bg-gray-700", "dark:text-gray-300"),
                            ("bg-red-100", "text-red-800", "dark:bg-red-900", "dark:text-red-300"),
                            ("bg-green-100", "text-green-800", "dark:bg-green-900", "dark:text-green-300"),
                            ("bg-yellow-100", "text-yellow-800", "dark:bg-yellow-900", "dark:text-yellow-300"),
                            ("bg-indigo-100", "text-indigo-800", "dark:bg-indigo-900", "dark:text-indigo-300"),
                            ("bg-purple-100", "text-purple-800", "dark:bg-purple-900", "dark:text-purple-300"),
                            ("bg-pink-100", "text-pink-800", "dark:bg-pink-900", "dark:text-pink-300")
                        ];

    let handle_select_change = {
        let content_state = content_state.clone();
        Callback::from(move |values: Vec<String>| {
            content_state.set(values);
        })
    };

    // fn get_file_type_icon(file_name: &str) -> &'static str {
    //     if file_name.ends_with(".jpg") || file_name.ends_with(".jpeg") || file_name.ends_with(".png") {
    //         "fas fa-image" // Image icon
    //     } else if file_name.ends_with(".mp4") || file_name.ends_with(".avi") {
    //         "fas fa-video" // Video icon
    //     } else if file_name.ends_with(".pdf") || file_name.ends_with(".docx") {
    //         "fas fa-file-alt" // Document icon
    //     } else {
    //         "fas fa-file" // Other files icon
    //     }
    // }

    
    let subtasks = vec![
        Subtask { 
            name: "Subtask 1".to_string(), 
            complete_rate: 50, 
            status: Status::InProgress 
        },
        Subtask { 
            name: "Subtask 2".to_string(), 
            complete_rate: 100, 
            status: Status::Completed 
        },
        Subtask { 
            name: "Subtask 3".to_string(), 
            complete_rate: 0, 
            status: Status::NotStarted 
        },
    ];

    let attachments = vec![
        Attachment { 
            name: "Attachment 1".to_string(), 
            uri: "test_uri".to_string(), 
            file_type: FileType::Image 
        },
        Attachment { 
            name: "Attachment 2".to_string(), 
            uri: "test_uri".to_string(), 
            file_type: FileType::Video 
        },
        Attachment { 
            name: "Attachment 3".to_string(), 
            uri: "test_uri".to_string(), 
            file_type: FileType::Document 
        },
    ];

    html! {
        if *is_editing {
            <div class="flex items-center gap-2">
                {
                    match content_type {
                        ContentType::Text | ContentType::Url => html! {
                            <input type="text"  value={content_state[0].clone()} oninput={handle_change}  class="min-w-32 bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg  block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
                        },
                        ContentType::SingleSelect  => html! {
                            <CustomMultiSelect multiple={false} handle_change={handle_select_change} value={(*content_state).clone()} />
                        },
                        ContentType::MultipleSelect => html! {
                            <CustomMultiSelect multiple={true} handle_change={handle_select_change} value={(*content_state).clone()} />
                        },
                        ContentType::Number => html! {
                            <input type="number"  value={content_state[0].clone()} oninput={handle_change} class="min-w-32 bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg  block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
                        },
                        ContentType::Date => html! {
                            <input type="date" value={content_state[0].clone()} oninput={handle_change}  class="bg-gray-50 max-w-50  border border-gray-300 text-gray-900 text-sm rounded-lg  block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
                        },
                        ContentType::Checkbox => html! {
                            <input id="default-checkbox" type="checkbox" checked={content_state[0] == "true"} onchange={handle_checkbox_change}  class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"/>
                        },
                        _ => html! {
                            <input type="checkbox" value={content_state[0].clone()} oninput={handle_change} class="min-w-20 bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg  block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
                        }
                    }
                }
                <button onclick={handle_save} class="text-white border border-gray-600 hover:bg-gray-600 hover:border-gray-800 focus:ring-4 focus:outline-none focus:ring-gray-300 font-medium rounded-lg text-sm w-full sm:w-auto p-1 text-center dark:border-gray-600 dark:hover:border-gray-700 dark:focus:ring-gray-800"><i class="fas fa-archive"></i></button>
            </div>
        } else {
            <div class="cursor-pointer w-fit-content ">
                {match content_type {
                    ContentType::SingleSelect | ContentType::MultipleSelect => {
                        html! {
                            <div onclick={toggle_editing} class="min-w-20 min-h-8">
                                { for content_state.iter().enumerate().map(|(index, value)| {
                                    let (bg_class, text_class, dark_bg_class, dark_text_class) = badge_classes[index % badge_classes.len()];
                                    html! {
                                        <span class={classes!(
                                            bg_class,
                                            text_class,
                                            dark_bg_class,
                                            dark_text_class,
                                            "text-xs",
                                            "font-medium",
                                            "me-2",
                                            "px-2.5",
                                            "py-0.5",
                                            "rounded",
                                            "cursor-pointer"
                                        )}>
                                            {value.clone()}
                                        </span>
                                    }
                                }) }
                            </div>
                        }
                    },
                    ContentType::Attachment => html! {
                        <AttachmentComponent attachments={attachments.clone()} toggle_modal={toggle_modal}/>
                    },
                    ContentType::Subtask => html! {
                        <SubtaskComponent subtasks={subtasks.clone()} toggle_modal={toggle_modal}/>
                    },
                    ContentType::Checkbox => html! {
                        <input onclick={toggle_editing} id="default-checkbox" type="checkbox" checked={content_state[0] == "true"} onchange={handle_checkbox_change}  class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"/>
                    },
                    _ => html! {
                        <div  onclick={toggle_editing} class="min-w-20 min-h-4 cursor-pointer">
                            { &(*content_state)[0] }
                        </div>
                    }
                }}
            </div>
        }
    }
}
#[function_component(Content)]
pub fn content() -> Html {
    
    let table = use_atom_value::<Table>();
    let tables = use_atom_value::<Tables>();
    let open_dropdowns = use_state::<Option<usize>, _>(|| None);
    let toggle_dropdown = {
        let open_dropdowns = open_dropdowns.clone();
        move |index: Option<usize>| {
            open_dropdowns.set(index);
        }
    };

    let table_handle: UseAtomHandle<Table> = use_atom::<Table>();
    let tables_handle: UseAtomHandle<Tables> = use_atom::<Tables>();

    // api functions
    let fetched_table = use_state(|| Rc::new(Vec::new()));
    {
        let table_handle = table_handle.clone();
        let tables_handle = tables_handle.clone();
        let fetched_table = fetched_table.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_table().await {
                    Ok(fetched_data) => {
                        let tables: Vec<Table> = fetched_data.iter()
                            .map(|item: &SimpleTableItem| deserialize_table(item))
                            .collect();
                        let tables_struct = Tables { tables: tables.clone() };
                        fetched_table.set(Rc::new(fetched_data.clone()));
                        tables_handle.set(tables_struct);
                        if let Some(first_table) = tables.first() {
                            table_handle.set(first_table.clone());
                        }
                    },
                    Err(_) => {
                        // Handle the error case
                    }
                }
            });

            || ()
        });
    }

    let open_drawer = use_state(|| false);

    let toggle_drawer = {
        let open_drawer = open_drawer.clone();
        move |_| {
            open_drawer.set(!*open_drawer);
        }
    };

    let update_cell_content = {
        let table_handle = table_handle.clone();
        let tables_handle = tables_handle.clone();
        let table = (*table).clone();

        Rc::new(move |row_idx: usize, col_idx: usize, new_content: Vec<String>| {
            let mut updated_table = table.clone();

            if row_idx < updated_table.data.len() && col_idx < updated_table.data[row_idx].len() {
                updated_table.data[row_idx][col_idx].content = new_content;
                table_handle.set(updated_table.clone());

                spawn_local({
                    let tables_handle = tables_handle.clone();
                    let table_handle = table_handle.clone();
                    async move {
                        match update_table(updated_table.clone()).await {
                            Ok(_) => {
                                match fetch_tables().await {
                                    Ok(res_tables) => {
                                        let tables_struct = Tables { tables: res_tables.clone() };
                                        tables_handle.set(tables_struct);
                                        // if let Some(first_table) = res_tables.first() {
                                        //     table_handle.set(first_table.clone());
                                        // }
                                        table_handle.set(updated_table.clone());
                                    },
                                    Err(_) => {}
                                }
                            },
                            Err(_) => {}
                        }
                    }
                });
            } else {
                web_sys::console::log_1(&format!("Index out of bounds: row_idx={}, col_idx={}", row_idx, col_idx).into());
            }
        })
    };

    let is_file_modal = use_state(|| false);

    let toggle_file_modal = {
        let is_file_modal = is_file_modal.clone();
        move |_| {
            is_file_modal.set(!*is_file_modal);
        }
    };

    let is_subtask_modal = use_state(|| false);

    let toggle_subtask_modal = {
        let is_subtask_modal = is_subtask_modal.clone();
        move |_| {
            is_subtask_modal.set(!*is_subtask_modal);
        }
    };

    let toggle_modal = {
        let toggle_subtask_modal = toggle_subtask_modal.clone();
        let toggle_file_modal = toggle_file_modal.clone();
        move |modal_type: String| {
            if modal_type == "file" {
                toggle_file_modal(());
            } else {
                toggle_subtask_modal(());
            }
        }
    };

    let col_id: UseStateHandle<Option<usize>> = use_state(|| None);
    let new_name: UseStateHandle<String> = use_state(|| "".to_string()); 
    let new_type: UseStateHandle<String> = use_state(|| "".to_string()); 

    let on_save_click = {
        let col_id = col_id.clone();
        let new_name = new_name.clone();
        let new_type = new_type.clone();
        let table_handle = table_handle.clone();
        let tables_handle = tables_handle.clone();
    
        Callback::from(move |_: MouseEvent| {
            log::info!("Saving new type: {}", *new_type);
    
            if let Some(col_id_value) = *col_id {
                let mut table_clone = (*table_handle).clone(); // Create a mutable clone of the table
    
                if col_id_value < table_clone.data[0].len() {
                    for row in &mut table_clone.data { // Update entire column with new content type
                        row[col_id_value].content_type = ContentType::from(new_type.clone());
                    }
                    table_clone.data[0][col_id_value].content = vec![new_name.clone().to_string()];
    
                    // Correctly set state handle
                    col_id.set(None);
                    web_sys::console::log_1(&format!("Updated Table:{:?}", table_clone.clone()).into());
    
                    let tables_handle_clone = tables_handle.clone();
                    let table_handle_clone = table_handle.clone();
                    let table_clone_async = table_clone.clone();
                    spawn_local(async move {
                        match update_table(table_clone_async.clone()).await {
                            Ok(_) => {
                                // Set the updated table as the current table
                                table_handle_clone.set(table_clone.clone());
    
                                match fetch_tables().await {
                                    Ok(tables) => {
                                        let tables_struct = Tables { tables: tables.clone() };
                                        tables_handle_clone.set(tables_struct);
                                        table_handle_clone.set(table_clone_async.clone());
    
                                        // if let Some(first_table) = tables.first() {
                                        // }
                                    },
                                    Err(_e) => {
                                        // Handle error
                                    }
                                }
                            },
                            Err(e) => {
                                // Handle error
                                // log::error!("Failed to update table: {}", e.as_string().unwrap_or_else(|| "Unknown error".to_string()));
                            }
                        }
                    });
                } else {
                    log::warn!("Column index out of bounds");
                }
            } else {
                log::warn!("col_id is None");
            }
        })
    };


    let on_input_change = {
        let new_name = new_name.clone();
        move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            new_name.set(input.value());
        }
    };

    let on_select_change = {
        let new_type = new_type.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
            new_type.set(input.value());
            
        })
    };

    let insert_column = {
        let table_handle = table_handle.clone();
        let tables_handle = tables_handle.clone();
    
        Rc::new(move |col_idx: usize, position: &str| {
            let mut updated_table = (*table_handle).clone();
    
            if col_idx < updated_table.data[0].len() {
                for row in &mut updated_table.data {
                    let new_cell = Cell {
                        content_type: if row.is_empty() { ContentType::Text } else { row[col_idx].content_type.clone() }, // copy the content type from the adjacent column if not header row
                        content: vec!["".to_string()],
                    };
    
                    if position == "left" {
                        row.insert(col_idx, new_cell);
                    } else {
                        row.insert(col_idx + 1, new_cell);
                    }
                }
    
                table_handle.set(updated_table.clone());
    
                spawn_local({
                    let tables_handle = tables_handle.clone();
                    let table_handle = table_handle.clone();
                    async move {
                        match update_table(updated_table.clone()).await {
                            Ok(_) => {
                                match fetch_tables().await {
                                    Ok(res_tables) => {
                                        let tables_struct = Tables { tables: res_tables.clone() };
                                        tables_handle.set(tables_struct);
                                        // if let Some(first_table) = res_tables.first() {
                                        //     table_handle.set(first_table.clone());
                                        // }
                                        table_handle.set(updated_table.clone());
                                    },
                                    Err(_) => {}
                                }
                            },
                            Err(_) => {}
                        }
                    }
                });
            }
        })
    } as Rc<dyn Fn(usize, &str)>;

    let insert_row = {
        let table_handle = table_handle.clone();
        let tables_handle = tables_handle.clone();
    
        Rc::new(move || {
            // Clone the current table data
            let mut updated_table = (*table_handle).clone();
    
            // Assuming the first row is the header
            if let Some(header_row) = updated_table.data.first() {
                // Create a new row by cloning the content_type and initializing empty content for each cell
                let new_row: Vec<Cell> = header_row.iter()
                    .map(|header_cell| Cell {
                        content_type: header_cell.content_type.clone(),
                        content: vec!["".to_string()],
                    })
                    .collect();
    
                // Append the new row to the table data
                updated_table.data.push(new_row);
    
                // Update the state
                table_handle.set(updated_table.clone());
    
                spawn_local({
                    let tables_handle = tables_handle.clone();
                    let table_handle = table_handle.clone();
                    async move {
                        match update_table(updated_table.clone()).await {
                            Ok(_) => {
                                match fetch_tables().await {
                                    Ok(res_tables) => {
                                        let tables_struct = Tables { tables: res_tables.clone() };
                                        tables_handle.set(tables_struct);
                                        // if let Some(first_table) = res_tables.first() {
                                        //     table_handle.set(first_table.clone());
                                        // }
                                        table_handle.set(updated_table.clone());
                                    },
                                    Err(_) => {}
                                }
                            },
                            Err(_) => {}
                        }
                    }
                });
            }
        })
    } as Rc<dyn Fn()>;
    

    let delete_column = Rc::new({
        let table_handle = table_handle.clone();
        let tables_handle = tables_handle.clone();

        move |col_idx: usize| {
            let mut updated_table = (*table_handle).clone();

            if col_idx < updated_table.data[0].len() {
                for row in &mut updated_table.data {
                    row.remove(col_idx);
                }

                table_handle.set(updated_table.clone());

                wasm_bindgen_futures::spawn_local({
                    let tables_handle = tables_handle.clone();
                    let table_handle = table_handle.clone();
                    async move {
                        match update_table(updated_table.clone()).await {
                            Ok(_) => {
                                match fetch_tables().await {
                                    Ok(res_tables) => {
                                        let tables_struct = Tables { tables: res_tables.clone() };
                                        tables_handle.set(tables_struct);
                                        if let Some(first_table) = res_tables.first() {
                                            table_handle.set(first_table.clone());
                                        }
                                    },
                                    Err(e) => {}
                                }
                            },
                            Err(e) => {},
                        }
                    }
                });
            }
        }
    }) as Rc<dyn Fn(usize)>;

    
    let checked_rows = use_state(|| HashSet::new());

    let handle_checkbox_change = {
        let checked_rows = checked_rows.clone();
        move |row_idx: usize| {
            let mut set = (*checked_rows).clone();
            if set.contains(&row_idx) {
                set.remove(&row_idx);
            } else {
                set.insert(row_idx);
            }
            checked_rows.set(set);
        }
    };

    let delete_checked_rows = {
        let table = table.clone();
        let checked_rows = checked_rows.clone();
        let update_table = update_table.clone();
        move |_| { // Accept the event argument
            let mut new_data = Vec::with_capacity(table.data.len());
            for (i, row) in table.data.iter().enumerate() {
                if i == 0 || !checked_rows.contains(&(i - 1)) {
                    new_data.push(row.clone());
                }
            }
            let new_table = Table {
                id: table.id.clone(),
                title: table.title.clone(),  // Make sure to copy any other necessary fields
                data: new_data,
                // add any other fields as necessary
            };
            
            let tables_handle = tables_handle.clone();
            let table_handle = table_handle.clone();
            let checked_rows = checked_rows.clone();
    
            spawn_local(async move {
                match update_table(new_table.clone()).await {
                    Ok(_) => {
                        match fetch_tables().await {
                            Ok(res_tables) => {
                                let tables_struct = Tables { tables: res_tables.clone() };
                                tables_handle.set(tables_struct);
                                table_handle.set(new_table);
                                checked_rows.set(HashSet::new())
                            },
                            Err(e) => {
                                // log::error!("Failed to fetch tables: {:?}", e);
                            }
                        }
                    },
                    Err(e) => {
                        // log::error!("Failed to update table: {:?}", e);
                    }
                }
            });
        }
    };

    html! {
        <>
            <div class="p-4 sm:ml-64 min-h-screen overflow-x-auto">
                <div>
                    <h2 class="text-white">{table.title.clone()}</h2>
                    <hr class="border-gray-600 my-2"/>
                    <div class="flex">
                        <div class="relative overflow-x-auto shadow-md sm:rounded-lg flex-col w-full" style="overflow:visible">
                            <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400 h-fit">
                                <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                                    <tr>
                                        <th scope="col" class="px-6 py-3 w-[50px]  cursor-pointer relative border-r border-gray-600">
                                            <button 
                                                onclick={delete_checked_rows.clone()} 
                                                class="text-white border border-gray-600 hover:bg-gray-600 hover:border-gray-800 focus:ring-4 focus:outline-none focus:ring-gray-300 font-medium rounded-lg text-sm p-2 text-center dark:border-gray-600 dark:hover:border-gray-700 dark:focus:ring-gray-800"
                                            >
                                                <i class="fas fa-trash text-gray-300"></i>
                                            </button>
                                        </th>
                                        {
                                            for table.data[0].iter().enumerate().map(|(index, header)| {
                                                let is_open = *open_dropdowns == Some(index);
                                                web_sys::console::log_1(&format!("init_type{:?}", new_type.clone().to_string()).into());

                                                if col_id.as_ref() == Some(&index) {
                                                    html! {
                                                        <th scope="col" class="px-6 py-3 cursor-pointer relative border-r border-gray-600 flex items-center">
                                                            <div class="flex items-center gap-2">
                                                                <input type="text" value={(*new_name).clone()} oninput={on_input_change.clone()} class="min-w-32 max-w-40 bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
                                                                <div  class="" >
                                                                    {
                                                                        html! {
                                                                            <select value={(*new_type).clone()} onchange={on_select_change.clone()} id="type" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
                                                                                <option value="text" selected={*new_type == "text"}>{"Text"}</option>
                                                                                <option value="number" selected={*new_type == "number"}>{"Number"}</option>
                                                                                <option value="single_select" selected={*new_type == "single_select"}>{"Single Select"}</option>
                                                                                <option value="multiple_select" selected={*new_type == "multiple_select"}>{"Multiple Select"}</option>
                                                                                <option value="date" selected={*new_type == "date"}>{"Date"}</option>
                                                                                <option value="url" selected={*new_type == "url"}>{"URL"}</option>
                                                                                <option value="checkbox" selected={*new_type == "checkbox"}>{"Checkbox"}</option>
                                                                                <option value="attachment" selected={*new_type == "attachment"}>{"Attachment"}</option>
                                                                                <option value="subtask" selected={*new_type == "subtask"}>{"Subtask"}</option>
                                                                            </select>
                                                                        }
                                                                    }
                                                                </div>
                                                                <button onclick={on_save_click.clone()} class="text-white border border-gray-600 hover:bg-gray-600 hover:border-gray-800 focus:ring-4 focus:outline-none focus:ring-gray-300 font-medium rounded-lg text-sm w-full sm:w-auto p-1 text-center dark:border-gray-600 dark:hover:border-gray-700 dark:focus:ring-gray-800"><i class="fas fa-archive"></i></button>
                                                            </div>
                                                        </th>
                                                    }
                                                } else {
                                                    html! { 
                                                        <th scope="col" class="px-6 py-3 cursor-pointer relative border-r border-gray-600"
                                                            onmouseenter={
                                                                let toggle_dropdown = toggle_dropdown.clone();
                                                                Callback::from(move |_| {
                                                                    toggle_dropdown(Some(index));
                                                                })
                                                            }
                                                            onmouseleave={
                                                                let toggle_dropdown = toggle_dropdown.clone();
                                                                Callback::from(move |_| {
                                                                    toggle_dropdown(None);
                                                                })
                                                            }
                                                        >
                                                            <div class="flex gap-x-2 items-center">
                                                                <DisplayCellIcon content_type={header.content_type.clone()} />
                                                                {header.content[0].clone()}
                                                                <i class="fas fa-sort-desc mb-1"></i>
                                                                <ColumnDropdown 
                                                                    column_index={index} 
                                                                    is_open={is_open} 
                                                                    set_col_id={Callback::from({
                                                                        let col_id = col_id.clone();
                                                                        let new_name = new_name.clone();
                                                                        let col_name = header.content[0].clone(); 
                                                                        let col_type = header.content_type.clone();
                                                                        let new_type = new_type.clone(); // Clone here
                                        
                                                                        move |id: Option<String>| {
                                                                            col_id.set(id.clone().and_then(|id_str| id_str.parse::<usize>().ok()));
                                        
                                                                            if id.is_some() {
                                                                                new_name.set(col_name.clone());
                                                                                new_type.set(col_type.clone().to_string()); // Convert col_type to String
                                                                            }
                                                                        }
                                                                    })}
                                                                    insert_column={insert_column.clone()}
                                                                    delete_column={delete_column.clone()}
                                                                />
                                                            </div>
                                                        </th>
                                                    }
                                                }
                                            })
                                        }
                                    </tr>
                                </thead>
                                <tbody>
                                {
                                    for table.data[1..].iter().enumerate().map(|(row_idx, row)| html! {
                                        <tr class="odd:bg-white odd:dark:bg-gray-900 even:bg-gray-50 even:dark:bg-gray-800 border-b dark:border-gray-700">
                                            
                                            <th scope="col" class="px-6 py-3 cursor-pointer relative border-r border-gray-600 flex justify-center h-full items-center">
                                                <input 
                                                    id="default-checkbox" 
                                                    type="checkbox" 
                                                    class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                                                    checked={checked_rows.contains(&(row_idx))}
                                                    onclick={
                                                        let handle_checkbox_change = handle_checkbox_change.clone();
                                                        Callback::from(move |_| handle_checkbox_change(row_idx))
                                                    }
                                                />
                                            </th>
                                            {
                                                for row.iter().enumerate().map(|(col_idx, cell)| html! {
                                                    <td class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white border-r border-gray-600">
                                                        <EditableCell
                                                            content_type={cell.content_type.clone()}
                                                            content={cell.content.clone()}
                                                            toggle_modal={toggle_modal.clone()}
                                                            on_save={Callback::from({
                                                                let update_cell_content = update_cell_content.clone();
                                                                move |new_content: Vec<String>| {
                                                                    update_cell_content(row_idx + 1, col_idx, new_content);
                                                                }
                                                            })}
                                                        />
                                                    </td>
                                                })
                                            }
                                        </tr>
                                    })
                                }
                                </tbody>
                            </table>
                            <div 
                                onclick={
                                    let insert_row = insert_row.clone();
                                    Callback::from(move |_| insert_row())
                                } 
                                class="w-full bg-gray-800 hover:bg-gray-700 text-white cursor-pointer text-sm text-center py-4 border-r border-gray-600"
                            >
                                <i class="fas fa-plus"></i>
                            </div>
                        </div>
                        <div class="w-20 bg-gray-700 hover:bg-gray-500 text-white cursor-pointer text-sm text-center py-4"
                            onclick= {
                                let insert_column = insert_column.clone();
                                let col_id = table.data[0].len()-1;
                                Callback::from(move |_| insert_column(col_id, "right"))
                            } 
                        ><i class="fas fa-plus"></i></div>
                    </div>
                </div>
                <Drawer is_open={*open_drawer} toggle_drawer={toggle_drawer}/>
                
            </div>
            <AddFileModal is_open={*is_file_modal} toggle_modal={toggle_file_modal.clone()}/>
            <SubtaskModal is_open={*is_subtask_modal} toggle_modal={toggle_subtask_modal.clone()}/>
        </>
    }
}

