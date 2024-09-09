use yew::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Attachment {
    pub name: String,
    pub file_type: FileType,
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileType {
    Image,
    Video,
    Document,
    Others
}

fn serialize_attachments(attachments: &[Attachment]) -> Vec<String> {
    attachments.iter()
        .map(|attachment| serde_json::to_string(attachment).unwrap())
        .collect()
}

fn deserialize_attachments(serialized_attachments: &[String]) -> Vec<Attachment> {
    serialized_attachments.iter()
        .map(|serialized| serde_json::from_str(serialized).unwrap())
        .collect()
}

// // Serialize
// let serialized_subtasks = serialize_subtasks(&subtasks);
// console::log_1(&format!("Serialized Subtasks: {:?}", serialized_subtasks).into());

// // Deserialize
// let deserialized_subtasks = deserialize_subtasks(&serialized_subtasks);
// console::log_1(&format!("Deserialized Subtasks: {:?}", deserialized_subtasks).into());

#[derive(Properties, Clone, PartialEq)]
pub struct AttachmentRowProps {
    pub attachments: Vec<Attachment>,
    pub toggle_modal: Callback<String>
}

#[function_component(AttachmentSummary)]
fn attachment_summary(props: &AttachmentRowProps) -> Html {
    let count_image = props.attachments.iter().filter(|s| matches!(s.file_type, FileType::Image)).count();
    let count_video = props.attachments.iter().filter(|s| matches!(s.file_type, FileType::Video)).count();
    let count_document = props.attachments.iter().filter(|s| matches!(s.file_type, FileType::Document)).count();
    let count_others = props.attachments.iter().filter(|s| matches!(s.file_type, FileType::Others)).count();

    html! {
        <div class="flex space-x-4">
            <span><i class="fas fa-image mr-1 text-gray-500"/>{count_image}</span>
            <span><i class="fas fa-video mr-1 text-gray-500"/>{ count_video}</span>
            <span><i class="fas fa-file-alt mr-1 text-gray-500"/>{ count_document}</span>
            <span><i class="fas fa-file mr-1 text-gray-500"/>{ count_others}</span>
        </div>
    }
}

#[function_component(AttachmentDetailsDropdown)]
fn attachment_details_dropdown(props: &AttachmentRowProps) -> Html {
    let AttachmentRowProps {toggle_modal,..} = props.clone();
    html! {
        <div class="absolute p-1 w-44 hidden group-hover:block z-50 top-0 right-0 w-full bg-white border-gray-300 rounded-lg shadow-sm dark:bg-gray-700">
            { for props.attachments.iter().map(|attachment| {
                let (file_icon, file_color) = match attachment.file_type {
                    FileType::Image => ("fa-image", "gray-500"),
                    FileType::Video => ("fa-video", "gray-500"),
                    FileType::Document => ("fa-file-alt", "gray-500"),
                    FileType::Others => ("fa-file", "gray-500"),
                };

                html! {
                    <div class={format!("p-1 border rounded-lg shadow-sm mb-1 border-{} hover:border-gray-300 flex flex-col items-center justify-between space-y-2 relative", file_color)}>
                        <i class="fas fa-times text-gray-500 text-[10px] hover:text-white absolute top-1 right-2"/>
                        <div class="flex items-center">
                            <i class={format!("fas {} mr-2 text-{}", file_icon, file_color)}></i>
                            <h3 class="text-sm">{ &attachment.name }</h3>
                        </div>
                    </div>
                }
            }) }
            <div
                onclick = {
                    let toggle_modal = toggle_modal.clone();
                    Callback::from(move |_: MouseEvent| {
                        toggle_modal.emit("file".to_string());
                    })
                }  
                class="p-1 border rounded-lg shadow-sm mb-1 border-gray-500 hover:border-gray-300 flex flex-col items-center justify-between space-y-2"
            >
                <div class="flex items-center" 
                    onclick = {
                        let toggle_modal = toggle_modal.clone();
                        Callback::from(move |_: MouseEvent| {
                            toggle_modal.emit("subtask".to_string());
                        })
                    }
                >
                    <i class="fas fa-plus text-gray-300"></i>
                </div>
            </div>
        </div>
    }
}


#[derive(Properties, Clone, PartialEq)]
pub struct AttachmentProps {
    pub attachments: Vec<Attachment>,
    pub toggle_modal: Callback<String>
}

#[function_component(AttachmentComponent)]
pub fn attachment_component(props: &AttachmentProps) -> Html {
    let AttachmentProps {toggle_modal, ..} = props.clone();
    html! {
        <div class="whitespace-nowrap relative group">
            <div class="group-hover:hidden">
                <AttachmentSummary attachments={props.attachments.clone()} toggle_modal={toggle_modal.clone()}/>
            </div>
            
            <AttachmentDetailsDropdown attachments={props.attachments.clone()} toggle_modal={toggle_modal.clone()}/>
        </div>
    }
}