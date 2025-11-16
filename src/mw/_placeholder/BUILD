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
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")
load("@score_baselibs_rust_crates//:defs.bzl", "all_crate_deps")

rust_library(
    name = "_placeholder",
    srcs = glob(["src/**/*.rs"]),
    visibility = ["//visibility:public"],
)

rust_test(
    name = "_placeholder_tests",
    crate = ":_placeholder",
    deps = all_crate_deps(
        normal = True,
    ) + [
        "//src/mw/_placeholder",
    ],
)
