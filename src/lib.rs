use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Allergies {
    allergy_to: HashMap<Allergen, bool>,
}

#[derive(Debug, PartialEq, Eq, Hash, EnumIter, Clone, Copy)]
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

impl Allergen {
    fn get_default_allergen_hashmap() -> HashMap<Allergen, bool> {
        HashMap::from_iter(Allergen::iter().map(|x| (x, false)))
    }

    fn get_scores_to_allergen_hashmap() -> HashMap<u32, Allergen> {
        HashMap::from_iter(
            Allergen::iter()
                .enumerate()
                .map(|(i, x)| ((2 as u32).pow(i as u32), x)),
        )
    }

    pub fn get_allergen_hashmap(score: u32) -> HashMap<Allergen, bool> {
        let initial_hashmap = Allergen::get_default_allergen_hashmap();
        Allergen::get_allergen_hashmap_rec(
            score,
            256,
            initial_hashmap,
            Allergen::get_scores_to_allergen_hashmap(),
        )
    }

    fn get_allergen_hashmap_rec(
        score: u32,
        single_allergen_score: u32,
        acc: HashMap<Allergen, bool>,
        scores_to_allergen_hashmap: HashMap<u32, Allergen>,
    ) -> HashMap<Allergen, bool> {
        if score <= 0 {
            return acc;
        }
        let next_single_allergen_score = single_allergen_score / 2;
        if score < single_allergen_score {
            return Allergen::get_allergen_hashmap_rec(
                score,
                next_single_allergen_score,
                acc,
                scores_to_allergen_hashmap,
            );
        } else {
            let next_score = score - single_allergen_score;
            let allergen = scores_to_allergen_hashmap
                .get(&single_allergen_score)
                .unwrap();
            let next_acc = functional_insert_to_hashmap(acc, allergen, true);
            return Allergen::get_allergen_hashmap_rec(
                next_score,
                next_single_allergen_score,
                next_acc,
                scores_to_allergen_hashmap,
            );
        }
    }
}

fn functional_insert_to_hashmap<
    T: std::hash::Hash + Clone + PartialEq + std::cmp::Eq + Copy,
    U: Clone + PartialEq,
>(
    hashmap: HashMap<T, U>,
    k: &T,
    v: U,
) -> HashMap<T, U> {
    let mut new_hashmap = hashmap.clone();
    new_hashmap.insert(*k, v);
    new_hashmap
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        // todo!("Given the '{score}' score, construct a new Allergies struct.");
        let limited_score = score % 256;
        Allergies {
            allergy_to: Allergen::get_allergen_hashmap(limited_score),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        *self.allergy_to.get(allergen).unwrap()
        // todo!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let allergy_to = &self.allergy_to;
        let res = allergy_to
            .into_iter()
            .filter(|(_allergen, is_allergic)| **is_allergic == true)
            .map(|(allergen, _is_allergic)| *allergen)
            .collect::<Vec<Allergen>>();
        res
        // todo!("Return the list of allergens contained within the score with which the Allergies struct was made.");
    }
}
