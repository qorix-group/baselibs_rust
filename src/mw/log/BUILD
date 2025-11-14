# *******************************************************************************
# Copyright (c) 2025 Contributors to the Eclipse Foundation
#
# See the NOTICE file(s) distributed with this work for additional
# information regarding copyright ownership.
#
# This program and the accompanying materials are made available under the
# terms of the Apache License Version 2.0 which is available at
# https://www.apache.org/licenses/LICENSE-2.0
#
# SPDX-License-Identifier: Apache-2.0
# *******************************************************************************
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_proc_macro")

rust_library(
    name = "mw_log_fmt",
    srcs = glob(["mw_log_fmt/src/**/*.rs"]),
    # TODO: expose required interface through `mw_log` and make it private again.
    # visibility = ["//visibility:private"],
    visibility = ["//visibility:public"],
)

rust_proc_macro(
    name = "mw_log_macro",
    srcs = glob(["mw_log_macro/src/**/*.rs"]),
    # TODO: expose required interface through `mw_log` and make it private again.
    # visibility = ["//visibility:private"],
    visibility = ["//visibility:public"],
    deps = [
        "//src/mw/log:mw_log_fmt",
        "@crates//:proc-macro2",
        "@crates//:quote",
        "@crates//:syn",
    ],
)

rust_library(
    name = "mw_log",
    srcs = glob(["mw_log/src/**/*.rs"]),
    crate_features = ["std"],
    proc_macro_deps = [
        "//src/mw/log:mw_log_macro",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "//src/mw/log:mw_log_fmt",
    ],
)
