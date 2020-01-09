use ansi_term::{Color, Style};
use once_cell::sync::Lazy;
use std::fmt;
use std::sync::Mutex;

static LABEL_CACHE: Lazy<Mutex<LabelCache>> = Lazy::new(|| Mutex::new(LabelCache::default()));

#[derive(Default)]
struct LabelCache(Vec<String>);

impl LabelCache {
    fn insert(&mut self, label: &str) -> Label {
        match self.0.iter().position(|x| x == label) {
            Some(x) => Label(x),
            None => {
                self.0.push(label.to_string());
                Label(self.0.len() - 1)
            }
        }
    }
}

fn idx_to_color(idx: usize) -> Color {
    match idx % 4 {
        0 => Color::Purple,
        1 => Color::Yellow,
        2 => Color::Cyan,
        3 => Color::Green,
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Ord, Eq, Copy)]
pub struct Label(usize);
impl From<&str> for Label {
    fn from(x: &str) -> Label {
        let mut cache = LABEL_CACHE.lock().unwrap();
        cache.insert(x)
    }
}
impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cache = LABEL_CACHE.lock().unwrap();
        let color = idx_to_color(self.0);
        let s = &cache.0[self.0];
        write!(f, "{}", Style::new().fg(color).paint(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roudtrip() {
        assert_eq!(
            Label::from("foobar").to_string(),
            "\u{1b}[35mfoobar\u{1b}[0m"
        );
        assert_eq!(
            Label::from("barqux").to_string(),
            "\u{1b}[33mbarqux\u{1b}[0m"
        );
        assert_eq!(
            Label::from("barqux").to_string(),
            "\u{1b}[33mbarqux\u{1b}[0m"
        );
        assert_eq!(
            Label::from("foobar").to_string(),
            "\u{1b}[35mfoobar\u{1b}[0m"
        );
    }
}
