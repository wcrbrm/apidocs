use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;
use std::path::Path;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum DocSectionLink {
    /// whether this is external
    External {
        /// url of the external link
        url: String,
    },
    Local,
}

impl DocSectionLink {
    pub fn from_url_opt(url: &Option<String>) -> Self {
        if let Some(u) = url {
            return Self::External { url: u.clone() };
        }
        Self::Local
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DocSection {
    /// name of the section, and name of the folder
    pub section: String,
    /// title of the section
    pub title: String,
    /// link to other sections
    pub link: DocSectionLink,
}

#[derive(Default, Serialize, Deserialize, ToSchema)]
pub struct Entry {
    /// ID of the service
    pub id: String,
    /// Title of the service
    pub title: String,
    /// Description of the service
    pub description: String,
    /// Documentation sections
    pub docs: Map<String, DocSection>,
}

impl Entry {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
            docs: Map::new(),
        }
    }

    pub fn add_section(&mut self, section: DocSection) {
        self.docs.insert(section.section.clone(), section);
    }

    pub fn remove_section(&mut self, section: &str) {
        self.docs.remove(section);
    }
}

#[derive(Default, Serialize, Deserialize, ToSchema)]
pub struct Config {
    /// map of entries
    pub entries: Map<String, Entry>,
}

impl Config {
    pub fn add(&mut self, entry: Entry) {
        self.entries.insert(entry.id.clone(), entry);
    }

    pub fn get(&self, id: &str) -> Option<&Entry> {
        self.entries.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Entry> {
        self.entries.get_mut(id)
    }

    pub fn remove(&mut self, id: &str) {
        self.entries.remove(id);
    }

    pub fn from_path(storage_path: impl AsRef<Path>) -> Self {
        use std::io::Read;
        let Ok(mut file) = std::fs::File::open(storage_path.as_ref().join("config.json")) else { 
            return Self::default();
        };
        let mut json = String::new();
        let Ok(_) = file.read_to_string(&mut json) else {
            return Self::default();
        };
        let Ok(config) = serde_json::from_str::<Config>(&json) else {
            return Self::default();
        };
        config
    }

    pub fn save(&self, storage_path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        use std::io::Write;
        let mut file = std::fs::File::create(storage_path.as_ref().join("config.json"))?;
        let json = serde_json::to_string_pretty(&self)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn into_html(&self) -> String {
        let mut html = String::new();
        for (id, entry) in self.entries.iter() {
            html.push_str(&format!(
                r#"<div class="entry" id="{}"><h2>{}</h2><p>{}</p>"#,
                id, entry.title, entry.description
            ));
            for (_, doc) in entry.docs.iter() {
                match &doc.link {
                    DocSectionLink::External { url } => {
                        html.push_str(&format!(
                            r#"<div class="doc"><a href="{}" target="_blank" rel="noreferer noopener">{}</a></div>"#,
                            url, doc.title,
                        ));
                    }
                    DocSectionLink::Local => {
                        html.push_str(&format!(
                            r#"<div class="doc"><a href="/{}">{}</a></div>"#,
                            doc.section, doc.title
                        )); // TODO: base URL
                    }
                }
            }
            html.push_str("</div>");
        }
        html
    }
}
