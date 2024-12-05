use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

type RefPages = Rc<RefCell<Vec<PageRule>>>;

#[derive(Debug, Clone)]
pub struct PageRule {
    number: usize,
    pages_after: RefPages,
}

impl PageRule {
    pub fn new(number: usize) -> Self {
        Self {
            number,
            pages_after: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn add_page_after(&self, page: PageRule) {
        self.pages_after.borrow_mut().push(page);
    }

    pub fn get_number(&self) -> usize {
        self.number
    }

    pub fn get_pages_number_after(&self) -> HashSet<usize> {
        self.pages_after
            .borrow_mut()
            .iter()
            .map(|page| page.get_number())
            .collect::<HashSet<_>>()
    }
}

impl Borrow<usize> for PageRule {
    fn borrow(&self) -> &usize {
        &self.number
    }
}

impl Hash for PageRule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.number.hash(state);
    }
}

impl PartialEq for PageRule {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl Eq for PageRule {}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_page_after() {
        let page1 = PageRule::new(1);
        let page2 = PageRule::new(2);

        page1.add_page_after(page2.clone());

        assert_eq!(page1.get_pages_number_after().len(), 1);
    }

    #[test]
    fn test_page_after_from_hashset() {
        let mut page_rules_set = HashSet::new();
        let page1 = PageRule::new(1);
        let page2 = PageRule::new(2);

        page1.add_page_after(page2.clone());
        page_rules_set.insert(page1.clone());

        let page1 = page_rules_set
            .get(&1)
            .expect("Page 1 should be in the set")
            .clone();
        assert_eq!(page1.get_pages_number_after().len(), 1);
    }
}
