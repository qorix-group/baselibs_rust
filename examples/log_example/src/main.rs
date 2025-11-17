//
// Copyright (c) 2025 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache License Version 2.0 which is available at
// <https://www.apache.org/licenses/LICENSE-2.0>
//
// SPDX-License-Identifier: Apache-2.0
//

mod logger;

use crate::logger::ExampleLogger;
use mw_log::{debug, error, info, trace, warn, LevelFilter};
use mw_log_fmt::{Arguments, Fragment};

fn main() {
    // Initialize logger.
    mw_log::set_max_level(LevelFilter::Info);
    mw_log::set_logger(&ExampleLogger).expect("Unable to set logger");

    // Example logs.
    trace!("trace log - hidden!");
    debug!("debug log - hidden!");
    info!("info log");
    warn!("warn log");
    error!("error log");
    // TODO: fatal log.
}
