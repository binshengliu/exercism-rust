#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    elements: Vec<T>,
}

impl<T> CustomSet<T>
where
    T: PartialEq + Clone + Ord,
{
    pub fn new(v: Vec<T>) -> CustomSet<T> {
        let mut s = CustomSet { elements: v };
        s.elements.sort();
        s
    }

    pub fn is_subset(&self, super_set: &CustomSet<T>) -> bool {
        self.elements.iter().all(|e| super_set.elements.contains(e))
    }

    pub fn is_disjoint(&self, another: &CustomSet<T>) -> bool {
        self.elements.iter().all(|e| !another.elements.contains(e))
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn contains(&self, e: &T) -> bool {
        self.elements.contains(e)
    }

    pub fn add(&mut self, e: T) {
        if !self.elements.contains(&e) {
            self.elements.push(e);
            self.elements.sort();
        }
    }

    pub fn intersection(&self, another: &CustomSet<T>) -> CustomSet<T> {
        let v: Vec<T> = self.elements
            .iter()
            .filter(|e| another.contains(e))
            .cloned()
            .collect();
        CustomSet::new(v)
    }

    pub fn difference(&self, another: &CustomSet<T>) -> CustomSet<T> {
        let v: Vec<T> = self.elements
            .iter()
            .filter(|e| !another.contains(e))
            .cloned()
            .collect();
        CustomSet::new(v)
    }

    pub fn union(&self, another: &CustomSet<T>) -> CustomSet<T> {
        let mut v = self.elements.clone();
        for e in another.elements.iter() {
            if !v.contains(e) {
                v.push(e.clone());
            }
        }
        v.sort();
        CustomSet::new(v)
    }
}
