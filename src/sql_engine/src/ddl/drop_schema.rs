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

use crate::query::SchemaId;
use data_manager::{DataManager, DropSchemaError, DropStrategy};
use kernel::SystemResult;
use protocol::{
    results::{QueryError, QueryEvent},
    Sender,
};
use std::sync::Arc;

pub(crate) struct DropSchemaCommand {
    name: SchemaId,
    cascade: bool,
    storage: Arc<DataManager>,
    sender: Arc<dyn Sender>,
}

impl DropSchemaCommand {
    pub(crate) fn new(
        name: SchemaId,
        cascade: bool,
        storage: Arc<DataManager>,
        sender: Arc<dyn Sender>,
    ) -> DropSchemaCommand {
        DropSchemaCommand {
            name,
            cascade,
            storage,
            sender,
        }
    }

    pub(crate) fn execute(&mut self) -> SystemResult<()> {
        let schema_name = self.name.name().to_string();
        let strategy = if self.cascade {
            DropStrategy::Cascade
        } else {
            DropStrategy::Restrict
        };
        match self.storage.schema_exists(&schema_name) {
            None => {
                self.sender
                    .send(Err(QueryError::schema_does_not_exist(schema_name)))
                    .expect("To Send Query Result to Client");
                Ok(())
            }
            Some(schema_id) => {
                match self.storage.drop_schema(schema_id, strategy) {
                    Err(error) => Err(error),
                    Ok(Err(DropSchemaError::CatalogDoesNotExist)) => {
                        //ignore. Catalogs are not implemented
                        Ok(())
                    }
                    Ok(Err(DropSchemaError::HasDependentObjects)) => {
                        self.sender
                            .send(Err(QueryError::schema_has_dependent_objects(schema_name)))
                            .expect("To Send Query Result to Client");
                        Ok(())
                    }
                    Ok(Err(DropSchemaError::DoesNotExist)) => {
                        self.sender
                            .send(Err(QueryError::schema_does_not_exist(schema_name)))
                            .expect("To Send Query Result to Client");
                        Ok(())
                    }
                    Ok(Ok(())) => {
                        self.sender
                            .send(Ok(QueryEvent::SchemaDropped))
                            .expect("To Send Query Result to Client");
                        Ok(())
                    }
                }
            }
        }
    }
}
