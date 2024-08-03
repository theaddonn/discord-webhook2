use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

impl EmbedField {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            value: String::new(),
            inline: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn inline(mut self, inline: bool) -> Self {
        self.inline = Some(inline);
        self
    }
}

impl Default for EmbedField {
    fn default() -> Self {
        Self::new()
    }
}