use crate::enums::TestEnum;
use std::collections::HashMap;

use yui::{YuiAttribute};

#[derive(YuiAttribute, Clone)]
pub struct NoField;

#[derive(YuiAttribute, Clone)]
pub struct Simple {
    #[attribute_field(alias = "i32")]
    pub int32: i32,
    #[attribute_field(alias = "u16")]
    pub unsigned16: u16,
    pub float: f32,
    pub string: String,
    #[attribute_field(enum_value = true, default = "variant_c")]
    pub enum1: Option<TestEnum>,
    #[attribute_field(enum_value = true)]
    pub enum2: TestEnum,
}

//#[derive(YuiAttribute, Clone)]
pub struct Tuple(i32, Option<String>);

#[derive(YuiAttribute, Clone)]
pub struct Full {
    pub object: Simple,
    pub vector: Vec<String>,
    #[attribute_field(enum_value = true)]
    pub map: HashMap<String, TestEnum>,
}
