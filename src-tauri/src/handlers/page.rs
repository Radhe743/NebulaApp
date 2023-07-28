use crate::{
    nebula::NebulaNotebookFile::{PageEntry, PageSimple},
    state::AppState,
    utils::status::error::ErrorResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct LoadPageResponse {
    page: PageEntry,
    expanded: Vec<String>,
}

#[tauri::command]
pub fn load_page(
    state: State<Arc<Mutex<AppState>>>,
    page_id: String,
) -> Result<LoadPageResponse, ErrorResponse> {
    let mut state = state.lock().unwrap();

    state.use_notebook(|notebook| {
        let page = notebook.get_page(&page_id)?;
        let expanded = notebook.pages_to_expand(&page.__id);
        let res = LoadPageResponse { page, expanded };
        Ok(res)
    })?
}

#[derive(Serialize, Deserialize)]
pub struct AddPageResponse {
    pages: Vec<PageSimple>,
    new_page_id: String,
}
#[tauri::command]
pub fn add_page(
    state: State<Arc<Mutex<AppState>>>,
    title: String,
    parent_id: Option<String>,
    insert_after_id: Option<String>,
) -> Result<AddPageResponse, ErrorResponse> {
    let mut state = state.lock().unwrap();

    state.use_notebook(|notebook| {
        let new_page_id = notebook.add_page(title, parent_id, insert_after_id);
        let pages = notebook.get_simple_pages();

        let res = AddPageResponse { pages, new_page_id };
        Ok(res)
    })?
}
#[tauri::command]
pub fn update_page(
    state: State<Arc<Mutex<AppState>>>,
    page_id: String,
    new_content: String,
) -> Result<String, ErrorResponse> {
    let mut state = state.lock().unwrap();

    state.use_notebook(|notebook| {
        if let Some(page) = notebook.page_map.get_mut(&page_id) {
            page.content.body = new_content.to_owned();
        }
        Ok(new_content)
    })?
}
