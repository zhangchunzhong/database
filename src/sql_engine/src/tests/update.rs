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
use protocol::sql_types::PostgreSqlType;

#[rstest::rstest]
fn update_all_records(sql_engine_with_schema: (QueryExecutor, ResultCollector)) {
    let (mut engine, collector) = sql_engine_with_schema;
    engine
        .execute("create table schema_name.table_name (column_test smallint);")
        .expect("no system errors");
    engine
        .execute("insert into schema_name.table_name values (123);")
        .expect("no system errors");
    engine
        .execute("insert into schema_name.table_name values (456);")
        .expect("no system errors");
    engine
        .execute("select * from schema_name.table_name;")
        .expect("no system errors");
    engine
        .execute("update schema_name.table_name set column_test=789;")
        .expect("no system errors");
    engine
        .execute("select * from schema_name.table_name;")
        .expect("no system errors");

    collector.assert_content_for_single_queries(vec![
        Ok(QueryEvent::SchemaCreated),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::TableCreated),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsInserted(1)),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsInserted(1)),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsSelected((
            vec![("column_test".to_owned(), PostgreSqlType::SmallInt)],
            vec![vec!["123".to_owned()], vec!["456".to_owned()]],
        ))),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsUpdated(2)),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsSelected((
            vec![("column_test".to_owned(), PostgreSqlType::SmallInt)],
            vec![vec!["789".to_owned()], vec!["789".to_owned()]],
        ))),
        Ok(QueryEvent::QueryComplete),
    ]);
}

#[rstest::rstest]
fn update_single_column_of_all_records(sql_engine_with_schema: (QueryExecutor, ResultCollector)) {
    let (mut engine, collector) = sql_engine_with_schema;
    engine
        .execute("create table schema_name.table_name (col1 smallint, col2 smallint);")
        .expect("no system errors");
    engine
        .execute("insert into schema_name.table_name values (123, 789);")
        .expect("no system errors");
    engine
        .execute("insert into schema_name.table_name values (456, 789);")
        .expect("no system errors");
    engine
        .execute("select * from schema_name.table_name;")
        .expect("no system errors");
    engine
        .execute("update schema_name.table_name set col2=357;")
        .expect("no system errors");
    engine
        .execute("select * from schema_name.table_name;")
        .expect("no system errors");

    collector.assert_content_for_single_queries(vec![
        Ok(QueryEvent::SchemaCreated),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::TableCreated),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsInserted(1)),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsInserted(1)),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsSelected((
            vec![
                ("col1".to_owned(), PostgreSqlType::SmallInt),
                ("col2".to_owned(), PostgreSqlType::SmallInt),
            ],
            vec![
                vec!["123".to_owned(), "789".to_owned()],
                vec!["456".to_owned(), "789".to_owned()],
            ],
        ))),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsUpdated(2)),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsSelected((
            vec![
                ("col1".to_owned(), PostgreSqlType::SmallInt),
                ("col2".to_owned(), PostgreSqlType::SmallInt),
            ],
            vec![
                vec!["123".to_owned(), "357".to_owned()],
                vec!["456".to_owned(), "357".to_owned()],
            ],
        ))),
        Ok(QueryEvent::QueryComplete),
    ]);
}

