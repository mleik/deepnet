use serde::{Serialize};

pub struct VendorService {
    
}

impl VendorService {
    pub fn get_state(&self, vendor_id: u64) ->  VendorState{
        VendorState{
            id: vendor_id,
            name: format!("DummyVendor{}",vendor_id)
        }
    }
}

#[derive(Serialize)]
pub struct VendorState {
  id:u64,
  name: String
}