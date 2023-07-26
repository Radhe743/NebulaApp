use crate::state::AppState;
use crate::utils::status::error::{ErrorCode, ErrorResponse};
use crate::{
    nebula::NebulaNotebookFile::{NebulaNotebook, PageSimple},
    utils::Application::get_notebook_data_dir,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::{
    fs,
    io::{Read, Write},
};
use tauri::State;
#[derive(Serialize, Deserialize)]
pub struct Response {
    notebook: NebulaNotebook,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NotebookMetadata {
    __id: String,
    name: String,
    thumbnail: Option<String>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct MetaDataFile {
    notebooks: Vec<NotebookMetadata>,
}
pub fn load_notebooks_metadata() -> Result<MetaDataFile, String> {
    let notebooks_dir = get_notebook_data_dir();
    let notebooks_metadata_json = notebooks_dir.join("meta_data.json");

    match notebooks_metadata_json.exists() {
        true => {
            let mut file =
                fs::File::open(&notebooks_metadata_json).map_err(|_| "Error Loading File")?;
            let mut json = String::new();

            file.read_to_string(&mut json)
                .map_err(|_| "Error reading from file")?;

            let data: MetaDataFile =
                serde_json::from_str(&json).map_err(|_| "Error Deserializing")?;

            Ok(data)
        }
        _ => Err("Error Loading MetaData File ".to_string()),
    }
}
pub fn put_metadata(data: &MetaDataFile) -> Result<(), String> {
    let notebooks_dir = get_notebook_data_dir();
    let notebooks_metadata_json = notebooks_dir.join("meta_data.json");

    let json_string = serde_json::to_string(&data).map_err(|_| "Error Json Stringify")?;
    let mut json_file =
        fs::File::create(notebooks_metadata_json).map_err(|_| "Error Creating File")?;
    json_file
        .write_all(json_string.as_bytes())
        .map_err(|_| "Error Saving to File")?;

    Ok(())
}

pub fn add_notebook_to_metadata_file(metadata: &NotebookMetadata) -> Result<MetaDataFile, String> {
    let notebooks_dir = get_notebook_data_dir();
    let notebooks_metadata_json = notebooks_dir.join("meta_data.json");

    let existing_data = if notebooks_metadata_json.exists() {
        let data = load_notebooks_metadata()?;
        Ok(data)
    } else {
        let new_meta_data_file_content = MetaDataFile {
            notebooks: Vec::new(),
        };
        Ok(new_meta_data_file_content)
    };

    match existing_data {
        Ok(mut data) => {
            data.notebooks.push(metadata.clone());
            put_metadata(&data)?;
            Ok(data)
        }
        Err(err) => err,
    }
}

#[tauri::command]
pub fn create_nebula_notebook(notebook_name: String) -> Result<Response, String> {
    let new_notebook = NebulaNotebook::new(notebook_name);
    let meta_data = NotebookMetadata {
        __id: new_notebook.__id.clone(),
        name: new_notebook.name.clone(),
        thumbnail: new_notebook.thumbnail.clone(),
    };

    match new_notebook.save_to_file() {
        Ok(()) => {
            let res = Response {
                notebook: new_notebook,
            };
            add_notebook_to_metadata_file(&meta_data)?;
            Ok(res)
        }
        Err(error) => Err(error),
    }
}
// Load Notebooks at startup
#[tauri::command]
pub fn load_nebula_notebooks() -> Result<MetaDataFile, String> {
    let res = load_notebooks_metadata()?;
    Ok(res)
}

#[derive(Serialize, Deserialize)]
pub struct NotebookResponse {
    pub __id: String,
    pub name: String,
    pub last_accessed_at: String,
    pub created_at: String,
    assets: Vec<String>,
    pub pages: Vec<PageSimple>,
    pub thumbnail: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
}

impl NotebookResponse {
    fn new(notebook: &NebulaNotebook, pages_simple: Vec<PageSimple>) -> Self {
        NotebookResponse {
            __id: notebook.__id.clone(),
            name: notebook.name.clone(),
            last_accessed_at: notebook.last_accessed_at.clone(),
            created_at: notebook.created_at.clone(),
            assets: notebook.assets.clone(),
            pages: pages_simple,
            thumbnail: notebook.thumbnail.clone(),
            description: notebook.description.clone(),
            author: notebook.author.clone(),
        }
    }
}

#[tauri::command]
pub fn load_nebula_notebook(
    state: State<'_, Arc<Mutex<AppState>>>,
    notebook_id: String,
) -> Result<NotebookResponse, ErrorResponse> {
    let mut state = state.lock().unwrap();
    let notebooks_dir = get_notebook_data_dir();
    let file_path = notebooks_dir.join(notebook_id + ".nb");

    let notebook = NebulaNotebook::load_from_file(&file_path);

    match notebook {
        Ok(notebook) => {
            state.set_notebook(notebook);

            match &state.notebook {
                Some(notebook) => {
                    let simple_pages = notebook.get_simple_pages();
                    let response = NotebookResponse::new(notebook, simple_pages);
                    Ok(response)
                }

                _ => Err(ErrorResponse::new(
                    ErrorCode::NotebookNotLoadedYet,
                    "Notebook is loaded".to_string(),
                )),
            }
        }
        Err(error) => Err(ErrorResponse::new(ErrorCode::NotFoundError, error)),
    }
}

// #[tauri::command]
// pub fn unload_nebula_notebook() {}