#[rstest::rstest]
fn update_multiple_columns_of_all_records(sql_engine_with_schema: (QueryExecutor, ResultCollector)) {
    let (mut engine, collector) = sql_engine_with_schema;
    engine
        .execute("create table schema_name.table_name (col1 smallint, col2 smallint, col3 smallint);")
        .expect("no system errors");
    engine
        .execute("insert into schema_name.table_name values (111, 222, 333);")
        .expect("no system errors");
    engine
        .execute("insert into schema_name.table_name values (444, 555, 666);")
        .expect("no system errors");
    engine
        .execute("select * from schema_name.table_name;")
        .expect("no system errors");
    engine
        .execute("update schema_name.table_name set col3=777, col1=999;")
        .expect("no system errors");
    engine
        .execute("select * from schema_name.table_name;")
        .expect("no system errors");

    collector.assert_content_for_single_queries(vec![
        Ok(QueryEvent::SchemaCreated),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::TableCreated),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsInserted(1)),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsInserted(1)),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsSelected((
            vec![
                ("col1".to_owned(), PostgreSqlType::SmallInt),
                ("col2".to_owned(), PostgreSqlType::SmallInt),
                ("col3".to_owned(), PostgreSqlType::SmallInt),
            ],
            vec![
                vec!["111".to_owned(), "222".to_owned(), "333".to_owned()],
                vec!["444".to_owned(), "555".to_owned(), "666".to_owned()],
            ],
        ))),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsUpdated(2)),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsSelected((
            vec![
                ("col1".to_owned(), PostgreSqlType::SmallInt),
                ("col2".to_owned(), PostgreSqlType::SmallInt),
                ("col3".to_owned(), PostgreSqlType::SmallInt),
            ],
            vec![
                vec!["999".to_owned(), "222".to_owned(), "777".to_owned()],
                vec!["999".to_owned(), "555".to_owned(), "777".to_owned()],
            ],
        ))),
        Ok(QueryEvent::QueryComplete),
    ]);
}

#[rstest::rstest]
fn update_all_records_in_multiple_columns(sql_engine_with_schema: (QueryExecutor, ResultCollector)) {
    let (mut engine, collector) = sql_engine_with_schema;
    engine
        .execute("create table schema_name.table_name (column_1 smallint, column_2 smallint, column_3 smallint);")
        .expect("no system errors");
    engine
        .execute("insert into schema_name.table_name values (1, 2, 3), (4, 5, 6), (7, 8, 9);")
        .expect("no system errors");
    engine
        .execute("select * from schema_name.table_name;")
        .expect("no system errors");
    engine
        .execute("update schema_name.table_name set column_1=10, column_2=-20, column_3=30;")
        .expect("no system errors");
    engine
        .execute("select * from schema_name.table_name;")
        .expect("no system errors");

    collector.assert_content_for_single_queries(vec![
        Ok(QueryEvent::SchemaCreated),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::TableCreated),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsInserted(3)),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsSelected((
            vec![
                ("column_1".to_owned(), PostgreSqlType::SmallInt),
                ("column_2".to_owned(), PostgreSqlType::SmallInt),
                ("column_3".to_owned(), PostgreSqlType::SmallInt),
            ],
            vec![
                vec!["1".to_owned(), "2".to_owned(), "3".to_owned()],
                vec!["4".to_owned(), "5".to_owned(), "6".to_owned()],
                vec!["7".to_owned(), "8".to_owned(), "9".to_owned()],
            ],
        ))),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsUpdated(3)),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsSelected((
            vec![
                ("column_1".to_owned(), PostgreSqlType::SmallInt),
                ("column_2".to_owned(), PostgreSqlType::SmallInt),
                ("column_3".to_owned(), PostgreSqlType::SmallInt),
            ],
            vec![
                vec!["10".to_owned(), "-20".to_owned(), "30".to_owned()],
                vec!["10".to_owned(), "-20".to_owned(), "30".to_owned()],
                vec!["10".to_owned(), "-20".to_owned(), "30".to_owned()],
            ],
        ))),
        Ok(QueryEvent::QueryComplete),
    ]);
}

#[rstest::rstest]
fn update_records_in_nonexistent_table(sql_engine_with_schema: (QueryExecutor, ResultCollector)) {
    let (mut engine, collector) = sql_engine_with_schema;
    engine
        .execute("update schema_name.table_name set column_test=789;")
        .expect("no system errors");

    collector.assert_content_for_single_queries(vec![
        Ok(QueryEvent::SchemaCreated),
        Ok(QueryEvent::QueryComplete),
        Err(QueryError::table_does_not_exist("schema_name.table_name".to_owned())),
        Ok(QueryEvent::QueryComplete),
    ]);
}

