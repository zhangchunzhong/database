// Copyright 2020 Alex Dukhno
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;
use sql_types::SqlType;

#[rstest::rstest]
fn update_all_records(mut storage: PersistentStorage) {
    create_schema_with_table(
        &mut storage,
        "schema_name",
        "table_name",
        vec![("column_test", SqlType::SmallInt(i16::min_value()))],
    );

    insert_into(&mut storage, "schema_name", "table_name", vec![], vec!["123"]);
    insert_into(&mut storage, "schema_name", "table_name", vec![], vec!["456"]);
    insert_into(&mut storage, "schema_name", "table_name", vec![], vec!["789"]);

    assert_eq!(
        storage
            .update_all(
                "schema_name",
                "table_name",
                vec![("column_test".to_owned(), "567".to_owned())]
            )
            .expect("no system errors"),
        Ok(3)
    );

    let table_columns = storage
        .table_columns("schema_name", "table_name")
        .expect("no system errors")
        .into_iter()
        .map(|column_definition| column_definition.name())
        .collect();

    assert_eq!(
        storage
            .select_all_from("schema_name", "table_name", table_columns)
            .expect("no system errors"),
        Ok((
            vec![column_definition("column_test", SqlType::SmallInt(i16::min_value()))],
            vec![vec!["567".to_owned()], vec!["567".to_owned()], vec!["567".to_owned()]]
        ))
    );
}

#[rstest::rstest]
fn update_not_existed_table(mut storage: PersistentStorage) {
    create_schema(&mut storage, "schema_name");

    assert_eq!(
        storage
            .update_all("schema_name", "not_existed", vec![])
            .expect("no system errors"),
        Err(OperationOnTableError::TableDoesNotExist)
    );
}

#[rstest::rstest]
fn update_non_existent_schema(mut storage: PersistentStorage) {
    assert_eq!(
        storage
            .update_all("non_existent", "not_existed", vec![])
            .expect("no system errors"),
        Err(OperationOnTableError::SchemaDoesNotExist)
    );
}

#[cfg(test)]
mod constraints {
    use super::*;
    use sql_types::ConstraintError;

    #[rstest::fixture]
    fn storage_with_ints_table(mut storage: PersistentStorage) -> PersistentStorage {
        create_schema_with_table(
            &mut storage,
            "schema_name",
            "table_name",
            vec![
                ("column_si", SqlType::SmallInt(i16::min_value())),
                ("column_i", SqlType::Integer(i32::min_value())),
                ("column_bi", SqlType::BigInt(i64::min_value())),
            ],
        );
        storage
    }

    #[rstest::fixture]
    fn storage_with_chars_table(mut storage: PersistentStorage) -> PersistentStorage {
        create_schema_with_table(
            &mut storage,
            "schema_name",
            "table_name",
            vec![("column_c", SqlType::Char(10)), ("column_vc", SqlType::VarChar(10))],
        );
        storage
    }

    #[rstest::rstest]
    fn out_of_range_violation(mut storage_with_ints_table: PersistentStorage) {
        storage_with_ints_table
            .insert_into(
                "schema_name",
                "table_name",
                vec![],
                vec![vec!["100".to_owned(), "100".to_owned(), "100".to_owned()]],
            )
            .expect("no system errors")
            .expect("record inserted");
        assert_eq!(
            storage_with_ints_table
                .update_all(
                    "schema_name",
                    "table_name",
                    vec![
                        ("column_si".to_owned(), "-32769".to_owned()),
                        ("column_i".to_owned(), "100".to_owned()),
                        ("column_bi".to_owned(), "100".to_owned())
                    ]
                )
                .expect("no system errors"),
            Err(OperationOnTableError::ConstraintViolations(vec![(
                ConstraintError::OutOfRange,
                column_definition("column_si", SqlType::SmallInt(i16::min_value()))
            )]))
        );
    }

    #[rstest::rstest]
    fn not_an_int_violation(mut storage_with_ints_table: PersistentStorage) {
        storage_with_ints_table
            .insert_into(
                "schema_name",
                "table_name",
                vec![],
                vec![vec!["100".to_owned(), "100".to_owned(), "100".to_owned()]],
            )
            .expect("no system errors")
            .expect("record inserted");
        assert_eq!(
            storage_with_ints_table
                .update_all(
                    "schema_name",
                    "table_name",
                    vec![
                        ("column_si".to_owned(), "abc".to_owned()),
                        ("column_i".to_owned(), "100".to_owned()),
                        ("column_bi".to_owned(), "100".to_owned())
                    ]
                )
                .expect("no system errors"),
            Err(OperationOnTableError::ConstraintViolations(vec![(
                ConstraintError::TypeMismatch("abc".to_owned()),
                column_definition("column_si", SqlType::SmallInt(i16::min_value()))
            )]))
        );
    }

    #[rstest::rstest]
    fn value_too_long_violation(mut storage_with_chars_table: PersistentStorage) {
        storage_with_chars_table
            .insert_into(
                "schema_name",
                "table_name",
                vec![],
                vec![vec!["100".to_owned(), "100".to_owned()]],
            )
            .expect("no system errors")
            .expect("record inserted");
        assert_eq!(
            storage_with_chars_table
                .update_all(
                    "schema_name",
                    "table_name",
                    vec![
                        ("column_c".to_owned(), "12345678901".to_owned()),
                        ("column_vc".to_owned(), "100".to_owned())
                    ]
                )
                .expect("no system errors"),
            Err(OperationOnTableError::ConstraintViolations(vec![(
                ConstraintError::ValueTooLong(10),
                column_definition("column_c", SqlType::Char(10))
            )]))
        );
    }

    #[rstest::rstest]
    fn multiple_columns_violation(mut storage_with_ints_table: PersistentStorage) {
        storage_with_ints_table
            .insert_into(
                "schema_name",
                "table_name",
                vec![],
                vec![vec!["100".to_owned(), "100".to_owned(), "100".to_owned()]],
            )
            .expect("no system errors")
            .expect("records inserted");

        assert_eq!(
            storage_with_ints_table
                .update_all(
                    "schema_name",
                    "table_name",
                    vec![
                        ("column_si".to_owned(), "-32769".to_owned()),
                        ("column_i".to_owned(), "-2147483649".to_owned()),
                        ("column_bi".to_owned(), "100".to_owned())
                    ]
                )
                .expect("no system errors"),
            Err(OperationOnTableError::ConstraintViolations(vec![
                (
                    ConstraintError::OutOfRange,
                    column_definition("column_si", SqlType::SmallInt(i16::min_value()))
                ),
                (
                    ConstraintError::OutOfRange,
                    column_definition("column_i", SqlType::Integer(i32::min_value()))
                )
            ]))
        )
    }
}
