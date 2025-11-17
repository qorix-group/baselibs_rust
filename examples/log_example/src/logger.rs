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

use mw_log::{Log, Metadata, Record};
use mw_log_fmt::Result as FmtResult;
use mw_log_fmt::{write, FormatSpec, ScoreWrite};
use std::fmt::Write;

struct StringWriter {
    buffer: String,
}

impl StringWriter {
    pub fn new() -> Self {
        Self { buffer: String::new() }
    }

    pub fn get(&self) -> &str {
        self.buffer.as_str()
    }
}

impl ScoreWrite for StringWriter {
    fn write_bool(&mut self, v: &bool, _spec: &FormatSpec) -> FmtResult {
        Ok(write!(self.buffer, "{}", v).unwrap())
    }

    fn write_f32(&mut self, v: &f32, _spec: &FormatSpec) -> FmtResult {
        Ok(write!(self.buffer, "{}", v).unwrap())
    }

    fn write_f64(&mut self, v: &f64, _spec: &FormatSpec) -> FmtResult {
        Ok(write!(self.buffer, "{}", v).unwrap())
    }

    fn write_i8(&mut self, v: &i8, _spec: &FormatSpec) -> FmtResult {
        Ok(write!(self.buffer, "{}", v).unwrap())
    }

    fn write_i16(&mut self, v: &i16, _spec: &FormatSpec) -> FmtResult {
        Ok(write!(self.buffer, "{}", v).unwrap())
    }

    fn write_i32(&mut self, v: &i32, _spec: &FormatSpec) -> FmtResult {
        Ok(write!(self.buffer, "{}", v).unwrap())
    }

    fn write_i64(&mut self, v: &i64, _spec: &FormatSpec) -> FmtResult {
        Ok(write!(self.buffer, "{}", v).unwrap())
    }

    fn write_u8(&mut self, v: &u8, _spec: &FormatSpec) -> FmtResult {
        Ok(write!(self.buffer, "{}", v).unwrap())
    }

    fn write_u16(&mut self, v: &u16, _spec: &FormatSpec) -> FmtResult {
        Ok(write!(self.buffer, "{}", v).unwrap())
    }

    fn write_u32(&mut self, v: &u32, _spec: &FormatSpec) -> FmtResult {
        Ok(write!(self.buffer, "{}", v).unwrap())
    }

    fn write_u64(&mut self, v: &u64, _spec: &FormatSpec) -> FmtResult {
        Ok(write!(self.buffer, "{}", v).unwrap())
    }

    fn write_str(&mut self, v: &str, _spec: &FormatSpec) -> FmtResult {
        Ok(write!(self.buffer, "{}", v).unwrap())
    }
}

pub struct ExampleLogger;

impl Log for ExampleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        // Create writer and write log data.
        let mut writer = StringWriter::new();
        write(&mut writer, *record.args()).unwrap();

        // Show to stderr.
        eprintln!("{}", writer.get());
    }

    fn flush(&self) {
        // No-op.
    }
}
