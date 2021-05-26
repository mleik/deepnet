use std::sync::Arc;
use crate::internal_api::reply::Json;
use warp::Filter;
use warp::Rejection;
use warp::reply;
use crate::SiteService;

pub struct InternalApi {
    site: Arc<SiteService>
}

impl InternalApi {
    pub fn new(site: SiteService) -> Self {
        InternalApi {
            site: Arc::new(site)
        }
    }

    pub fn into_routes(self) -> impl Filter<Extract = (Json,), Error = Rejection> + Clone  {
        let site = self.site;
        warp::path!("site" / u64)
            .map(move |id| {
                reply::json(&site.get_state(id))
            })
    }
}