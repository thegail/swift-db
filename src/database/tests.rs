#[allow(unused_imports)]
use super::*;
use crate::schema::*;

struct Cleanup;
impl Drop for Cleanup {
    fn drop(&mut self) {
        let res = std::fs::remove_file("test.sdb");
        if let Err(err) = res {
            eprintln!("{}", err);
        }
    }
}

fn test_fields() -> Vec<Field> {
    vec![
        Field {
            name: "name".to_string(),
            id: 0x1,
            field_type: FieldType::String,
        },
        Field {
            name: "apple_count".to_string(),
            id: 0x2,
            field_type: FieldType::UInt,
        },
        Field {
            name: "money".to_string(),
            id: 0x3,
            field_type: FieldType::Float,
        },
        Field {
            name: "has_pet".to_string(),
            id: 0x4,
            field_type: FieldType::Bool,
        },
        Field {
            name: "birthday".to_string(),
            id: 0x5,
            field_type: FieldType::DateTime,
        },
    ]
}

fn test_schema() -> Schema {
    Schema {
        name: "people".to_string(),
        id: 0x10,
        fields: test_fields(),
    }
}

#[test]
fn create_document() {
    let _c = Cleanup;
    let field_instances = vec![
        FieldInstance {
            id: 0x1,
            value: FieldValue::String("John Doe".to_string()),
        },
        FieldInstance {
            id: 0x2,
            value: FieldValue::UInt(11),
        },
        FieldInstance {
            id: 0x3,
            value: FieldValue::Float(100.25),
        },
        FieldInstance {
            id: 0x4,
            value: FieldValue::Bool(true),
        },
        FieldInstance {
            id: 0x5,
            value: FieldValue::DateTime(chrono::Utc::now()),
        },
    ];
    let document = Document {
        schema: test_schema(),
        fields: field_instances,
    };
    _ = std::fs::File::create("test.sdb");
    let mut database = super::database::Database::new("test.sdb".to_string(), vec![test_schema()])
        .expect("Database construction failed");
    database.create(document).expect("Creation failed");
}

#[test]
fn read_document() {
    let mut database = super::database::Database::new("test.sdb".to_string(), vec![test_schema()])
        .expect("Database construction failed");
    let _document = database
        .find(
            0x10,
            query::Query {
                collection: 0x10,
                fields_of_interest: vec![0x2],
                condition: query::Condition::Equal(
                    query::Expression::Field(0x2),
                    query::Expression::Value(FieldValue::UInt(11)),
                ),
            },
        )
        .expect("Read error");
}

#[test]
fn write_read_bench() {
    _ = std::fs::File::create("test.sdb");
    let mut database = super::database::Database::new("test.sdb".to_string(), vec![test_schema()])
        .expect("Database construction failed");
    let _c = Cleanup;
    let mut docs = vec![];
    for i in 1..1000 {
        let field_instances = vec![
            FieldInstance {
                id: i,
                value: FieldValue::String("John Doe".repeat(i as usize / 10).to_string()),
            },
            FieldInstance {
                id: 0x2,
                value: FieldValue::UInt(i as u32),
            },
            FieldInstance {
                id: 0x3,
                value: FieldValue::Float(100.25 + i as f64),
            },
            FieldInstance {
                id: 0x4,
                value: FieldValue::Bool(i.is_power_of_two()),
            },
            FieldInstance {
                id: 0x5,
                value: FieldValue::DateTime(chrono::Utc::now()),
            },
        ];
        let document = Document {
            schema: test_schema(),
            fields: field_instances,
        };
        docs.push(document);
    }
    let w_start = std::time::Instant::now();
    for doc in docs {
        database.create(doc).expect("Creation failed");
    }
    let w_finish = std::time::Instant::now();
    println!("Write elapsed: {:?}", w_finish - w_start);
    let r_start = std::time::Instant::now();
    for i in 1..1000 {
        _ = database
            .find(
                0x10,
                query::Query {
                    collection: 0x10,
                    fields_of_interest: vec![0x1, 0x2, 0x3, 0x4, 0x5],
                    condition: query::Condition::Equal(
                        query::Expression::Field(0x2),
                        query::Expression::Value(FieldValue::UInt(i)),
                    ),
                },
            )
            .expect("Read error");
    }
    let r_finish = std::time::Instant::now();
    println!("Read elapsed: {:?}", r_finish - r_start);
}