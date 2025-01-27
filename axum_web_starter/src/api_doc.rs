use utoipa::OpenApi;
use crate::models::customer::Customer;
use crate::models::customer::CustomerPayload;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::customer_handler::create_customer_api,
        crate::handlers::customer_handler::list_customers_api,
        crate::handlers::customer_handler::get_customer_api,
        crate::handlers::customer_handler::update_customer_api,
        crate::handlers::customer_handler::delete_customer_api,
    ),
    components(
        schemas(Customer, CustomerPayload)
    ),
    tags(
        (name = "Customers", description = "API for managing customers")
    )
)]
pub struct ApiDoc;
