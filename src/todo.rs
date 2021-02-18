use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Todo {
    pub line: u32,
    pub prefix: String,
    pub keyword: String,
    pub title: String,
    pub issue_id: Option<u32>,
    pub comments: Vec<String>,
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let issue_str = if let Some(ref issue) = self.issue_id {
            format!("(#{})", issue)
        } else {
            String::new()
        };

        let comments_str = if self.comments.len() == 0 {
            String::new()
        } else {
            let mut out = String::from("\n  ");
            out.push_str(&self.comments.join("\n  "));

            out
        };

        write!(
            f,
            "{}: {}{}: {}{}\n",
            self.line, self.keyword, issue_str, self.title, comments_str
        )
    }
}

impl Todo {
    pub fn reported_view(&self) -> Option<String> {
        self.issue_id
            .map(|i| format!("{}{}(#{}): {}", self.prefix, self.keyword, i, self.title))
    }

    pub fn unreported_view(&self) -> String {
        format!("{}{}: {}", self.prefix, self.keyword, self.title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_without_issue_and_comments() {
        let todo = Todo {
            line: 10,
            prefix: String::from("// "),
            keyword: String::from("TODO"),
            title: String::from("Something"),
            issue_id: None,
            comments: vec![],
        };

        assert_eq!("10: TODO: Something\n", format!("{}", todo))
    }

    #[test]
    fn display_with_issue() {
        let todo = Todo {
            line: 10,
            prefix: String::from("// "),
            keyword: String::from("TODO"),
            title: String::from("Something"),
            issue_id: Some(42),
            comments: vec![],
        };

        assert_eq!("10: TODO(#42): Something\n", format!("{}", todo))
    }

    #[test]
    fn display_with_comments() {
        let todo = Todo {
            line: 10,
            prefix: String::from("// "),
            keyword: String::from("TODO"),
            title: String::from("Something"),
            issue_id: None,
            comments: vec!["More".to_owned(), "And More".to_owned()],
        };

        assert_eq!(
            "10: TODO: Something\n  More\n  And More\n",
            format!("{}", todo)
        )
    }

    #[test]
    fn reported_view_none() {
        let todo = Todo {
            line: 10,
            prefix: String::from("// "),
            keyword: String::from("TODO"),
            title: String::from("Something"),
            issue_id: None,
            comments: vec!["More".to_owned(), "And More".to_owned()],
        };

        assert_eq!(None, todo.reported_view())
    }

    #[test]
    fn reported_view_some() {
        let todo = Todo {
            line: 10,
            prefix: String::from("// "),
            keyword: String::from("TODO"),
            title: String::from("Something"),
            issue_id: Some(123),
            comments: vec!["More".to_owned(), "And More".to_owned()],
        };

        assert_eq!(
            Some("// TODO(#123): Something".to_owned()),
            todo.reported_view()
        )
    }

    #[test]
    fn unreported_view() {
        let todo = Todo {
            line: 10,
            prefix: String::from("// "),
            keyword: String::from("TODO"),
            title: String::from("Something"),
            issue_id: Some(123),
            comments: vec!["More".to_owned(), "And More".to_owned()],
        };

        assert_eq!("// TODO: Something".to_owned(), todo.unreported_view())
    }
}
