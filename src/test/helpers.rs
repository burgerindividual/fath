use alloc::string::{String, ToString};

struct MacroType<'a> {
    combined_type_string: &'a str,
    used_lane_counts: &'a [usize],
    unused_lane_counts: &'a [usize],
    combined_lane_counts: &'a [usize],
}

impl<'a> MacroType<'a> {
    pub fn new(combined_type_string: &'a str, used_lane_counts: &'a [usize], unused_lane_counts: &'a [usize]) -> Self {
        let combined_lane_counts: &'a [usize] = (&*[used_lane_counts, unused_lane_counts].concat()).clone();

        MacroType {
            combined_type_string,
            used_lane_counts,
            unused_lane_counts,
            combined_lane_counts,
        }
    }
}

#[test]
pub fn generate_dyn_swizzle_macros() {
    let macro_types: [MacroType; 4] = [
        MacroType::new("u8, i8, ", &[2, 4, 8, 16], &[32, 64]),
        MacroType::new("u16, i16, ", &[2, 4, 8], &[16, 32, 64]),
        MacroType::new("u32, i32, f32, ", &[2, 4], &[8, 16, 32, 64]),
        MacroType::new("u64, i64, f64, ", &[2], &[4, 8, 16, 32, 64]),
    ];

    let mut builder = String::new();

    for t in macro_types {
        builder.push_str(t.combined_type_string);

        let mut last = t.combined_lane_counts.len();
        for &unused_lane_count in t.unused_lane_counts.iter().rev() {
            for &lane_count in t.combined_lane_counts[..last].iter().rev() {
                builder.push_str(&*unused_lane_count.to_string());
                builder.push_str(", ");
                builder.push_str(&*lane_count.to_string());
                builder.push_str(", ");
            }

            last -= 1;
        }
    }

    println!("{builder}");
}