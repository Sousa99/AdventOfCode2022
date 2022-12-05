
// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================

pub type ResponsabilityBoundary = usize;

pub struct ResponsabilityInterval {
    start: ResponsabilityBoundary,
    end: ResponsabilityBoundary
}

pub type PairResponsabilities = (ResponsabilityInterval, ResponsabilityInterval);

// =============================================== AUXILIARY FUNCTIONS ===============================================

pub fn detect_total_overlap(first_interval: &ResponsabilityInterval, second_interval: &ResponsabilityInterval) -> bool {

    return (first_interval.start >= second_interval.start && first_interval.end <= second_interval.end) ||
        (second_interval.start >= first_interval.start && second_interval.end <= first_interval.end);
}

pub fn detect_partial_overlap(first_interval: &ResponsabilityInterval, second_interval: &ResponsabilityInterval) -> bool {

    return (first_interval.start >= second_interval.start && first_interval.start <= second_interval.end) ||
        (first_interval.end >= second_interval.start && first_interval.end <= second_interval.end) ||
        (second_interval.start >= first_interval.start && second_interval.start <= first_interval.end) ||
        (second_interval.end >= first_interval.start && second_interval.end <= first_interval.end);
}

// ================================================= IMPLEMENTATIONS =================================================

impl ResponsabilityInterval {

    pub fn new(start: ResponsabilityBoundary, end: ResponsabilityBoundary) -> ResponsabilityInterval {
        ResponsabilityInterval { start: start, end: end }
    }
}
