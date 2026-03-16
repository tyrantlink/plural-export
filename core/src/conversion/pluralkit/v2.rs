use std::collections::HashMap;

use chrono::Utc;

use super::ExportConversion;
use crate::models::{
    intermediary::{
        Group as IntermediaryGroup,
        IntermediaryExport,
        Member as IntermediaryMember,
        ProxyTag as IntermediaryProxyTag
    },
    pluralkit::v2::{
        Export as PluralKitExportV2,
        Group as PluralKitGroup,
        Member as PluralKitMember,
        member::ProxyTag as PluralKitProxyTag
    }
};


impl ExportConversion for PluralKitExportV2 {
    fn into_intermediary(self) -> IntermediaryExport {
        let mut export = IntermediaryExport::default();

        let mut member_id_to_group_id = HashMap::new();

        for (group_index, group) in self.groups.iter().enumerate() {
            for (member_index, ..) in group.members.iter().enumerate() {
                member_id_to_group_id
                    .insert(member_index as u64, group_index as u64);
            }
        }

        export.groups = self
            .groups
            .into_iter()
            .enumerate()
            .map(|(index, group)| IntermediaryGroup {
                avatar_url:  group.icon,
                created:     group.created.unwrap_or_else(Utc::now),
                description: group.description,
                tag:         None,
                id:          index as u64,
                name:        group.name
            })
            .collect();

        for (member_index, member) in self.members.into_iter().enumerate() {
            export.members.push(IntermediaryMember {
                avatar_url: member.avatar_url,
                banner_url: member.banner,
                birthday: member.birthday,
                created_at: member.created.unwrap_or_else(Utc::now),
                description: member.description,
                display_name: member.display_name,
                group: member_id_to_group_id
                    .get(&(member_index as u64))
                    .copied(),
                id: member_index as u64,
                last_used: None,
                message_count: member.message_count.unwrap_or_default(),
                name: member.name,
                proxy_tags: member
                    .proxy_tags
                    .into_iter()
                    .map(|tag| IntermediaryProxyTag {
                        prefix: tag.prefix,
                        suffix: tag.suffix
                    })
                    .collect(),
                tag: None
            });
        }

        export
    }
}

impl From<IntermediaryExport> for PluralKitExportV2 {
    fn from(value: IntermediaryExport) -> Self {
        todo!();
    }
}
