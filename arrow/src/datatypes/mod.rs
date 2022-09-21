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

//! Defines the logical data types of Arrow arrays.
//!
//! The most important things you might be looking for are:
//!  * [`Schema`](crate::datatypes::Schema) to describe a schema.
//!  * [`Field`](crate::datatypes::Field) to describe one field within a schema.
//!  * [`DataType`](crate::datatypes::DataType) to describe the type of a field.

use std::sync::Arc;

mod native;
pub use native::*;
mod numeric;
pub use numeric::*;
mod types;
pub use types::*;
mod decimal;
mod delta;
pub use decimal::*;

pub use arrow_schema::{DataType, Field, IntervalUnit, Schema, TimeUnit, UnionMode};

#[cfg(feature = "ffi")]
mod ffi;
#[cfg(feature = "ffi")]
pub use ffi::*;

/// A reference-counted reference to a [`Schema`](crate::datatypes::Schema).
pub type SchemaRef = Arc<Schema>;