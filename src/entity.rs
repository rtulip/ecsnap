use std::any::{Any, TypeId};
use std::collections::HashMap;

pub type Eid = usize;
pub struct Entity{
    id: Eid,
    components: HashMap<TypeId, Box<dyn Any>>
}