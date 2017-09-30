#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergen {
    pub fn into_iter() -> std::vec::IntoIter<Allergen> {
        use Allergen::*;
        vec![
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ].into_iter()
    }
}

pub struct Allergies {
    score: u32,
}

impl Allergies {
    pub fn new(score: u32) -> Allergies {
        Allergies { score: score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let score = self.score;
        let allergen = *allergen as u32;
        score & allergen != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::into_iter().fold(Vec::new(), |mut vec, elem| {
            if self.is_allergic_to(&elem) {
                vec.push(elem);
            }
            vec
        })
    }
}
