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

mod java;
mod parser;
mod python;

use crate::workspace_dir;
use anyhow::Result;

pub fn run(language: &str) -> Result<()> {
    let workspace_dir = workspace_dir();
    let services_path = workspace_dir.join("core/src/services").canonicalize()?;
    let services = parser::parse(&services_path.to_string_lossy())?;

    match language {
        "java" => java::generate(workspace_dir, services),
        "python" | "py" => python::generate(workspace_dir, services),
        _ => anyhow::bail!("unsupported language: {}", language),
    }
}
