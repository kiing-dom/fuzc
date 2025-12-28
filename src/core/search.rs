use super::engine::Comment;

pub fn search<'a>(comments: &'a [Comment], query: &str) -> Vec<&'a Comment<'a>> {
    if query.trim().is_empty() {
        return Vec::new();
    }

    let query_lower = query.to_ascii_lowercase();

    let mut results: Vec<&'a Comment<'a>> = comments.iter()
        .filter(|comment| {
            comment.text.to_lowercase().contains(&query_lower)
        })
        .collect();

    results.sort_by_key(|comment| {
        comment.text.to_lowercase().find(&query_lower).unwrap_or(usize::MAX)
    });

    results
}