pub(crate) mod parse;

#[derive(Debug, PartialEq)]
pub(crate) enum Comment<'a> {
    Line(&'a str),
    Block(&'a str),
}

#[cfg(test)]
mod test_parse_line_comment {
    use crate::parser::comment::{Comment, parse::parse_line_comment};

    
    #[test]
    fn test_parse_line_comment_single_line() {
        assert_eq!(
            parse_line_comment("// This is a comment\n"),
            Ok(("\n", Comment::Line(" This is a comment")))
        );
    }

    #[test]
    fn test_parse_line_comment_single_line_with_trailing_whitespace() {
        assert_eq!(
            parse_line_comment("// This is another comment with trailing whitespace    \n"),
            Ok((
                "\n",
                Comment::Line(" This is another comment with trailing whitespace    ")
            ))
        );
    }

    #[test]
    fn test_parse_line_comment_single_line_empty() {
        assert_eq!(
            parse_line_comment("//\n"),
            Ok(("\n", Comment::Line("")))
        );
    }

    #[test]
    fn test_parse_line_comment_single_line_with_leading_whitespace() {
        assert!(
            parse_line_comment("  // This is a comment with leading whitespace\n")
            .is_err()
        );
    }

    #[test]
    fn test_parse_line_comment_multi_line() {
        assert_eq!(
            parse_line_comment("// This is a comment that extends over multiple lines\n// with another line\n"),
            Ok((
                "\n// with another line\n",
                Comment::Line(" This is a comment that extends over multiple lines")
            ))
        );
    }

    #[test]
    fn test_parse_line_comment_block_comment() {
        assert_eq!(
            parse_line_comment("/* This is a block comment\n and it's on \n 3 lines */\n"),
            Ok((
                "\n",
                Comment::Block(" This is a block comment\n and it's on \n 3 lines ")
            ))
        );
    }
}

#[cfg(test)]
mod test_empty_comments_and_whitespace {
    use crate::parser::comment::parse::many1_comments_or_whitespace;

    
    #[test]
    fn test_empty_string() {
        assert!(
            many1_comments_or_whitespace("")
            .is_err()
        );
    }

    #[test]
    fn test_new_line() {
        assert_eq!(
            many1_comments_or_whitespace("\n"),
            Ok(("", 1))
        );
    }

    #[test]
    fn test_empty_comments_and_whitespace() {
        assert_eq!(
            many1_comments_or_whitespace("  // This is a comment\n  // with another line\n"),
            Ok(("", 5))
        );
    }

    #[test]
    fn test_empty_comments_and_whitespace_with_leading_whitespace() {
        assert_eq!(
            many1_comments_or_whitespace("  // This is a comment\n  // with another line\n"),
            Ok(("", 5))
        );
    }
}
