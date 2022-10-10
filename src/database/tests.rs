#[allow(unused_imports)]
use super::*;
use crate::schema::*;

struct Cleanup;
impl Drop for Cleanup {
    fn drop(&mut self) {
        // let res = std::fs::remove_file("test.sdb");
        // if let Err(err) = res {
        //     eprintln!("{}", err);
        // }
    }
}

#[test]
fn create_document() {
    let _c = Cleanup;
    let test_fields = vec![
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
    ];
    let test_schema = Schema {
        name: "people".to_string(),
        id: 0x10,
        fields: test_fields,
    };
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
        schema: test_schema.clone(),
        fields: field_instances,
    };
    _ = std::fs::File::create("test.sdb");
    let mut database = super::database::Database::new("test.sdb".to_string(), vec![test_schema])
        .expect("Database construction failed");
    database.create(document).expect("Creation failed");
}
