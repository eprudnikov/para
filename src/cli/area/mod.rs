use crate::cli::context::Context;

pub struct Area {
    pub name: String
}

impl Area {
    pub fn read(name: &str, ctx: &Context) -> Self {
        Area {
            name: String::from(name)
        }
    }
}