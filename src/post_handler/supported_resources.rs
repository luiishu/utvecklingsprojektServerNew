pub trait SupportedResourcesConstants {
    const USERS: &'static str = "users";
    const ORDERS: &'static str = "orders";
}

pub struct SupportedResources;
impl SupportedResourcesConstants for SupportedResources {
    const USERS: &'static str = "users";
    const ORDERS: &'static str = "orders";
}

