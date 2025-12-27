use super::source::SourceFile;
use super::parser::{extract_comments_from_content, Language};

pub struct Comment<'a> {
    pub line: usize,
    pub text: &'a str,
    pub file_name: &'a str,
}

pub fn extract_comments<'a>(files: &'a [SourceFile]) -> Vec<Comment<'a>> {
    let mut comments = Vec::new();
    
    for file in files {
        let mut line_offsets = vec![0];
        let comment_matches = extract_comments_from_content(&file.content, Language::Java);

        for (i, c) in file.content.char_indices() {
            if c == '\n' {
                line_offsets.push(i + 1);
            }
        }
        for comment_match in comment_matches {
            let text = &file.content[comment_match.start_byte..comment_match.end_byte];      
            let line_num = match line_offsets.binary_search(&comment_match.start_byte) {
                Ok(idx) => idx + 1,
                Err(idx) => idx,
            };

            comments.push(Comment {
                line: line_num,
                text: text,
                file_name: &file.name,
            });
        }
    }
    comments
}