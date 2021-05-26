use serde::{Serialize};

pub struct SiteService {
    
}

impl SiteService {
    pub fn get_state(&self, site_id: u64) -> SiteState {
        SiteState{
            id: site_id,
            name: format!("DummySite{}",site_id)
        }
    }
}

#[derive(Serialize)]
pub struct SiteState {
  id:u64,
  name: String
}