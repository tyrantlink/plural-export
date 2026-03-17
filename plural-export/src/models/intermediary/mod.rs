mod enums;
mod group;
mod member;
mod proxy_tag;

pub use self::{
    enums::Image,
    group::Group,
    member::Member,
    proxy_tag::ProxyTag
};


#[derive(Debug, Default)]
pub struct IntermediaryExport {
    pub groups:  Vec<Group>,
    pub members: Vec<Member>
}
