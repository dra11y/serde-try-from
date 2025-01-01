use serde_json::Value;
use serde_try_from::{TryFrom, TryFromDe, TryFromSe};

use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Serialize, Deserialize, TryFrom, PartialEq)]
struct TestStruct {
    field1: String,
    field2: i32,
}

#[test]
fn test_try_from_value() {
    let json_value = serde_json::json!({
        "field1": "value1",
        "field2": 42
    });

    let expected = TestStruct {
        field1: "value1".to_string(),
        field2: 42,
    };

    let result = TestStruct::try_from(json_value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_try_from_struct() {
    let test_struct = TestStruct {
        field1: "value1".to_string(),
        field2: 42,
    };

    let expected_json = serde_json::json!({
        "field1": "value1",
        "field2": 42
    });

    let json_value = Value::try_from(test_struct);
    assert!(json_value.is_ok());
    assert_eq!(json_value.unwrap(), expected_json);
}

#[test]
fn test_try_into_value() {
    let json_value = serde_json::json!({
        "field1": "value1",
        "field2": 42
    });

    let expected_struct = TestStruct {
        field1: "value1".to_string(),
        field2: 42,
    };

    let result_json: Result<serde_json::Value, serde_json::Error> = expected_struct.try_into();

    assert!(result_json.is_ok());
    assert_eq!(result_json.unwrap(), json_value);
}

#[test]
fn test_invalid_try_from_value() {
    let invalid_json = serde_json::json!({
        "field1": "value1"
        // Missing field2
    });

    let result = TestStruct::try_from(invalid_json);
    assert!(result.is_err());
}

#[test]
fn test_only_try_from_deserialize() {
    #[derive(Debug, Deserialize, TryFromDe, PartialEq)]
    struct StructWithDeserializeOnly {
        field1: String,
        field2: i32,
    }

    let json_value = serde_json::json!({
        "field1": "value1",
        "field2": 42
    });

    let expected = StructWithDeserializeOnly {
        field1: "value1".to_string(),
        field2: 42,
    };

    let result = StructWithDeserializeOnly::try_from(json_value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_value_try_from_failure_custom_type() {
    use serde::ser::Serialize;

    #[derive(Debug)]
    struct NonSerializable;

    impl Serialize for NonSerializable {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            // Explicitly return an error to simulate a serialization failure
            Err(serde::ser::Error::custom(
                "Serialization is not supported for NonSerializable",
            ))
        }
    }

    #[derive(Debug, Serialize, TryFromSe)]
    struct StructWithNonSerializableField {
        field1: String,
        field2: NonSerializable,
    }

    let invalid_struct = StructWithNonSerializableField {
        field1: "value1".to_string(),
        field2: NonSerializable,
    };

    let result = Value::try_from(invalid_struct);

    // This should fail because field2 cannot be serialized
    assert!(result.is_err());
    assert!(
        format!("{:?}", result.unwrap_err())
            .contains("Serialization is not supported for NonSerializable")
    );
}
