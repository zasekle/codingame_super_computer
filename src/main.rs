use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
struct InitialValues {
    start_day: usize,
    duration: usize,
}

#[derive(Debug, Eq, PartialEq, Ord)]
struct OrderedVals {
    start_day: usize,
    start_time: bool,
    index: usize,
}

impl PartialOrd for OrderedVals {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.start_day == other.start_day {
            return Some(other.start_time.cmp(&self.start_time));
        }
        Some(self.start_day.cmp(&other.start_day))
    }
}

fn main() {

    //Test Case #1
    let values = Vec::from([
        InitialValues { start_day: 2, duration: 5 },
        InitialValues { start_day: 9, duration: 7 },
        InitialValues { start_day: 15, duration: 6 },
        InitialValues { start_day: 9, duration: 3 },
    ]);

    //Test Case #2
    // let values = Vec::from([
    //     InitialValues { start_day: 3, duration: 5 },
    //     InitialValues { start_day: 9, duration: 2 },
    //     InitialValues { start_day: 24, duration: 5 },
    //     InitialValues { start_day: 16, duration: 9 },
    //     InitialValues { start_day: 11, duration: 6 },
    // ]);

    let mut ordered_values = Vec::<OrderedVals>::new();
    for (i, v) in values.iter().enumerate() {
        ordered_values.push(OrderedVals { start_day: v.start_day, start_time: true, index: i });
        ordered_values.push(OrderedVals { start_day: v.start_day + v.duration - 1, start_time: false, index: i });
    }
    ordered_values.sort();

    let mut num_completed_sets = 0;
    let mut outstanding_sets: HashMap<usize, usize> = HashMap::new();
    for val in &ordered_values {
        if val.start_time {
            outstanding_sets.insert(val.index, num_completed_sets + 1);
        } else {
            let completed_set = outstanding_sets.remove(&val.index).expect("failed to remove");
            num_completed_sets = std::cmp::max(num_completed_sets, completed_set);
        }
    }

    println!("{}", num_completed_sets);
}
