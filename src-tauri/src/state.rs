use crate::nebula::NebulaNotebookFile::NebulaNotebook;
pub struct AppState {
    pub notebook: Option<NebulaNotebook>,
}

impl AppState {
    pub fn new() -> Self {
        AppState { notebook: None }
    }
    pub fn set_notebook(&mut self, notebook: NebulaNotebook) {
        println!("Setting Notebook \n\n");
        println!("{:?}", &notebook);
        self.notebook = Some(notebook);
    }
    pub fn unload_notebook(&mut self) -> Result<bool, String> {
        match &self.notebook {
            Some(notebook) => {
                println!("\n Unloading Notebook '{}'", notebook.name);
                notebook.save_to_file()?;
                self.notebook = None;
                Ok(true)
            }
            _ => Err("No Notebook To Unload".to_string()),
        }
    }
}
