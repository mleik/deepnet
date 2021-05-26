use deepnet_operator::VendorService;
use deepnet_operator::OperatorService;
use warp::Filter;
use deepnet_operator::InternalApi;
use deepnet_operator::SiteService;
use deepnet_operator::ExternalApi;

#[tokio::main]
async fn main() {
    let site = SiteService{};
    let vendor = VendorService{};
    let operator = OperatorService{};
    
    let external = ExternalApi::new(operator, vendor);
    let internal = InternalApi::new(site);

    let external_routes = external.into_routes();
    let internal_routes = internal.into_routes();

    println!("Starting api server on http://localhost:8080/");
    warp::serve(external_routes.or(internal_routes))
        .run(([0, 0, 0, 0], 8080))
        .await;
}