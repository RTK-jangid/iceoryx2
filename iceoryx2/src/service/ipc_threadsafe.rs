// Copyright (c) 2025 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache Software License 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
// which is available at https://opensource.org/licenses/MIT.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Example
//!
//! ```
//! use iceoryx2::prelude::*;
//!
//! # fn main() -> Result<(), Box<dyn core::error::Error>> {
//! let node = NodeBuilder::new().create::<ipc_threadsafe::Service>()?;
//!
//! // use `ipc` as communication variant
//! let service = node.service_builder(&"My/Funk/ServiceName".try_into()?)
//!     .publish_subscribe::<u64>()
//!     .open_or_create()?;
//!
//! let publisher = service.publisher_builder().create()?;
//! let subscriber = service.subscriber_builder().create()?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! See [`Service`](crate::service) for more detailed examples.

use alloc::sync::Arc;
use core::fmt::Debug;

use crate::service::dynamic_config::DynamicConfig;
use iceoryx2_cal::shm_allocator::pool_allocator::PoolAllocator;
use iceoryx2_cal::*;

use super::ServiceState;

/// Defines a zero copy inter-process communication setup based on posix mechanisms.
#[derive(Debug, Clone)]
pub struct Service {
    state: Arc<ServiceState<Self>>,
}

impl crate::service::Service for Service {
    type StaticStorage = static_storage::recommended::Ipc;
    type ConfigSerializer = serialize::recommended::Recommended;
    type DynamicStorage = dynamic_storage::recommended::Ipc<DynamicConfig>;
    type ServiceNameHasher = hash::recommended::Recommended;
    type SharedMemory = shared_memory::recommended::Ipc<PoolAllocator>;
    type ResizableSharedMemory = resizable_shared_memory::recommended::Ipc<PoolAllocator>;
    type Connection = zero_copy_connection::recommended::Ipc;
    type Event = event::recommended::Ipc;
    type Monitoring = monitoring::recommended::Ipc;
    type Reactor = reactor::recommended::Ipc;
    type ArcThreadSafetyPolicy<T: Send + Debug> =
        arc_sync_policy::mutex_protected::MutexProtected<T>;
}

impl crate::service::internal::ServiceInternal<Service> for Service {
    fn __internal_from_state(state: ServiceState<Self>) -> Self {
        Self {
            state: Arc::new(state),
        }
    }

    fn __internal_state(&self) -> &Arc<ServiceState<Self>> {
        &self.state
    }
}
