#[allow(unused_imports)]
use super::*;
// use crate::schema::*;

// struct Cleanup;
// impl Drop for Cleanup {
//     fn drop(&mut self) {
//         let res = std::fs::remove_file("test.sdb");
//         if let Err(err) = res {
//             eprintln!("{}", err);
//         }
//     }
// }

// fn test_fields() -> Vec<Field> {
//     vec![
//         Field {
//             name: "name".to_string(),
//             id: 0x1,
//             field_type: FieldType::String,
//         },
//         Field {
//             name: "apple_count".to_string(),
//             id: 0x2,
//             field_type: FieldType::UInt,
//         },
//         Field {
//             name: "money".to_string(),
//             id: 0x3,
//             field_type: FieldType::Float,
//         },
//         Field {
//             name: "has_pet".to_string(),
//             id: 0x4,
//             field_type: FieldType::Bool,
//         },
//         Field {
//             name: "birthday".to_string(),
//             id: 0x5,
//             field_type: FieldType::DateTime,
//         },
//         Field {
//             name: "state".to_string(),
//             id: 0x6,
//             field_type: FieldType::Enum(vec![
//                 EnumCase {
//                     id: 0x1,
//                     name: "happy".to_string(),
//                     associated_value: None,
//                 },
//                 EnumCase {
//                     id: 0x2,
//                     name: "sad".to_string(),
//                     associated_value: None,
//                 },
//                 EnumCase {
//                     id: 0x3,
//                     name: "angry".to_string(),
//                     associated_value: None,
//                 },
//             ]),
//         },
//         Field {
//             name: "coordinates".to_string(),
//             id: 0x7,
//             field_type: FieldType::Object(Box::new(coords_schema())),
//         },
//     ]
// }

// fn coords_schema() -> Schema {
//     Schema {
//         name: "coordinates".to_string(),
//         id: 0x20,
//         fields: vec![
//             Field {
//                 name: "x".to_string(),
//                 id: 0x1,
//                 field_type: FieldType::Int,
//             },
//             Field {
//                 name: "y".to_string(),
//                 id: 0x2,
//                 field_type: FieldType::Int,
//             },
//         ],
//     }
// }

// fn test_schema() -> Schema {
//     Schema {
//         name: "people".to_string(),
//         id: 0x10,
//         fields: test_fields(),
//     }
// }

// fn bench_test_instance(i: i32) -> Vec<FieldInstance> {
//     vec![
//         FieldInstance {
//             id: 0x1,
//             value: FieldValue::String("John Doe".repeat(i as usize / 10)),
//         },
//         FieldInstance {
//             id: 0x2,
//             value: FieldValue::UInt(i as u32),
//         },
//         FieldInstance {
//             id: 0x3,
//             value: FieldValue::Float(100.25 + i as f64),
//         },
//         FieldInstance {
//             id: 0x4,
//             value: FieldValue::Bool((i as u32).is_power_of_two()),
//         },
//         FieldInstance {
//             id: 0x5,
//             value: FieldValue::DateTime(chrono::DateTime::<chrono::Utc>::from(
//                 std::time::SystemTime::now(),
//             )),
//         },
//         FieldInstance {
//             id: 0x6,
//             value: FieldValue::Enum(Box::new(EnumValue {
//                 case_id: 0x2,
//                 associated_value: None,
//             })),
//         },
//         FieldInstance {
//             id: 0x7,
//             value: FieldValue::Object(Box::new(Document {
//                 schema: coords_schema(),
//                 fields: vec![
//                     FieldInstance {
//                         id: 0x1,
//                         value: FieldValue::Int(100 + i),
//                     },
//                     FieldInstance {
//                         id: 0x2,
//                         value: FieldValue::Int(190),
//                     },
//                 ],
//             })),
//         },
//     ]
// }

