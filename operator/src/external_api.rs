use warp::Reply;
use warp::filters::BoxedFilter;
use crate::VendorService;
use std::sync::Arc;
use crate::OperatorService;
use warp::Filter;
use warp::reply;


pub struct ExternalApi {
  operator: Arc<OperatorService>,
  vendor: Arc<VendorService>,
}

impl ExternalApi {
    pub fn new(operator: OperatorService, vendor: VendorService) -> Self {
        Self {
            operator: Arc::new(operator),
            vendor: Arc::new(vendor),
        }
    }

    pub fn into_routes(self) -> BoxedFilter<(impl Reply,)>  {
        let operator = self.operator;
        let operator = warp::path!("operator")
            .map(move || {
                reply::json(&operator.get_sites())
            });
        
        let vendor = self.vendor;
        warp::path!("vendor" / u64)
            .map(move |id| {
                reply::json(&vendor.get_state(id))
            })
            .or(operator).boxed()
    }
}