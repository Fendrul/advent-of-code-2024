use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

type RefPages = Rc<RefCell<HashSet<usize>>>;

#[derive(Debug, Clone)]
pub struct PageRule {
    number: usize,
    pages_after: RefPages,
}

impl PageRule {
    pub fn new(number: usize) -> Self {
        Self {
            number,
            pages_after: Rc::new(RefCell::new(HashSet::new())),
        }
    }

    pub fn add_page_number_after(&self, page: &usize) {
        self.pages_after.borrow_mut().insert(*page);
    }

    pub fn get_number(&self) -> usize {
        self.number
    }

    pub fn get_ref_pages_number_after(&self) -> Ref<'_, HashSet<usize>> {
        self.pages_after.as_ref().borrow()
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
impl Default for PageRule {
    fn default() -> Self {
        Self::new(0)
    }
}
