use std::collections::{HashMap, HashSet};

use tracing::info;

use super::provider::EpisodeResource;
use crate::entity::subscription;

pub fn filter_episode<'a>(
    resources: &'a Vec<EpisodeResource>,
    sub: &'a subscription::Model,
    episode_numbers_need_fetch: &HashMap<u32, HashSet<u32>>,
) -> Vec<&'a EpisodeResource> {
    let mut res = vec![];
    let mut episode_added: HashSet<(u32, u32)> = HashSet::new();

    for r in resources {
        if r.episode.season_number.is_none() {
            info!(sub_id = sub.id, "can not find season number from {}", r.raw_name);
            continue;
        } else if !episode_numbers_need_fetch.contains_key(&r.episode.season_number.unwrap()) {
            continue;
        }

        if r.episode.episode_number.is_none() {
            info!(sub_id = sub.id, "can not find episode number from {}", r.raw_name);
            continue;
        } else if !episode_numbers_need_fetch
            .get(&r.episode.season_number.unwrap())
            .unwrap()
            .contains(&r.episode.episode_number.unwrap())
        {
            continue;
        }

        if sub.resolutions.is_some() {
            if r.episode.resolution.is_none() {
                info!(sub_id = sub.id, "can not find resolution from {}", r.raw_name,);
                continue;
            } else if !sub
                .resolutions
                .as_ref()
                .unwrap()
                .contains(r.episode.resolution.as_ref().unwrap())
            {
                continue;
            }
        }

        if sub.subtitles.is_some() {
            if r.episode.subtitles.is_none() {
                info!(sub_id = sub.id, "can not find subtitles from {}", r.raw_name,);
                continue;
            } else {
                let needed_subtitles = sub.subtitles.as_ref().unwrap();
                let episode_subtitles = r.episode.subtitles.as_ref().unwrap();

                let mut found = false;
                for s in episode_subtitles {
                    if needed_subtitles.contains(s) {
                        found = true;
                    }
                }
                if !found {
                    info!(sub_id = sub.id, "subtitle not match from {}", r.raw_name,);
                    continue;
                }
            }
        }

        // match
        let episode_key = (r.episode.season_number.unwrap(), r.episode.episode_number.unwrap());
        if !episode_added.contains(&episode_key) {
            episode_added.insert(episode_key);

            info!(
                sub_id = sub.id,
                "matched, season {}, episode {} from {}",
                r.episode.season_number.unwrap(),
                r.episode.episode_number.unwrap(),
                r.raw_name,
            );
            res.push(r);
        } else {
            // already added to result list
        }
    }

    res
}