#[rstest::rstest]
fn update_non_existent_columns_of_records(sql_engine_with_schema: (QueryExecutor, ResultCollector)) {
    let (mut engine, collector) = sql_engine_with_schema;
    engine
        .execute("create table schema_name.table_name (column_test smallint);")
        .expect("no system errors");
    engine
        .execute("insert into schema_name.table_name values (123);")
        .expect("no system errors");
    engine
        .execute("select * from schema_name.table_name;")
        .expect("no system errors");
    engine
        .execute("update schema_name.table_name set col1=456, col2=789;")
        .expect("no system errors");

    collector.assert_content_for_single_queries(vec![
        Ok(QueryEvent::SchemaCreated),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::TableCreated),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsInserted(1)),
        Ok(QueryEvent::QueryComplete),
        Ok(QueryEvent::RecordsSelected((
            vec![("column_test".to_owned(), PostgreSqlType::SmallInt)],
            vec![vec!["123".to_owned()]],
        ))),
        Ok(QueryEvent::QueryComplete),
        Err(QueryError::column_does_not_exist(vec![
            "col1".to_owned(),
            "col2".to_owned(),
        ])),
        Ok(QueryEvent::QueryComplete),
    ]);
}

#[cfg(test)]
mod operators {
    use super::*;

    #[cfg(test)]
    mod mathematical {
        use super::*;

        #[cfg(test)]
        mod integers {
            use super::*;

            #[rstest::fixture]
            fn with_table(
                sql_engine_with_schema: (QueryExecutor, ResultCollector),
            ) -> (QueryExecutor, ResultCollector) {
                let (mut engine, collector) = sql_engine_with_schema;
                engine
                    .execute("create table schema_name.table_name(column_si smallint);")
                    .expect("no system errors");

                engine
                    .execute("insert into schema_name.table_name values (2);")
                    .expect("no system errors");

                (engine, collector)
            }

