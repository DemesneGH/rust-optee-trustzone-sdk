// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

// Each key-value data, the storage key is "$TABLE_NAME#$KEY"
// For example, if we store a Structure type named ExampleData with id "123",
// the storage key will be "ExampleData#123"

const CONCAT: &str = "#";

pub trait Storable<K>: Serialize + for<'de> Deserialize<'de>
where
    K: TryFrom<String> + Into<String> + Clone,
{
    fn unique_id(&self) -> K;

    fn table_name() -> &'static str {
        // keeps the last part of the path
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .unwrap_or("WRONG_TABLE_NAME")
    }

    fn storage_key(&self) -> String {
        format!(
            "{}{}{}",
            Self::table_name(),
            CONCAT,
            Into::<String>::into(self.unique_id())
        )
    }

    fn concat_key(key: &str) -> String {
        format!("{}{}{}", Self::table_name(), CONCAT, key)
    }
}
