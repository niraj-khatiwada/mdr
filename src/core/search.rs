/// Represents a match found in text content.
pub struct SearchResult {
    pub line_index: usize,
    pub byte_offset: usize,
    pub length: usize,
}

/// Search for a query string in content, returning all matches.
/// When case_sensitive is false, performs case-insensitive matching.
pub fn search_text(content: &str, query: &str, case_sensitive: bool) -> Vec<SearchResult> {
    if query.is_empty() {
        return Vec::new();
    }
    let mut results = Vec::new();
    let (search_content, search_query) = if case_sensitive {
        (content.to_string(), query.to_string())
    } else {
        (content.to_lowercase(), query.to_lowercase())
    };

    for (line_index, line) in search_content.lines().enumerate() {
        let mut start = 0;
        while let Some(pos) = line[start..].find(&search_query) {
            results.push(SearchResult {
                line_index,
                byte_offset: start + pos,
                length: query.len(),
            });
            start += pos + 1;
        }
    }
    results
}

/// Find which line indices contain matches (deduplicated).
pub fn matching_lines(content: &str, query: &str) -> Vec<usize> {
    if query.is_empty() {
        return Vec::new();
    }
    let query_lower = query.to_lowercase();
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&query_lower))
        .map(|(i, _)| i)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_empty_query_returns_empty() {
        assert!(search_text("hello world", "", false).is_empty());
    }

    #[test]
    fn search_no_match() {
        assert!(search_text("hello world", "xyz", false).is_empty());
    }

    #[test]
    fn search_single_match() {
        let results = search_text("hello world", "world", false);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].line_index, 0);
        assert_eq!(results[0].byte_offset, 6);
        assert_eq!(results[0].length, 5);
    }

    #[test]
    fn search_multiple_matches_same_line() {
        let results = search_text("abcabc", "abc", false);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn search_case_insensitive() {
        let results = search_text("Hello World", "hello", false);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn search_case_sensitive() {
        let results = search_text("Hello World", "hello", true);
        assert!(results.is_empty());
    }

    #[test]
    fn search_multiple_lines() {
        let results = search_text("line one\nline two\nline three", "line", false);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].line_index, 0);
        assert_eq!(results[1].line_index, 1);
        assert_eq!(results[2].line_index, 2);
    }

    #[test]
    fn matching_lines_basic() {
        let lines = matching_lines("foo\nbar\nfoo bar", "foo");
        assert_eq!(lines, vec![0, 2]);
    }

    #[test]
    fn matching_lines_empty_query() {
        assert!(matching_lines("foo", "").is_empty());
    }

    #[test]
    fn matching_lines_case_insensitive() {
        let lines = matching_lines("FOO\nbar\nFoo", "foo");
        assert_eq!(lines, vec![0, 2]);
    }
}
