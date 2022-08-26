pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        // simply return struct with the score; the allergies can be inferred from the score easily
        // by calling self.allergies()
        return Self{ score };
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        // retrieve the list of allergies
        let allergens: Vec<Allergen> = self.allergies();

        // check for allergen in the allergies list
        return allergens.iter().any(|i| i==allergen);
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        // initiate allergens vector
        let mut allergens: Vec<Allergen> = vec![];

        // the scores are given as 2**0 -> 2**7, i.e. it can be represented by a byte. E.g. if you
        // were allergic to eggs and strawberries, you score would be (in binary form) "00001001",
        // or 9. This solution seeks to use this to solve the exercise.
        //
        // format the score as an 8-digit binary string
        let binary_string = format!("{:0>8b}", self.score as u8);

        // loop through string, and match with allergens
        for (index, bin_val) in binary_string.chars().enumerate() {
            if bin_val == '1' {
                match index {
                    7 => allergens.push(Allergen::Eggs),
                    6 => allergens.push(Allergen::Peanuts),
                    5 => allergens.push(Allergen::Shellfish),
                    4 => allergens.push(Allergen::Strawberries),
                    3 => allergens.push(Allergen::Tomatoes),
                    2 => allergens.push(Allergen::Chocolate),
                    1 => allergens.push(Allergen::Pollen),
                    0 => allergens.push(Allergen::Cats),
                    _ => println!("Something went wrong with matching the score."),
                }
            }
        }

        return allergens;
    }
}
