use serde::Serialize;
use typescript_type_def::TypeDef;

#[derive(Debug, Clone, Serialize, TypeDef)]
pub struct ListItemsResponse {}
