use super::engine::Comment;

#[derive(Debug, Clone)]
pub enum SearchMode {
    And, // all terms must match
    Or,  // any term can match
}

pub fn search<'a>(comments: &'a [Comment], query: &str, mode: SearchMode) -> Vec<&'a Comment<'a>> {
    if query.trim().is_empty() {
        return Vec::new();
    }

    let terms: Vec<&str> = query.split_whitespace().collect();

    let mut results: Vec<&'a Comment<'a>> = comments
        .iter()
        .filter(|comment| match mode {
            SearchMode::And => terms.iter().all(|term| {
                comment.text.to_lowercase().contains(&term.to_lowercase())
                    || comment.file_name.to_lowercase().contains(&term.to_lowercase())
            }),
            SearchMode::Or => terms.iter().any(|term| {
                comment.text.to_lowercase().contains(&term.to_lowercase())
                    || comment.file_name.to_lowercase().contains(&term.to_lowercase())
            }),
        })
        .collect();

    results.sort_by_key(|comment| calculate_score(comment, &terms)); // current scoring to replace

    results
}

fn calculate_score(comment: &Comment<'_>, terms: &[&str]) -> usize {
    let file_name_lower = comment.file_name.to_lowercase();
    let text_lower = comment.text.to_lowercase();

    // base score
    let mut score = 0;
    for term in terms {
        if let Some(pos) = file_name_lower.find(term) {
            score += 2000usize.saturating_sub(pos);
        } else if let Some(pos) = text_lower.find(term) {
            score += 1000usize.saturating_sub(pos);
        }
    }

    usize::MAX - score
}
