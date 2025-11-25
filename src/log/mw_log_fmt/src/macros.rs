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

#[macro_export]
macro_rules! score_write {
    ($dst:expr, $($arg:tt)*) => {
        write($dst, mw_log_format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! score_writeln {
    ($dst:expr $(,)?) => {
        $crate::score_write!($dst, "\n")
    };
    ($dst:expr, $($arg:tt)*) => {
        write($dst, mw_log_format_args_nl!($($arg)*))
    };
}