// #[test]
// fn create_document() {
//     let _c = Cleanup;
//     let field_instances = vec![
//         FieldInstance {
//             id: 0x1,
//             value: FieldValue::String("John Doe".to_string()),
//         },
//         FieldInstance {
//             id: 0x2,
//             value: FieldValue::UInt(11),
//         },
//         FieldInstance {
//             id: 0x3,
//             value: FieldValue::Float(100.25),
//         },
//         FieldInstance {
//             id: 0x4,
//             value: FieldValue::Bool(true),
//         },
//         FieldInstance {
//             id: 0x5,
//             value: FieldValue::DateTime(chrono::DateTime::<chrono::Utc>::from(
//                 std::time::SystemTime::now(),
//             )),
//         },
//         FieldInstance {
//             id: 0x6,
//             value: FieldValue::Enum(Box::new(EnumValue {
//                 case_id: 0x2,
//                 associated_value: None,
//             })),
//         },
//         FieldInstance {
//             id: 0x7,
//             value: FieldValue::Object(Box::new(Document {
//                 schema: coords_schema(),
//                 fields: vec![
//                     FieldInstance {
//                         id: 0x1,
//                         value: FieldValue::Int(100),
//                     },
//                     FieldInstance {
//                         id: 0x2,
//                         value: FieldValue::Int(190),
//                     },
//                 ],
//             })),
//         },
//     ];
//     let document = Document {
//         schema: test_schema(),
//         fields: field_instances,
//     };
//     _ = std::fs::File::create("test.sdb");
//     let (_, rx) = std::sync::mpsc::channel();
//     let mut database =
//         super::backend::Backend::new("test.sdb".to_string(), vec![test_schema()], rx)
//             .expect("Database construction failed");
//     database.create(document).expect("Creation failed");
// }

// #[test]
// #[ignore]
// fn read_document() {
//     let (_, rx) = std::sync::mpsc::channel();
//     let mut database =
//         super::backend::Backend::new("test.sdb".to_string(), vec![test_schema()], rx)
//             .expect("Database construction failed");
//     let _document = database
//         .find_one(query::Query {
//             collection: 0x10,
//             condition: query::Condition::Equal(
//                 query::Expression::Field(0x2),
//                 query::Expression::Value(FieldValue::UInt(11)),
//             ),
//         })
//         .expect("Read error");
// }

// #[test]
// fn write_read_bench() {
//     _ = std::fs::File::create("test.sdb");
//     let (_, rx) = std::sync::mpsc::channel();
//     let mut database =
//         super::backend::Backend::new("test.sdb".to_string(), vec![test_schema()], rx)
//             .expect("Database construction failed");
//     let _c = Cleanup;
//     let mut docs = vec![];
//     for i in 1..=1000 {
//         let field_instances = bench_test_instance(i);
//         let document = Document {
//             schema: test_schema(),
//             fields: field_instances,
//         };
//         docs.push(document);
//     }
//     let w_start = std::time::Instant::now();
//     for doc in docs {
//         database.create(doc).expect("Creation failed");
//     }
//     let w_finish = std::time::Instant::now();
//     println!("Write elapsed: {:?}", w_finish - w_start);
//     let r_start = std::time::Instant::now();
//     for i in (1..=1000).rev() {
//         _ = database
//             .find_one(query::Query {
//                 collection: 0x10,
//                 condition: query::Condition::Equal(
//                     query::Expression::Field(0x2),
//                     query::Expression::Value(FieldValue::UInt(i)),
//                 ),
//             })
//             .expect("Read error");
//     }
//     let r_finish = std::time::Instant::now();
//     println!("Read elapsed: {:?}", r_finish - r_start);
// }

// #[test]
// #[ignore]
// fn read_many_test() {
//     let (_, rx) = std::sync::mpsc::channel();
//     let mut database =
//         super::backend::Backend::new("test.sdb".to_string(), vec![test_schema()], rx)
//             .expect("Database construction failed");
//     _ = database
//         .find_many(query::Query {
//             collection: 0x10,
//             condition: query::Condition::LessThan(
//                 query::Expression::Field(0x2),
//                 query::Expression::Value(crate::schema::FieldValue::UInt(10)),
//             ),
//         })
//         .expect("Read error");
// }
