use serde::{Serialize};

pub struct OperatorService {
    
}

impl OperatorService {
    pub fn get_sites(&self) ->  Vec<SiteState>{
        vec![
            SiteState{ id: 1,name: "DummySite1".into()},
            SiteState{ id: 2,name: "DummySite2".into()},
            SiteState{ id: 3,name: "DummySite3".into()},
            SiteState{ id: 4,name: "DummySite4".into()},
            ]
    }
}

#[derive(Serialize)]
pub struct SiteState {
  id:u64,
  name: String
}