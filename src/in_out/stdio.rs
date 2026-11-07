use std::collections::BTreeSet;
use std::fs;
use std::io::IsTerminal;
use std::ops::RangeInclusive;
use std::path::Path;

use crate::algorithm::{DuplicateGroup, Instance};

const MAX_PREVIEW_LINES: usize = 8;
const MAX_LINE_WIDTH: usize = 100;

pub fn color_enabled() -> bool {
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if std::env::var("CLICOLOR_FORCE").as_deref() == Ok("1") {
        return true;
    }
    std::io::stdout().is_terminal()
}

#[derive(Clone, Copy)]
struct Style {
    color: bool,
}

impl Style {
    fn new(color: bool) -> Self {
        Self { color }
    }

    fn paint(self, code: &str, text: &str) -> String {
        if self.color {
            format!("\x1b[{code}m{text}\x1b[0m")
        } else {
            text.to_string()
        }
    }

    fn bold(self, text: &str) -> String {
        self.paint("1", text)
    }

    fn cyan(self, text: &str) -> String {
        self.paint("36", text)
    }

    fn yellow(self, text: &str) -> String {
        self.paint("33", text)
    }

    fn dim(self, text: &str) -> String {
        self.paint("2", text)
    }
}

struct Summary {
    groups: usize,
    occurrences: usize,
    files: usize,
    tokens: usize,
}

fn summarize(groups: &[DuplicateGroup]) -> Summary {
    let mut files = BTreeSet::new();
    let mut occurrences = 0;
    let mut tokens = 0;
    for group in groups {
        occurrences += group.instances.len();
        for instance in &group.instances {
            files.insert(instance.path.clone());
            tokens += instance.tokens.end.saturating_sub(instance.tokens.start);
        }
    }
    Summary {
        groups: groups.len(),
        occurrences,
        files: files.len(),
        tokens,
    }
}

fn display_path(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .or_else(|_| path.strip_prefix("./"))
        .unwrap_or(path)
        .display()
        .to_string()
}

fn location_label(instance: &Instance, root: &Path) -> String {
    let path = display_path(&instance.path, root);
    match &instance.lines {
        Some(lines) if lines.start() == lines.end() => {
            format!("{path}:{}", lines.start())
        }
        Some(lines) => format!("{path}:{}-{}", lines.start(), lines.end()),
        None => format!(
            "{path} (chars {}..{})",
            instance.chars.start, instance.chars.end
        ),
    }
}

fn truncate_line(line: &str) -> String {
    let mut truncated: String = line.chars().take(MAX_LINE_WIDTH).collect();
    if line.chars().count() > MAX_LINE_WIDTH {
        truncated.push('…');
    }
    truncated
}

fn read_preview_lines(path: &Path, lines: &RangeInclusive<usize>) -> Option<Vec<(usize, String)>> {
    let content = fs::read_to_string(path).ok()?;
    let start = *lines.start();
    let end = *lines.end();
    if start == 0 || end < start {
        return None;
    }
    let limit = (end - start + 1).min(MAX_PREVIEW_LINES);
    let mut out = Vec::with_capacity(limit);
    for (index, line) in content.lines().enumerate() {
        let number = index + 1;
        if number < start {
            continue;
        }
        if number > end || out.len() >= limit {
            break;
        }
        out.push((number, truncate_line(line)));
    }
    if out.is_empty() { None } else { Some(out) }
}

fn format_preview(instance: &Instance, root: &Path, style: Style) -> Option<String> {
    let lines = instance.lines.as_ref()?;
    let preview = read_preview_lines(&instance.path, lines)?;
    let shown = preview.len();
    let total = lines.end().saturating_sub(*lines.start()) + 1;
    let path = display_path(&instance.path, root);
    let mut out = String::new();
    out.push_str(&style.dim(&format!(
        "  preview {}:{}-{}",
        path,
        lines.start(),
        lines.end()
    )));
    let width = preview
        .last()
        .map(|(number, _)| number.to_string().len())
        .unwrap_or(1);
    for (number, line) in preview {
        out.push('\n');
        out.push_str(&style.dim(&format!("  {number:>width$} │ ")));
        out.push_str(&line);
    }
    if shown < total {
        out.push('\n');
        out.push_str(&style.dim(&format!("  … {} more line(s)", total - shown)));
    }
    Some(out)
}

fn format_group(number: usize, group: &DuplicateGroup, root: &Path, style: Style) -> String {
    let tokens = group
        .instances
        .iter()
        .map(|instance| instance.tokens.end.saturating_sub(instance.tokens.start))
        .max()
        .unwrap_or(0);
    let mut out = String::new();
    out.push_str(&style.bold(&format!("#{number}")));
    out.push(' ');
    out.push_str(&style.dim(&format!(
        "{} copies · {} tokens · up to {} chars",
        group.instances.len(),
        tokens,
        group.max_chars()
    )));
    for instance in &group.instances {
        out.push('\n');
        out.push_str("  ");
        out.push_str(&style.cyan(&location_label(instance, root)));
    }
    if let Some(preview) = group
        .instances
        .iter()
        .find_map(|instance| format_preview(instance, root, style))
    {
        out.push('\n');
        out.push_str(&preview);
    }
    out
}

fn format_summary(summary: &Summary, style: Style) -> String {
    let mut out = String::new();
    out.push_str(&style.bold("Debtlint report"));
    out.push('\n');
    out.push_str(&style.dim("───────────────"));
    for (label, value) in [
        ("groups", summary.groups),
        ("occurrences", summary.occurrences),
        ("files", summary.files),
        ("tokens", summary.tokens),
    ] {
        out.push('\n');
        out.push_str(&format!(
            "  {} {}",
            style.dim(&format!("{:<12}", format!("{label}:"))),
            style.yellow(&value.to_string())
        ));
    }
    out
}

/// Format duplicate groups as a concise human-readable terminal report.
pub fn format_groups_text(groups: &[DuplicateGroup], root: &Path, color: bool) -> String {
    let style = Style::new(color);
    let summary = summarize(groups);
    let mut out = format_summary(&summary, style);
    if groups.is_empty() {
        out.push_str("\n\n");
        out.push_str(&style.dim("No duplicated blocks found."));
        return out;
    }
    for (index, group) in groups.iter().enumerate() {
        out.push_str("\n\n");
        out.push_str(&format_group(index + 1, group, root, style));
    }
    out
}
