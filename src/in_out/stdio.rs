use crate::algorithm::DuplicateGroup;

/// Format duplicate groups as human-readable text.
pub fn format_groups_text(groups: &[DuplicateGroup]) -> String {
    let mut out = String::new();
    out.push_str(&format!("{} duplicated block(s) found", groups.len()));
    for (number, group) in groups.iter().enumerate() {
        out.push_str(&format!(
            "\n\n#{} - {} copies (up to {} chars)",
            number + 1,
            group.instances.len(),
            group.max_chars()
        ));
        for instance in &group.instances {
            match &instance.lines {
                Some(lines) => out.push_str(&format!(
                    "\n  {}:{}-{}",
                    instance.path.display(),
                    lines.start(),
                    lines.end()
                )),
                None => out.push_str(&format!(
                    "\n  {} (chars {}..{})",
                    instance.path.display(),
                    instance.chars.start,
                    instance.chars.end
                )),
            }
        }
    }
    out
}
