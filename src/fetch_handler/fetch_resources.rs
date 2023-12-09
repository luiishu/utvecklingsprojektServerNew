pub trait FetchConstants {
    const ADDRESS: &'static str = "address";
    const USER: &'static str = "user";
    const PRODUCT: &'static str = "user";
    const PRODUCT_IMAGE: &'static str = "user";
    const PRODUCT_REVIEW: &'static str = "user";
    const PRODUCT_TYPE: &'static str = "user";
}

pub struct FetchResource;

pub trait TableName {
    const TABLE_NAME: &'static str;
}

pub struct Admin;

impl TableName for Admin {
    const TABLE_NAME: &'static str = "Admin";
}

impl FetchConstants for FetchResource {}

