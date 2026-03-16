pub mod group;
pub mod member;
pub mod proxy_tag;

pub use self::{group::Group, member::Member, proxy_tag::ProxyTag};


#[derive(Debug, Default)]
pub struct IntermediaryExport {
    pub groups:  Vec<Group>,
    pub members: Vec<Member>
}