            #[rstest::rstest]
            fn addition(with_table: (QueryExecutor, ResultCollector)) {
                let (mut engine, collector) = with_table;
                engine
                    .execute("update schema_name.table_name set column_si = 1 + 2;")
                    .expect("no system errors");
                engine
                    .execute("select * from schema_name.table_name;")
                    .expect("no system errors");

                collector.assert_content_for_single_queries(vec![
                    Ok(QueryEvent::SchemaCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::TableCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsInserted(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsUpdated(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsSelected((
                        vec![("column_si".to_owned(), PostgreSqlType::SmallInt)],
                        vec![vec!["3".to_owned()]],
                    ))),
                    Ok(QueryEvent::QueryComplete),
                ]);
            }

            #[rstest::rstest]
            fn subtraction(with_table: (QueryExecutor, ResultCollector)) {
                let (mut engine, collector) = with_table;
                engine
                    .execute("update schema_name.table_name set column_si = 1 - 2;")
                    .expect("no system errors");
                engine
                    .execute("select * from schema_name.table_name;")
                    .expect("no system errors");

                collector.assert_content_for_single_queries(vec![
                    Ok(QueryEvent::SchemaCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::TableCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsInserted(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsUpdated(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsSelected((
                        vec![("column_si".to_owned(), PostgreSqlType::SmallInt)],
                        vec![vec!["-1".to_owned()]],
                    ))),
                    Ok(QueryEvent::QueryComplete),
                ]);
            }

            #[rstest::rstest]
            fn multiplication(with_table: (QueryExecutor, ResultCollector)) {
                let (mut engine, collector) = with_table;
                engine
                    .execute("update schema_name.table_name set column_si = 3 * 2;")
                    .expect("no system errors");
                engine
                    .execute("select * from schema_name.table_name;")
                    .expect("no system errors");

                collector.assert_content_for_single_queries(vec![
                    Ok(QueryEvent::SchemaCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::TableCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsInserted(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsUpdated(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsSelected((
                        vec![("column_si".to_owned(), PostgreSqlType::SmallInt)],
                        vec![vec!["6".to_owned()]],
                    ))),
                    Ok(QueryEvent::QueryComplete),
                ]);
            }

            #[rstest::rstest]
            fn division(with_table: (QueryExecutor, ResultCollector)) {
                let (mut engine, collector) = with_table;
                engine
                    .execute("update schema_name.table_name set column_si = 8 / 2;")
                    .expect("no system errors");
                engine
                    .execute("select * from schema_name.table_name;")
                    .expect("no system errors");

                collector.assert_content_for_single_queries(vec![
                    Ok(QueryEvent::SchemaCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::TableCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsInserted(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsUpdated(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsSelected((
                        vec![("column_si".to_owned(), PostgreSqlType::SmallInt)],
                        vec![vec!["4".to_owned()]],
                    ))),
                    Ok(QueryEvent::QueryComplete),
                ]);
            }

            #[rstest::rstest]
            fn modulo(with_table: (QueryExecutor, ResultCollector)) {
                let (mut engine, collector) = with_table;
                engine
                    .execute("update schema_name.table_name set column_si = 8 % 2;")
                    .expect("no system errors");
                engine
                    .execute("select * from schema_name.table_name;")
                    .expect("no system errors");

                collector.assert_content_for_single_queries(vec![
                    Ok(QueryEvent::SchemaCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::TableCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsInserted(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsUpdated(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsSelected((
                        vec![("column_si".to_owned(), PostgreSqlType::SmallInt)],
                        vec![vec!["0".to_owned()]],
                    ))),
                    Ok(QueryEvent::QueryComplete),
                ]);
            }

            #[rstest::rstest]
            #[ignore]
            // TODO ^ is bitwise in SQL standard
            //      # is bitwise in PostgreSQL and it does not supported in sqlparser-rs
            fn exponentiation(with_table: (QueryExecutor, ResultCollector)) {
                let (mut engine, collector) = with_table;
                engine
                    .execute("update schema_name.table_name set column_si = 8 ^ 2;")
                    .expect("no system errors");
                engine
                    .execute("select * from schema_name.table_name;")
                    .expect("no system errors");

                collector.assert_content_for_single_queries(vec![
                    Ok(QueryEvent::SchemaCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::TableCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsInserted(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsUpdated(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsSelected((
                        vec![("column_si".to_owned(), PostgreSqlType::SmallInt)],
                        vec![vec!["64".to_owned()]],
                    ))),
                    Ok(QueryEvent::QueryComplete),
                ]);
            }

            #[rstest::rstest]
            #[ignore]
            // TODO |/<n> is square root in PostgreSQL and it does not supported in sqlparser-rs
            fn square_root(with_table: (QueryExecutor, ResultCollector)) {
                let (mut engine, collector) = with_table;
                engine
                    .execute("update schema_name.table_name set column_si = |/ 16;")
                    .expect("no system errors");
                engine
                    .execute("select * from schema_name.table_name;")
                    .expect("no system errors");

                collector.assert_content_for_single_queries(vec![
                    Ok(QueryEvent::SchemaCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::TableCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsInserted(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsUpdated(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsSelected((
                        vec![("column_si".to_owned(), PostgreSqlType::SmallInt)],
                        vec![vec!["4".to_owned()]],
                    ))),
                    Ok(QueryEvent::QueryComplete),
                ]);
            }

            #[rstest::rstest]
            #[ignore]
            // TODO ||/<n> is cube root in PostgreSQL and it does not supported in sqlparser-rs
            fn cube_root(with_table: (QueryExecutor, ResultCollector)) {
                let (mut engine, collector) = with_table;
                engine
                    .execute("update schema_name.table_name set column_si = ||/ 8;")
                    .expect("no system errors");
                engine
                    .execute("select * from schema_name.table_name;")
                    .expect("no system errors");

                collector.assert_content_for_single_queries(vec![
                    Ok(QueryEvent::SchemaCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::TableCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsInserted(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsUpdated(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsSelected((
                        vec![("column_si".to_owned(), PostgreSqlType::SmallInt)],
                        vec![vec!["2".to_owned()]],
                    ))),
                    Ok(QueryEvent::QueryComplete),
                ]);
            }

            #[rstest::rstest]
            #[ignore]
            // TODO <n>! is factorial in PostgreSQL and it does not supported in sqlparser-rs
            fn factorial(with_table: (QueryExecutor, ResultCollector)) {
                let (mut engine, collector) = with_table;
                engine
                    .execute("update schema_name.table_name set column_si = 5!;")
                    .expect("no system errors");
                engine
                    .execute("select * from schema_name.table_name;")
                    .expect("no system errors");

                collector.assert_content_for_single_queries(vec![
                    Ok(QueryEvent::SchemaCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::TableCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsInserted(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsUpdated(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsSelected((
                        vec![("column_si".to_owned(), PostgreSqlType::SmallInt)],
                        vec![vec!["120".to_owned()]],
                    ))),
                    Ok(QueryEvent::QueryComplete),
                ]);
            }

            #[rstest::rstest]
            #[ignore]
            // TODO !!<n> is prefix factorial in PostgreSQL and it does not supported in sqlparser-rs
            fn prefix_factorial(with_table: (QueryExecutor, ResultCollector)) {
                let (mut engine, collector) = with_table;
                engine
                    .execute("update schema_name.table_name set column_si = !!5;")
                    .expect("no system errors");
                engine
                    .execute("select * from schema_name.table_name;")
                    .expect("no system errors");

                collector.assert_content_for_single_queries(vec![
                    Ok(QueryEvent::SchemaCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::TableCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsInserted(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsUpdated(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsSelected((
                        vec![("column_si".to_owned(), PostgreSqlType::SmallInt)],
                        vec![vec!["120".to_owned()]],
                    ))),
                    Ok(QueryEvent::QueryComplete),
                ]);
            }

            #[rstest::rstest]
            #[ignore]
            // TODO @<n> is absolute value in PostgreSQL and it does not supported in sqlparser-rs
            fn absolute_value(with_table: (QueryExecutor, ResultCollector)) {
                let (mut engine, collector) = with_table;
                engine
                    .execute("update schema_name.table_name set column_si = @-5;")
                    .expect("no system errors");
                engine
                    .execute("select * from schema_name.table_name;")
                    .expect("no system errors");

                collector.assert_content_for_single_queries(vec![
                    Ok(QueryEvent::SchemaCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::TableCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsInserted(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsUpdated(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsSelected((
                        vec![("column_si".to_owned(), PostgreSqlType::SmallInt)],
                        vec![vec!["5".to_owned()]],
                    ))),
                    Ok(QueryEvent::QueryComplete),
                ]);
            }

            #[rstest::rstest]
            fn bitwise_and(with_table: (QueryExecutor, ResultCollector)) {
                let (mut engine, collector) = with_table;
                engine
                    .execute("update schema_name.table_name set column_si = 5 & 1;")
                    .expect("no system errors");
                engine
                    .execute("select * from schema_name.table_name;")
                    .expect("no system errors");

                collector.assert_content_for_single_queries(vec![
                    Ok(QueryEvent::SchemaCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::TableCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsInserted(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsUpdated(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsSelected((
                        vec![("column_si".to_owned(), PostgreSqlType::SmallInt)],
                        vec![vec!["1".to_owned()]],
                    ))),
                    Ok(QueryEvent::QueryComplete),
                ]);
            }

            #[rstest::rstest]
            fn bitwise_or(with_table: (QueryExecutor, ResultCollector)) {
                let (mut engine, collector) = with_table;
                engine
                    .execute("update schema_name.table_name set column_si = 5 | 2;")
                    .expect("no system errors");
                engine
                    .execute("select * from schema_name.table_name;")
                    .expect("no system errors");

                collector.assert_content_for_single_queries(vec![
                    Ok(QueryEvent::SchemaCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::TableCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsInserted(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsUpdated(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsSelected((
                        vec![("column_si".to_owned(), PostgreSqlType::SmallInt)],
                        vec![vec!["7".to_owned()]],
                    ))),
                    Ok(QueryEvent::QueryComplete),
                ]);
            }

            #[rstest::rstest]
            #[ignore]
            // TODO ~ <n> is bitwise NOT in PostgreSQL and it does not supported in sqlparser-rs
            fn bitwise_not(with_table: (QueryExecutor, ResultCollector)) {
                let (mut engine, collector) = with_table;
                engine
                    .execute("update schema_name.table_name set column_si = ~1;")
                    .expect("no system errors");
                engine
                    .execute("select * from schema_name.table_name;")
                    .expect("no system errors");

                collector.assert_content_for_single_queries(vec![
                    Ok(QueryEvent::SchemaCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::TableCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsInserted(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsUpdated(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsSelected((
                        vec![("column_si".to_owned(), PostgreSqlType::SmallInt)],
                        vec![vec!["-2".to_owned()]],
                    ))),
                    Ok(QueryEvent::QueryComplete),
                ]);
            }

            #[rstest::rstest]
            #[ignore]
            // TODO <n> << <m> is bitwise SHIFT LEFT in PostgreSQL and it does not supported in sqlparser-rs
            fn bitwise_shift_left(with_table: (QueryExecutor, ResultCollector)) {
                let (mut engine, collector) = with_table;
                engine
                    .execute("update schema_name.table_name set column_si = 1 << 4;")
                    .expect("no system errors");
                engine
                    .execute("select * from schema_name.table_name;")
                    .expect("no system errors");

                collector.assert_content_for_single_queries(vec![
                    Ok(QueryEvent::SchemaCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::TableCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsInserted(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsUpdated(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsSelected((
                        vec![("column_si".to_owned(), PostgreSqlType::SmallInt)],
                        vec![vec!["16".to_owned()]],
                    ))),
                    Ok(QueryEvent::QueryComplete),
                ]);
            }

            #[rstest::rstest]
            #[ignore]
            // TODO <n> >> <m> is bitwise SHIFT RIGHT in PostgreSQL and it does not supported in sqlparser-rs
            fn bitwise_right_left(with_table: (QueryExecutor, ResultCollector)) {
                let (mut engine, collector) = with_table;
                engine
                    .execute("update schema_name.table_name set column_si = 8 >> 2;")
                    .expect("no system errors");
                engine
                    .execute("select * from schema_name.table_name;")
                    .expect("no system errors");

                collector.assert_content_for_single_queries(vec![
                    Ok(QueryEvent::SchemaCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::TableCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsInserted(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsUpdated(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsSelected((
                        vec![("column_si".to_owned(), PostgreSqlType::SmallInt)],
                        vec![vec!["2".to_owned()]],
                    ))),
                    Ok(QueryEvent::QueryComplete),
                ]);
            }

            #[rstest::rstest]
            fn evaluate_many_operations(with_table: (QueryExecutor, ResultCollector)) {
                let (mut engine, collector) = with_table;
                engine
                    .execute("update schema_name.table_name set column_si = 5 & 13 % 10 + 1 * 20 - 40 / 4;")
                    .expect("no system errors");
                engine
                    .execute("select * from schema_name.table_name;")
                    .expect("no system errors");

                collector.assert_content_for_single_queries(vec![
                    Ok(QueryEvent::SchemaCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::TableCreated),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsInserted(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsUpdated(1)),
                    Ok(QueryEvent::QueryComplete),
                    Ok(QueryEvent::RecordsSelected((
                        vec![("column_si".to_owned(), PostgreSqlType::SmallInt)],
                        vec![vec!["5".to_owned()]],
                    ))),
                    Ok(QueryEvent::QueryComplete),
                ]);
            }
        }
    }

    #[cfg(test)]
    mod string {
        use super::*;

        #[rstest::fixture]
        fn with_table(sql_engine_with_schema: (QueryExecutor, ResultCollector)) -> (QueryExecutor, ResultCollector) {
            let (mut engine, collector) = sql_engine_with_schema;
            engine
                .execute("create table schema_name.table_name(strings char(5));")
                .expect("no system errors");

            engine
                .execute("insert into schema_name.table_name values ('x');")
                .expect("no system errors");

            (engine, collector)
        }

        #[rstest::rstest]
        fn concatenation(with_table: (QueryExecutor, ResultCollector)) {
            let (mut engine, collector) = with_table;
            engine
                .execute("update schema_name.table_name set strings = '123' || '45';")
                .expect("no system errors");
            engine
                .execute("select * from schema_name.table_name;")
                .expect("no system errors");

            collector.assert_content_for_single_queries(vec![
                Ok(QueryEvent::SchemaCreated),
                Ok(QueryEvent::QueryComplete),
                Ok(QueryEvent::TableCreated),
                Ok(QueryEvent::QueryComplete),
                Ok(QueryEvent::RecordsInserted(1)),
                Ok(QueryEvent::QueryComplete),
                Ok(QueryEvent::RecordsUpdated(1)),
                Ok(QueryEvent::QueryComplete),
                Ok(QueryEvent::RecordsSelected((
                    vec![("strings".to_owned(), PostgreSqlType::Char)],
                    vec![vec!["12345".to_owned()]],
                ))),
                Ok(QueryEvent::QueryComplete),
            ]);
        }

        #[rstest::rstest]
        fn concatenation_with_number(with_table: (QueryExecutor, ResultCollector)) {
            let (mut engine, collector) = with_table;
            engine
                .execute("update schema_name.table_name set strings = 1 || '45';")
                .expect("no system errors");
            engine
                .execute("select * from schema_name.table_name;")
                .expect("no system errors");
            engine
                .execute("update schema_name.table_name set strings = '45' || 1;")
                .expect("no system errors");
            engine
                .execute("select * from schema_name.table_name;")
                .expect("no system errors");

            collector.assert_content_for_single_queries(vec![
                Ok(QueryEvent::SchemaCreated),
                Ok(QueryEvent::QueryComplete),
                Ok(QueryEvent::TableCreated),
                Ok(QueryEvent::QueryComplete),
                Ok(QueryEvent::RecordsInserted(1)),
                Ok(QueryEvent::QueryComplete),
                Ok(QueryEvent::RecordsUpdated(1)),
                Ok(QueryEvent::QueryComplete),
                Ok(QueryEvent::RecordsSelected((
                    vec![("strings".to_owned(), PostgreSqlType::Char)],
                    vec![vec!["145".to_owned()]],
                ))),
                Ok(QueryEvent::QueryComplete),
                Ok(QueryEvent::RecordsUpdated(1)),
                Ok(QueryEvent::QueryComplete),
                Ok(QueryEvent::RecordsSelected((
                    vec![("strings".to_owned(), PostgreSqlType::Char)],
                    vec![vec!["451".to_owned()]],
                ))),
                Ok(QueryEvent::QueryComplete),
            ]);
        }

        #[rstest::rstest]
        fn non_string_concatenation_not_supported(with_table: (QueryExecutor, ResultCollector)) {
            let (mut engine, collector) = with_table;
            engine
                .execute("update schema_name.table_name set column_si = 1 || 2;")
                .expect("no system errors");

            collector.assert_content_for_single_queries(vec![
                Ok(QueryEvent::SchemaCreated),
                Ok(QueryEvent::QueryComplete),
                Ok(QueryEvent::TableCreated),
                Ok(QueryEvent::QueryComplete),
                Ok(QueryEvent::RecordsInserted(1)),
                Ok(QueryEvent::QueryComplete),
                Err(QueryError::undefined_function(
                    "||".to_owned(),
                    "NUMBER".to_owned(),
                    "NUMBER".to_owned(),
                )),
                Ok(QueryEvent::QueryComplete),
            ]);
        }
    }
}
