use crate::utils::status::error::{ErrorCode, ErrorResponse};
use crate::{
    nebula::Header::{FileHeader, FILE_FORMAT_CURRENT_VERSION},
    utils::Application::get_notebook_data_dir,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    path::PathBuf,
};
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageContent {
    pub doctype: String,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageEntry {
    pub __id: String,
    pub title: String,
    pub content: PageContent,
    pub created_at: String,
    pub updated_at: String,
    pub pinned: bool,
    pub starred: bool,
    pub tags: Option<Vec<String>>,
    pub parent_id: Option<String>,
    pub sub_pages: Vec<String>,
    pub is_in_trash: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NebulaNotebook {
    pub __id: String,
    pub name: String,
    pub thumbnail: Option<String>,
    pub created_at: String,
    pub pages: Vec<String>,
    pub page_map: HashMap<String, PageEntry>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub assets: Vec<String>,
    pub last_accessed_at: String,
    pub is_in_trash: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageSimple {
    pub __id: String,
    pub parent_id: Option<String>,
    pub title: String,
    pub pinned: bool,
    pub starred: bool,
    pub sub_pages: Vec<PageSimple>,
}
// Page Entry Implementation
impl PageEntry {
    pub fn new(title: String, parent_id: Option<String>) -> Self {
        PageEntry {
            __id: Uuid::new_v4().to_string(),
            title,
            content: PageContent::new(),
            created_at: Utc::now().to_rfc3339().to_string(),
            updated_at: Utc::now().to_rfc3339().to_string(),
            pinned: false,
            starred: false,
            parent_id,
            sub_pages: Vec::new(),
            tags: None,
            is_in_trash: false,
        }
    }
}
// Content Implementation
impl PageContent {
    fn new() -> Self {
        PageContent {
            doctype: "markdown".to_string(),
            body: "".to_string(),
        }
    }
}

impl NebulaNotebook {
    pub fn new(name: String) -> Self {
        NebulaNotebook {
            __id: Uuid::new_v4().to_string(),
            name,
            thumbnail: None,
            created_at: Utc::now().to_rfc3339().to_string(),
            last_accessed_at: Utc::now().to_rfc3339().to_string(),
            assets: Vec::new(),
            pages: Vec::new(),
            page_map: HashMap::new(),
            author: None,
            description: None,
            is_in_trash: false,
        }
    }
    pub fn get_page(&self, page_id: &str) -> Result<PageEntry, ErrorResponse> {
        match self.page_map.get(page_id) {
            Some(page) => Ok(page.clone()),
            _ => Err(ErrorResponse::new(
                ErrorCode::NotFoundError,
                String::from("Page Not found"),
            )),
        }
    }
    pub fn pages_to_expand(&self, page_id: &str) -> Vec<String> {
        let mut expanded_pages: Vec<String> = Vec::new();

        // Helper function to perform the recursive backtrace
        fn backtrace(
            page: &PageEntry,
            expanded_pages: &mut Vec<String>,
            page_map: &HashMap<String, PageEntry>,
        ) {
            if let Some(parent_id) = &page.parent_id {
                if let Some(parent) = page_map.get(parent_id) {
                    expanded_pages.push(parent_id.to_string());
                    backtrace(parent, expanded_pages, page_map);
                }
            }
        }

        if let Some(page) = self.page_map.get(page_id) {
            backtrace(page, &mut expanded_pages, &self.page_map);
        }

        expanded_pages
    }

    pub fn get_simple_pages(&self) -> Vec<PageSimple> {
        let mut simple_pages: Vec<PageSimple> = Vec::new();

        for page_id in &self.pages {
            if let Some(page) = self.page_map.get(page_id) {
                let simple_page = self.recursive_convert(page);
                simple_pages.push(simple_page);
            }
        }

        simple_pages
    }
    fn recursive_convert(&self, page: &PageEntry) -> PageSimple {
        let mut simple_page = PageSimple {
            __id: page.__id.clone(),
            title: page.title.clone(),
            parent_id: page.parent_id.clone(),
            pinned: page.pinned.clone(),
            starred: page.starred.clone(),
            sub_pages: Vec::new(),
        };
        if !page.sub_pages.is_empty() {
            for sub_page_id in &page.sub_pages {
                if let Some(sub_page) = self.page_map.get(sub_page_id) {
                    let sub_simple_page = self.recursive_convert(sub_page);
                    simple_page.sub_pages.push(sub_simple_page);
                }
            }
        }
        simple_page
    }

    pub fn add_page(
        &mut self,
        title: String,
        parent_id: Option<String>,
        insert_after: Option<String>,
    ) -> String {
        let new_page = PageEntry::new(title, parent_id.to_owned());
        self.page_map
            .insert(new_page.__id.to_owned(), new_page.to_owned());

        match (parent_id, insert_after) {
            (Some(parent_id), Some(insert_after_id)) => {
                if let Some(parent_page) = self.page_map.get_mut(&parent_id) {
                    let idx = parent_page
                        .sub_pages
                        .iter()
                        .position(|id| *id == insert_after_id)
                        .unwrap_or_else(|| parent_page.sub_pages.len());
                    // Add to  subpage list
                    parent_page
                        .sub_pages
                        .insert(idx + 1, new_page.__id.to_owned());
                }
            }
            (Some(parent_id), None) => {
                if let Some(parent_page) = self.page_map.get_mut(&parent_id) {
                    parent_page.sub_pages.insert(0, new_page.__id.to_owned());
                }
            }
            (None, Some(insert_after_id)) => {
                if let Some(insert_after_page) = self.page_map.get_mut(&insert_after_id) {
                    match &insert_after_page.parent_id {
                        // If the there is a parent_id means that the page is a sub page and the element
                        Some(_) => {
                            let idx = insert_after_page
                                .sub_pages
                                .iter()
                                .position(|id| *id == insert_after_id)
                                .unwrap_or_else(|| insert_after_page.sub_pages.len());

                            // May panic
                            insert_after_page
                                .sub_pages
                                .insert(idx + 1, new_page.__id.to_owned());
                        }
                        // If the parent id is None means that the page is a root page
                        None => {
                            let idx = self
                                .pages
                                .iter()
                                .position(|id| *id == insert_after_id)
                                .unwrap_or_else(|| self.pages.len());
                            println!("Idx {}", idx);
                            self.pages.insert(idx + 1, new_page.__id.to_owned());
                        }
                    }
                };
            }
            (None, None) => {
                self.pages.insert(0, new_page.__id.to_owned());
            }
        }
        new_page.__id.to_owned()
    }

    pub fn save_to_file(&self) -> Result<(), ErrorResponse> {
        let notebook_data_dir = get_notebook_data_dir();
        //?  Get the storage Dir and make a new file with the data

        let new_filepath = notebook_data_dir.join(self.__id.to_owned() + ".nb");
        let mut file = fs::File::create(new_filepath).map_err(|err| {
            ErrorResponse::new(ErrorCode::IoError, format!("Error creating file {}", err))
        })?;

        let file_header = FileHeader::new();

        //? Serialize the data and file header
        let serialized_header = bincode::serialize(&file_header).map_err(|err| {
            ErrorResponse::new(
                ErrorCode::SerializationError,
                format!("Error serializing Header {}", err),
            )
        })?;
        let serialized_data = bincode::serialize(&self).map_err(|err| {
            ErrorResponse::new(
                ErrorCode::SerializationError,
                format!("Error serializing Data{}", err),
            )
        })?;

        file.write_all(&serialized_header).map_err(|err| {
            ErrorResponse::new(ErrorCode::IoError, format!("Error Writing to file {}", err))
        })?;
        file.write_all(&serialized_data).map_err(|err| {
            ErrorResponse::new(ErrorCode::IoError, format!("Error Writing to file {}", err))
        })?;

        Ok(())
    }

    pub fn load_from_file(filepath: &PathBuf) -> Result<Self, ErrorResponse> {
        match filepath.is_file() {
            true => {
                // ? Read File to buffer
                let mut file = fs::File::open(filepath).map_err(|err| {
                    ErrorResponse::new(ErrorCode::IoError, format!("Error opening file {}", err))
                })?;
                let mut buffer: Vec<u8> = Vec::new();
                file.read_to_end(&mut buffer).map_err(|err| {
                    ErrorResponse::new(ErrorCode::IoError, format!("Error opening file {}", err))
                })?;

                //? Split The header and data Part

                let header_size: usize = std::mem::size_of::<FileHeader>();
                let header_data: &[u8] = &buffer[..header_size];
                let notebook_data: &[u8] = &buffer[header_size..];

                let header: FileHeader =
                    bincode::deserialize::<FileHeader>(&header_data).map_err(|err| {
                        ErrorResponse::new(
                            ErrorCode::DeserializationError,
                            format!("Error retrieving header {}", err),
                        )
                    })?;

                let file_version = header.__version__;

                // ? Match the file version
                match file_version {
                    FILE_FORMAT_CURRENT_VERSION => {
                        let mut notebook = bincode::deserialize::<NebulaNotebook>(&notebook_data)
                            .map_err(|err| {
                            ErrorResponse::new(
                                ErrorCode::DeserializationError,
                                format!("Error deserializing notebook data {}", err),
                            )
                        })?;
                        notebook.last_accessed_at = Utc::now().to_rfc3339().to_string();
                        Ok(notebook)
                    }
                    // Handle Migration logic
                    _ => Err(ErrorResponse::new(
                        ErrorCode::Unsupported,
                        "This format is not supported".to_string(),
                    )),
                }
            }
            _ => Err(ErrorResponse::new(
                ErrorCode::NotFoundError,
                "Notebook not found".to_string(),
            )),
        }
    }
}