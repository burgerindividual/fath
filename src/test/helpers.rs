// use std::string::{String, ToString};

// #[derive(Clone)]
// struct MacroType<'a> {
//     combined_type_string: &'a str,
//     used_lane_counts: &'a [usize],
//     unused_lane_counts: &'a [usize],
//     combined_lane_counts: Vec<usize>,
// }

// impl<'a> MacroType<'a> {
//     pub fn new(combined_type_string: &'a str, used_lane_counts: &'a [usize], unused_lane_counts: &'a [usize]) -> Self {
//         let combined_lane_counts: Vec<usize> = [used_lane_counts, unused_lane_counts].concat();

//         MacroType {
//             combined_type_string,
//             used_lane_counts,
//             unused_lane_counts,
//             combined_lane_counts,
//         }
//     }
// }

// pub fn main() {
//     let macro_types: [MacroType; 4] = [
//         MacroType::new("u8, i8, ", &[2, 4, 8, 16], &[32, 64]),
//         MacroType::new("u16, i16, ", &[2, 4, 8], &[16, 32, 64]),
//         MacroType::new("u32, i32, f32, ", &[2, 4], &[8, 16, 32, 64]),
//         MacroType::new("u64, i64, f64, ", &[2], &[4, 8, 16, 32, 64]),
//     ];
//     let mut builder = String::new();
    
//     builder.push_str("empty_impl!(\n");
//     for t in macro_types.clone() {
//         builder.push_str(t.combined_type_string);

//         let mut last = t.combined_lane_counts.len();
//         for &lane_count_1 in t.unused_lane_counts.iter().rev() {
//             for &lane_count_2 in t.combined_lane_counts[..last].iter().rev() {
//                 // builder.push_str("[");
//                 builder.push_str(&*lane_count_1.to_string());
//                 builder.push_str(", ");
//                 builder.push_str(&*lane_count_2.to_string());
//                 // builder.push_str("]");
//                 builder.push_str(", ");
//                 if lane_count_1 != lane_count_2 {
//                     // builder.push_str("[");
//                     builder.push_str(&*lane_count_2.to_string());
//                     builder.push_str(", ");
//                     builder.push_str(&*lane_count_1.to_string());
//                     // builder.push_str("]");
//                     builder.push_str(", ");
//                 }
//             }
            
//             builder.push_str("\n");
//             last -= 1;
//         }
//     }
    
//     builder.push_str(");\n\nintrinsic_impl!([insert fn here], [insert bytes here],\n");
    
//     for t in macro_types.clone() {
//         builder.push_str(t.combined_type_string);

//         let mut last = t.used_lane_counts.len();
//         for &lane_count_1 in t.used_lane_counts.iter().rev() {
//             for &lane_count_2 in t.used_lane_counts[..last].iter().rev() {
//                 // builder.push_str("[");
//                 builder.push_str(&*lane_count_1.to_string());
//                 builder.push_str(", ");
//                 builder.push_str(&*lane_count_2.to_string());
//                 // builder.push_str("]");
//                 builder.push_str(", ");
//                 if lane_count_1 != lane_count_2 {
//                     // builder.push_str("[");
//                     builder.push_str(&*lane_count_2.to_string());
//                     builder.push_str(", ");
//                     builder.push_str(&*lane_count_1.to_string());
//                     // builder.push_str("]");
//                     builder.push_str(", ");
//                 }
//             }
            
//             builder.push_str("\n");
//             last -= 1;
//         }
//     }
//     builder.push_str(");");

//     println!("{builder}");
// }