//! A Markdown checklist: the `- [ ]` / `- [x]` lines in a file such as
//! `TODO.md`. Other lines are kept as they are, so editing the list never
//! loses the notes around it.

use std::fmt;

/// One line of a checklist file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Line {
    Item { title: String, done: bool },
    Text(String),
}

/// A checklist file, line by line.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Checklist {
    lines: Vec<Line>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Items are numbered from 1, as `list` prints them.
    NoSuchItem { number: usize, items: usize },
    EmptyTitle,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSuchItem { number, items } => {
                write!(f, "there is no item {number}; the list has {items}")
            }
            Self::EmptyTitle => write!(f, "an item needs a title"),
        }
    }
}

impl std::error::Error for Error {}

impl Checklist {
    pub fn parse(text: &str) -> Self {
        let lines = text
            .lines()
            .map(|line| {
                let trimmed = line.trim_start();
                let item = |rest: &str, done| Line::Item {
                    title: rest.trim().to_owned(),
                    done,
                };
                if let Some(rest) = trimmed.strip_prefix("- [ ] ") {
                    item(rest, false)
                } else if let Some(rest) = trimmed
                    .strip_prefix("- [x] ")
                    .or_else(|| trimmed.strip_prefix("- [X] "))
                {
                    item(rest, true)
                } else {
                    Line::Text(line.to_owned())
                }
            })
            .collect();
        Self { lines }
    }

    pub fn render(&self) -> String {
        self.lines
            .iter()
            .map(|line| match line {
                Line::Item { title, done } => {
                    format!("- [{}] {title}\n", if *done { 'x' } else { ' ' })
                }
                Line::Text(text) => format!("{text}\n"),
            })
            .collect()
    }

    /// The items, in order, as `(number, title, done)`.
    pub fn items(&self) -> impl Iterator<Item = (usize, &str, bool)> {
        self.lines
            .iter()
            .filter_map(|line| match line {
                Line::Item { title, done } => Some((title.as_str(), *done)),
                Line::Text(_) => None,
            })
            .enumerate()
            .map(|(index, (title, done))| (index + 1, title, done))
    }

    /// How many items are done, and how many there are.
    pub fn progress(&self) -> (usize, usize) {
        self.items()
            .fold((0, 0), |(done, all), (_, _, d)| (done + usize::from(d), all + 1))
    }

    pub fn add(&mut self, title: &str) -> Result<(), Error> {
        let title = title.trim();
        if title.is_empty() {
            return Err(Error::EmptyTitle);
        }
        self.lines.push(Line::Item {
            title: title.to_owned(),
            done: false,
        });
        Ok(())
    }

    pub fn complete(&mut self, number: usize) -> Result<(), Error> {
        let items = self.progress().1;
        let found = self
            .lines
            .iter_mut()
            .filter_map(|line| match line {
                Line::Item { done, .. } => Some(done),
                Line::Text(_) => None,
            })
            .nth(number.wrapping_sub(1));
        match found {
            Some(done) if number > 0 => {
                *done = true;
                Ok(())
            }
            _ => Err(Error::NoSuchItem { number, items }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "# Plan\n\n- [ ] Run the tests\n- [x] Clone the sample\nSome notes.\n";

    #[test]
    fn rendering_a_parsed_file_gives_it_back() {
        assert_eq!(Checklist::parse(SAMPLE).render(), SAMPLE);
    }

    #[test]
    fn items_are_numbered_from_one_and_skip_other_lines() {
        let list = Checklist::parse(SAMPLE);
        assert_eq!(
            list.items().collect::<Vec<_>>(),
            [(1, "Run the tests", false), (2, "Clone the sample", true)]
        );
        assert_eq!(list.progress(), (1, 2));
    }

    #[test]
    fn completing_an_item_checks_only_that_item() {
        let mut list = Checklist::parse(SAMPLE);
        list.complete(1).unwrap();
        assert_eq!(list.progress(), (2, 2));
        assert!(list.render().contains("- [x] Run the tests\n"));
        assert!(list.render().contains("Some notes.\n"));
    }

    #[test]
    fn completing_a_missing_item_says_how_many_there_are() {
        let mut list = Checklist::parse(SAMPLE);
        assert_eq!(list.complete(0), Err(Error::NoSuchItem { number: 0, items: 2 }));
        assert_eq!(list.complete(3), Err(Error::NoSuchItem { number: 3, items: 2 }));
    }

    #[test]
    fn added_items_are_trimmed_and_never_empty() {
        let mut list = Checklist::default();
        list.add("  Write a test  ").unwrap();
        assert_eq!(list.add("   "), Err(Error::EmptyTitle));
        assert_eq!(list.render(), "- [ ] Write a test\n");
    }
}
