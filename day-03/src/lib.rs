use std::collections::HashSet;

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================

pub type PriorityScore = usize;
pub type CompartmentItem = char;
pub type Compartment = Vec<CompartmentItem>;

#[derive(PartialEq, Eq, Clone, Copy)]
enum CompartmentItemType {
    Lowercase,
    Uppercase
}

// =============================================== AUXILIARY FUNCTIONS ===============================================

fn item_to_item_type(item: &CompartmentItem) -> CompartmentItemType {
    
    return match item.is_lowercase() {
        true => CompartmentItemType::Lowercase,
        false => CompartmentItemType::Uppercase
    }
}

fn item_priority_value(item: &CompartmentItem) -> PriorityScore {

    let item_reduced: CompartmentItem = *item.to_lowercase()
        .collect::<Vec<CompartmentItem>>()
        .get(0)
        .unwrap();

    let base_value: PriorityScore = 'a' as PriorityScore;
    let reduced_value: PriorityScore = item_reduced as PriorityScore;

    let priority_reduced = reduced_value - base_value + 1;

    return match item_to_item_type(item) {
        CompartmentItemType::Lowercase => priority_reduced,
        CompartmentItemType::Uppercase => priority_reduced + 26
    }

}

pub fn priority_rugsack(first_compartment: &Compartment, second_compartment: &Compartment) -> PriorityScore {

    let match_compartment_set: HashSet<CompartmentItem> = first_compartment.iter()
        .map(|item| *item)
        .collect();

    for second_compartment_item in second_compartment {

        if match_compartment_set.contains(&second_compartment_item) {
            return item_priority_value(&second_compartment_item);
        }
    }

    panic!("🚨 I assume it should not be possible, not sure ...")
}

pub fn priority_group_badge(group_rugsacks: Vec<(Compartment, Compartment)>) -> PriorityScore {

    let first_rugsasck: &(Compartment, Compartment) = group_rugsacks.first().unwrap();
    let first_rugsack_first_compartment: HashSet<&CompartmentItem> = first_rugsasck.0.iter().collect();
    let first_rugsack_second_compartment: HashSet<&CompartmentItem> = first_rugsasck.1.iter().collect();

    let mut rugsack_intersection: HashSet<&CompartmentItem> = first_rugsack_first_compartment.union(&first_rugsack_second_compartment).into_iter().map(|item| *item).collect();

    for group_rugsack in group_rugsacks.iter().skip(1) {
     
        let current_rugsack_first_compartment: HashSet<&CompartmentItem> = group_rugsack.0.iter().collect();
        let current_rugsack_second_compartment: HashSet<&CompartmentItem> = group_rugsack.1.iter().collect();

        let current_rugsack: HashSet<&CompartmentItem> = current_rugsack_first_compartment.union(&current_rugsack_second_compartment).into_iter().map(|item| *item).collect();
        rugsack_intersection = rugsack_intersection.intersection(&current_rugsack).into_iter().map(|item| *item).collect();

    }

    if rugsack_intersection.len() != 1 {
        panic!("🚨 I assume it should not be possible, not sure ...")
    }

    let item_intersected: &CompartmentItem = rugsack_intersection.into_iter().collect::<Vec<&CompartmentItem>>().get(0).unwrap();
    return item_priority_value(item_intersected);

}

// ================================================= IMPLEMENTATIONS =================================================

