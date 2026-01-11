use serde::{Deserialize, Serialize};
use crate::types::profile::Profile;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub profiles: HashMap<String, Profile>,
}

impl AppConfig {
    pub fn get_profile(&self, id: &str) -> Option<&Profile> {
        self.profiles.get(id)
    }

    pub fn add_profile(&mut self, profile: Profile) {
        self.profiles.insert(profile.id.clone(), profile);
    }
    
    pub fn remove_profile(&mut self, id: &str) -> Option<Profile> {
        self.profiles.remove(id)
    }
}
