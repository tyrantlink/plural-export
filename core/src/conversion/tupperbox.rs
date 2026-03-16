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
    tupperbox::{
        Group as TupperboxGroup,
        Tupper as TupperboxTupper,
        TupperboxExport
    }
};


impl ExportConversion for TupperboxExport {
    fn into_intermediary(self) -> IntermediaryExport {
        let user_id = self.find_user_id();
        let mut export = IntermediaryExport::default();

        let mut group_id_map = HashMap::new();

        for (index, group) in self.groups.into_iter().enumerate() {
            group_id_map.insert(group.id, index);

            export.groups.push(IntermediaryGroup {
                avatar_url:  group.avatar.zip(user_id).map(|(avatar_hash, user_id)| {
                    format!("https://cdn.tupperbox.app/group-pfp/{user_id}/{avatar_hash}.webp")
                }),
                created:     Utc::now(),
                description: group.description,
                id:          index as u64,
                name:        group.name,
                tag:         group.tag
            });
        }

        for (index, tupper) in self.tuppers.into_iter().enumerate() {
            export.members.push(IntermediaryMember {
                avatar_url: tupper.avatar_url,
                banner_url: tupper.banner.zip(user_id).map(|(banner_hash, user_id)| {
                    format!("https://cdn.tupperbox.app/banner/{user_id}/{banner_hash}.webp")
                }),
                birthday: tupper.birthday,
                created_at: tupper.created_at,
                description: tupper.description,
                display_name: tupper.nick,
                group: group_id_map.get(&tupper.group_id).map(|id| *id as u64),
                id: index as u64,
                last_used: Some(tupper.last_used),
                message_count: tupper.posts,
                name: tupper.name,
                proxy_tags: {
                    let [prefix, suffix] = tupper.brackets;

                    match (prefix.is_empty(), suffix.is_empty()) {
                        (true, true) => Vec::new(),
                        (true, false) => vec![IntermediaryProxyTag { prefix: None, suffix: Some(suffix) }],
                        (false, true) => vec![IntermediaryProxyTag { prefix: Some(prefix), suffix: None }],
                        (false, false) => vec![IntermediaryProxyTag { prefix: Some(prefix), suffix: Some(suffix) }],
                    }
                },
                tag: tupper.tag
            });
        }

        export
    }
}

impl From<IntermediaryExport> for TupperboxExport {
    fn from(value: IntermediaryExport) -> Self {
        Self {
            groups:  value
                .groups
                .into_iter()
                .map(|group| TupperboxGroup {
                    avatar:      group
                        .avatar_url
                        .as_ref()
                        .and_then(|url| url.rsplit_once('/'))
                        .and_then(|(_, file)| {
                            file.split_once('.')
                                .map(|(avatar, _)| avatar.to_owned())
                        }),
                    description: group.description,
                    id:          group.id,
                    name:        group.name,
                    tag:         group.tag
                })
                .collect(),
            tuppers: value
                .members
                .into_iter()
                .map(|member| TupperboxTupper {
                    avatar: member
                        .avatar_url
                        .as_ref()
                        .and_then(|url| url.rsplit_once('/'))
                        .and_then(|(_, file)| {
                            file.split_once('.')
                                .map(|(avatar, _)| avatar.to_owned())
                        }),
                    avatar_url: member.avatar_url,
                    banner: member.banner_url,
                    birthday: member.birthday,
                    // TODO probably do this better
                    brackets: if let Some(proxy_tag) =
                        member.proxy_tags.into_iter().next()
                    {
                        [
                            proxy_tag.prefix.unwrap_or_default(),
                            proxy_tag.suffix.unwrap_or_default()
                        ]
                    } else {
                        [String::new(), String::new()]
                    },
                    created_at: member.created_at,
                    description: member.description,
                    group_id: member.group.unwrap_or(0),
                    id: member.id,
                    last_used: member.last_used.unwrap_or_else(Utc::now),
                    name: member.name,
                    nick: member.display_name,
                    posts: member.message_count,
                    show_brackets: false,
                    tag: member.tag
                })
                .collect()
        }
    }
}
