use yew::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Subtask {
    pub name: String,
    pub complete_rate: u8,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

fn serialize_subtasks(subtasks: &[Subtask]) -> Vec<String> {
    subtasks.iter()
        .map(|subtask| serde_json::to_string(subtask).unwrap())
        .collect()
}

fn deserialize_subtasks(serialized_subtasks: &[String]) -> Vec<Subtask> {
    serialized_subtasks.iter()
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
pub struct TaskRowProps {
    pub subtasks: Vec<Subtask>,
    pub toggle_modal: Callback<String>
}

#[function_component(TaskSummary)]
fn task_summary(props: &TaskRowProps) -> Html {
    let count_not_started = props.subtasks.iter().filter(|s| matches!(s.status, Status::NotStarted)).count();
    let count_in_progress = props.subtasks.iter().filter(|s| matches!(s.status, Status::InProgress)).count();
    let count_completed = props.subtasks.iter().filter(|s| matches!(s.status, Status::Completed)).count();

    html! {
        <div class="flex space-x-4">
            <span><i class="fas fa-times-circle mr-1 text-red-500"/>{count_not_started}</span>
            <span><i class="fas fa-hourglass-half mr-1 text-yellow-500"/>{ count_in_progress}</span>
            <span><i class="fas fa-check-circle mr-1 text-green-500"/>{ count_completed}</span>
        </div>
    }
}

#[function_component(SubtaskDetailsDropdown)]
fn subtask_details_dropdown(props: &TaskRowProps) -> Html {
    let TaskRowProps {toggle_modal, ..} = props.clone();
    html! {
        <div class="absolute p-1 w-44 hidden group-hover:block z-50 top-0 right-0 w-full bg-white border-gray-300 rounded-lg shadow-sm dark:bg-gray-700">
            { for props.subtasks.iter().map(|subtask| {
                let (status_color, status_icon) = match subtask.status {
                    Status::NotStarted => ("red-500", "fa-times-circle"),
                    Status::InProgress => ("yellow-500", "fa-hourglass-half"),
                    Status::Completed => ("green-500", "fa-check-circle"),
                };

                html! {
                    <div class={format!("relative p-1 border rounded-lg shadow-sm mb-1 border-{} hover:border-gray-300 flex flex-col items-center justify-between space-y-2", status_color)}>
                        <i class="fas fa-times text-gray-500 text-[10px] hover:text-white absolute top-1 right-2"/>
                        <div class="flex items-center">
                            <i class={format!("fas {} mr-2 text-{}", status_icon, status_color)}></i>
                            <h3 class="text-sm">{ &subtask.name }</h3>
                        </div>
                        <div class={format!("w-full h-2.5 bg-gray-200 rounded-full dark:bg-gray-800")}>
                            <div class={format!("h-2.5 rounded-full bg-{}", status_color)} style={format!("width: {}%", subtask.complete_rate)}></div>
                        </div>
                    </div>
                }
            }) }
            <div
                onclick = {
                    let toggle_modal = toggle_modal.clone();
                    Callback::from(move |_: MouseEvent| {
                        toggle_modal.emit("subtask".to_string());
                    })
                }  
                class="p-1 border rounded-lg shadow-sm mb-1 border-gray-500 hover:border-gray-300 flex flex-col items-center justify-between space-y-2"
            >
                <div class="flex items-center">
                    <i class="fas fa-plus text-gray-300"></i>
                </div>
            </div>
        </div>
    }
}


#[derive(Properties, Clone, PartialEq)]
pub struct SubtaskProps {
    pub subtasks: Vec<Subtask>,
    pub toggle_modal: Callback<String>
}

#[function_component(SubtaskComponent)]
pub fn subtask_component(props: &SubtaskProps) -> Html {
    let SubtaskProps {toggle_modal, ..} = props.clone();
    html! {
        <div class="whitespace-nowrap relative group">
            <div class="group-hover:hidden">
                <TaskSummary subtasks={props.subtasks.clone()} toggle_modal={toggle_modal.clone()}/>
            </div>
            
            <SubtaskDetailsDropdown subtasks={props.subtasks.clone()} toggle_modal={toggle_modal.clone()}/>
        </div>
    }
}