
// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================

pub type SnackCalories = i32;

struct ElfSnack {
    calories: SnackCalories
}

pub struct Elf {
    snacks: Vec<ElfSnack>
}

// =============================================== AUXILIARY FUNCTIONS ===============================================

pub fn convert_input_to_correct_format(input_lines: Vec<String>) -> Vec<Vec<SnackCalories>> {

    let mut input_formatted: Vec<Vec<SnackCalories>> = Vec::new();

    let mut current_list: Vec<SnackCalories> = Vec::new();
    for line in input_lines.into_iter() {

        if line.is_empty() {

            input_formatted.push(current_list);
            current_list = Vec::new();

        } else {

            let new_snack_calories: SnackCalories = line.parse().unwrap();
            current_list.push(new_snack_calories);

        }
    }

    input_formatted.push(current_list);
    return input_formatted;
}

// ================================================= IMPLEMENTATIONS =================================================

impl ElfSnack {
    
    pub fn new(calories: SnackCalories) -> ElfSnack {
        ElfSnack {
            calories: calories
        }
    }
}


impl Elf {
    
    pub fn new(snacks_calories: Vec<SnackCalories>) -> Elf {
        Elf {
            snacks: snacks_calories.into_iter()
            .map(|calories| ElfSnack::new(calories))
            .collect()
        }
    }

    pub fn get_total_snack_calories(&self) -> SnackCalories {

        return self.snacks.iter()
            .map(|snack| snack.calories)
            .sum();
    }
}