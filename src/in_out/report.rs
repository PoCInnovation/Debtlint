use std::io::{self, ErrorKind};

use serde::Serialize;

use crate::algorithm::{DuplicateGroup, Instance};

#[derive(Debug, Serialize, PartialEq, Eq)]
struct Bounds {
    start: usize,
    end: usize,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
struct Fragment {
    source: String,
    tokens: Bounds,
    chars: Bounds,
    lines: Option<Bounds>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
struct Diagnostic {
    ranges: Vec<Fragment>,
}

fn fragment(instance: &Instance) -> Fragment {
    Fragment {
        source: instance.path.display().to_string(),
        tokens: Bounds {
            start: instance.tokens.start,
            end: instance.tokens.end,
        },
        chars: Bounds {
            start: instance.chars.start,
            end: instance.chars.end,
        },
        lines: instance
            .lines
            .as_ref()
            .map(|lines| Bounds {
                start: *lines.start(),
                end: *lines.end(),
            }),
    }
}

fn diagnostics(groups: &[DuplicateGroup]) -> Vec<Diagnostic> {
    groups
        .iter()
        .map(|group| Diagnostic {
            ranges: group.instances.iter().map(fragment).collect(),
        })
        .collect()
}

pub fn format_groups_json(groups: &[DuplicateGroup]) -> io::Result<String> {
    serde_json::to_string_pretty(&diagnostics(groups))
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))
}