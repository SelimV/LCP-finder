use std::collections::{HashMap, HashSet};

type Label = String;

type Relation = HashMap<Label, HashSet<Label>>;

pub type LCPMap = HashMap<Vec<Label>, Vec<Label>>;

enum CheckResult {
    Pass(Option<Relation>),
    Fail,
}

/// Checks if the addition to the relation breaks compatibility with the passive
/// configurations. Returns a relation with the addition if it does not. Assumes
/// that the input relation is compatible and the configurations are sorted.
fn check(
    relation: &Relation,
    addition_source: &Label,
    addition_target: &Label,
    passive_source: &HashSet<Vec<Label>>,
    passive_target: &HashSet<Vec<Label>>,
    passive_degree: &usize,
) -> CheckResult {
    let targets_of_addition_source = relation.get(addition_source);
    if let Some(targets_of_addition_source) = targets_of_addition_source {
        if targets_of_addition_source.contains(addition_target) {
            // No new addition, nothing to check
            return CheckResult::Pass(None);
        }
    }

    // Include the addition in the relation
    let mut targets_with_addition = targets_of_addition_source.cloned().unwrap_or_default();
    targets_with_addition.insert(addition_target.clone());
    let mut relation_with_addition = relation.clone();
    relation_with_addition.insert(addition_source.clone(), targets_with_addition);

    // Check compatibility using lemma 39
    let passive_source_with_addition = passive_source
        .iter()
        .filter_map(|configuration| {
            let index_of_addition_source = configuration.binary_search(addition_source).ok();
            index_of_addition_source.map(|index| {
                let mut configuration = configuration.clone();
                // No need to preserve order for source configurations
                configuration.swap_remove(index);
                configuration
            })
        })
        .collect::<HashSet<_>>();

    let passive_target_with_addition = passive_target
        .iter()
        .filter_map(|configuration| {
            let index_of_addition_target = configuration.binary_search(addition_target).ok();
            index_of_addition_target.map(|index| {
                let mut configuration = configuration.clone();
                // Preserve order for source configurations, hence no swap remove
                configuration.remove(index);
                configuration
            })
        })
        .collect::<HashSet<_>>();

    // Map each source label to the set of possible target labels and expand to
    // different configurations
    let possible_passive_configurations_after_mapping = (0..(passive_degree - 1))
        .fold(passive_source_with_addition, |acc, index| {
            acc.iter()
                .flat_map(|row| {
                    relation_with_addition
                        .get(&row[index])
                        .cloned()
                        .unwrap_or_default()
                        .iter()
                        .map(|target_label| {
                            let mut row = row.clone();
                            row[index] = target_label.clone();
                            row
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<HashSet<_>>()
        })
        // Sort the configurations so we can compare with passive_target
        .iter()
        .map(|configuration| {
            let mut configuration = configuration.clone();
            configuration.sort();
            configuration
        })
        .collect::<HashSet<_>>();

    if possible_passive_configurations_after_mapping.is_subset(&passive_target_with_addition) {
        // All the configurations are valid after mapping
        CheckResult::Pass(Some(relation_with_addition))
    } else {
        CheckResult::Fail
    }
}

fn try_permutations(
    relation: &Relation,
    passive_source: &HashSet<Vec<Label>>,
    passive_target: &HashSet<Vec<Label>>,
    active_target: &[Vec<Label>],
    active_source_remaining: &[Vec<Label>],
    remaining_labels_in_selected_target_configuration: &Vec<Label>,
    passive_degree: &usize,
) -> Option<(LCPMap, Vec<Label>)> {
    if remaining_labels_in_selected_target_configuration.is_empty() {
        let mut active_source_remaining = Vec::from(active_source_remaining);
        active_source_remaining.pop();
        return try_configurations(
            relation,
            passive_source,
            passive_target,
            active_target,
            &active_source_remaining,
            passive_degree,
        )
        .map(|map| (map, Vec::new()));
    }

    let addition_source = &active_source_remaining.last().unwrap()
        [remaining_labels_in_selected_target_configuration.len() - 1];

    for index in 0..remaining_labels_in_selected_target_configuration.len() {
        let addition_target = &remaining_labels_in_selected_target_configuration[index];

        if let CheckResult::Pass(relation_with_addition) = check(
            relation,
            addition_source,
            addition_target,
            passive_source,
            passive_target,
            passive_degree,
        ) {
            let mut remaining_labels_in_selected_target_configuration =
                remaining_labels_in_selected_target_configuration.clone();
            remaining_labels_in_selected_target_configuration.remove(index);
            let found_map_extension = if let Some(relation_with_addition) = relation_with_addition {
                try_permutations(
                    &relation_with_addition,
                    passive_source,
                    passive_target,
                    active_target,
                    active_source_remaining,
                    &remaining_labels_in_selected_target_configuration,
                    passive_degree,
                )
            } else {
                try_permutations(
                    relation,
                    passive_source,
                    passive_target,
                    active_target,
                    active_source_remaining,
                    &remaining_labels_in_selected_target_configuration,
                    passive_degree,
                )
            };

            if let Some((map, mut partial_permutation)) = found_map_extension {
                partial_permutation.push(addition_target.clone());
                return Some((map, partial_permutation));
            }
        }
    }

    None
}

fn try_configurations(
    relation: &Relation,
    passive_source: &HashSet<Vec<Label>>,
    passive_target: &HashSet<Vec<Label>>,
    active_target: &[Vec<Label>],
    active_source_remaining: &[Vec<Label>],
    passive_degree: &usize,
) -> Option<LCPMap> {
    if active_source_remaining.is_empty() {
        println!("{:?}",relation);
        return Some(LCPMap::new());
    }
    for selected_target_configuration in active_target {
        if let Some((mut map, permutation)) = try_permutations(
            relation,
            passive_source,
            passive_target,
            active_target,
            active_source_remaining,
            selected_target_configuration,
            passive_degree,
        ) {
            map.insert(active_source_remaining.last().unwrap().clone(), permutation);
            return Some(map);
        }
    }
    None
}

pub fn find_lcp(
    active_source: &[Vec<Label>],
    passive_source: &[Vec<Label>],
    active_target: &[Vec<Label>],
    passive_target: &[Vec<Label>],
) -> Option<LCPMap> {
    let passive_degree = passive_source
        .first()
        .expect("No passive source configurations.")
        .len();
    try_configurations(
        &Relation::new(),
        &passive_source.iter().cloned().collect(),
        &passive_target.iter().cloned().collect(),
        active_target,
        active_source,
        &passive_degree,
    )
}
