use anyhow::Result;
use clap::ArgMatches;
use std::collections::HashMap;

#[inline]
pub(crate) fn strings_from_matches(matches: &ArgMatches, key: &str) -> Option<Vec<String>> {
    matches.values_of(key).map(|files| {
        files
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
    })
}

#[inline]
pub(crate) fn hashmaps_from_matches(
    matches: &ArgMatches,
    key: &str,
) -> Option<Result<HashMap<String, String>>> {
    matches.values_of(key).map(|files| {
        files
            .into_iter()
            .map(|s| {
                let split_values = s.split('=').collect::<Vec<&str>>();
                if split_values.len() == 2
                    && !split_values[0].trim().is_empty()
                    && !split_values[1].trim().is_empty()
                {
                    Ok((split_values[0].to_string(), split_values[1].to_string()))
                } else {
                    Err(anyhow::anyhow!("Invalid value for top level arguments"))
                }
            })
            .collect::<Result<HashMap<String, String>>>()
    })
}

#[inline]
pub(crate) fn string_from_matches(matches: &ArgMatches, key: &str) -> Option<String> {
    matches.value_of(key).map(|v| v.to_string())
}

#[inline]
pub(crate) fn bool_from_matches(matches: &ArgMatches, key: &str) -> Option<bool> {
    let occurrences = matches.occurrences_of(key);
    if occurrences > 0 {
        Some(true)
    } else {
        None
    }
}

#[inline]
pub(crate) fn u32_from_matches(matches: &ArgMatches, key: &str) -> Option<u32> {
    let occurrences = matches.occurrences_of(key);
    if occurrences > 0 {
        Some(occurrences as u32)
    } else {
        None
    }
}
