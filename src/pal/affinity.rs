// *******************************************************************************
// Copyright (c) 2026 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache License Version 2.0 which is available at
// <https://www.apache.org/licenses/LICENSE-2.0>
//
// SPDX-License-Identifier: Apache-2.0
// *******************************************************************************

//! Affinity handling differs between Linux and QNX.
//! Module ensures similar behavior between both OSes.

const MAX_CPU_NUM: usize = 1024;
const MAX_CPU_ID: usize = MAX_CPU_NUM - 1;

/// Common CPU set representation.
/// Limited to 1024 CPUs.
#[derive(Clone, Copy)]
pub struct CpuSet {
    mask: [u8; 128],
}

impl CpuSet {
    /// Create a new CPU set.
    pub fn new(affinity: &[usize]) -> Self {
        let mask = Self::create_mask(affinity);
        Self { mask }
    }

    /// Create mask based on provided list.
    fn create_mask(affinity: &[usize]) -> [u8; 128] {
        let mut mask = [0u8; _];
        for cpu_id in affinity.iter().copied() {
            if cpu_id > MAX_CPU_ID {
                panic!("CPU ID provided to affinity exceeds max supported size, provided: {cpu_id}, max: {MAX_CPU_ID}");
            }

            let index = cpu_id / 8;
            let offset = cpu_id % 8;
            mask[index] |= 1 << offset;
        }

        mask
    }

    /// Create list based on provided mask.
    fn create_list(mask: &[u8; 128]) -> Vec<usize> {
        let mut list = Vec::new();
        for cpu_id in 0..MAX_CPU_NUM {
            let index = cpu_id / 8;
            let offset = cpu_id % 8;

            if (mask[index] & (1 << offset)) != 0 {
                list.push(cpu_id);
            }
        }

        list
    }

    pub fn set(&mut self, affinity: &[usize]) {
        self.mask = Self::create_mask(affinity);
    }

    pub fn get(&self) -> Vec<usize> {
        Self::create_list(&self.mask)
    }
}

#[cfg(target_os = "linux")]
impl From<crate::cpu_set_t> for CpuSet {
    fn from(value: crate::cpu_set_t) -> Self {
        let mask: [u8; 128] = unsafe { core::mem::transmute(value) };
        Self { mask }
    }
}

#[cfg(target_os = "linux")]
impl From<CpuSet> for crate::cpu_set_t {
    fn from(value: CpuSet) -> Self {
        unsafe { core::mem::transmute(value.mask) }
    }
}

/// QNX representation of a CPU set.
///
/// Number of CPUs is restricted to 1024 - same as for Linux.
/// QNX docs recommend the following:
/// - read the number of CPUs from `_syspage_ptr->num_cpu`
/// - allocate mask fields dynamically
///
/// Current approach avoids dynamic alloc and aligns the behavior with Linux.
#[cfg(target_os = "nto")]
#[repr(C)]
#[derive(Clone, Copy)]
struct NtoCpuSet {
    // Expected to always be set to `32` - see comment above.
    pub size: i32,
    pub run_mask: [u32; 32],
    // Expected to always be zeroed - left unaltered.
    pub inherit_mask: [u32; 32],
}

#[cfg(target_os = "nto")]
impl NtoCpuSet {
    fn new(mask: [u32; 32]) -> Self {
        Self {
            size: 32,
            run_mask: mask,
            inherit_mask: [0; 32],
        }
    }
}

#[cfg(target_os = "nto")]
impl From<NtoCpuSet> for CpuSet {
    fn from(value: NtoCpuSet) -> Self {
        let mask = unsafe { core::mem::transmute(value.run_mask) };
        Self { mask }
    }
}

#[cfg(target_os = "nto")]
impl From<CpuSet> for NtoCpuSet {
    fn from(value: CpuSet) -> Self {
        let run_mask = unsafe { core::mem::transmute(value.mask) };
        Self::new(run_mask)
    }
}

/// Set affinity of a current thread.
pub fn set_affinity(cpu_set: CpuSet) {
    #[cfg(target_os = "linux")]
    {
        let native_cpu_set = crate::cpu_set_t::from(cpu_set);
        let cpu_set_size = core::mem::size_of::<crate::cpu_set_t>();
        let rc = unsafe { crate::sched_setaffinity(0, cpu_set_size, &native_cpu_set as *const _) };
        if rc != 0 {
            panic!("sched_setaffinity failed, rc: {rc}");
        }
    }

    #[cfg(target_os = "nto")]
    {
        let mut native_cpu_set = NtoCpuSet::from(cpu_set);
        let rc = unsafe {
            crate::ThreadCtl(
                crate::_NTO_TCTL_RUNMASK_GET_AND_SET_INHERIT as crate::c_int,
                (&mut native_cpu_set as *mut NtoCpuSet).cast(),
            )
        };
        if rc != 0 {
            panic!("ThreadCtl failed, rc: {rc}");
        }
    }
}

/// Get affinity of a current thread.
pub fn get_affinity() -> Vec<usize> {
    #[cfg(target_os = "linux")]
    {
        let mut native_cpu_set = unsafe { core::mem::zeroed::<crate::cpu_set_t>() };
        let cpu_set_size = core::mem::size_of::<crate::cpu_set_t>();
        let rc = unsafe { crate::sched_getaffinity(0, cpu_set_size, &mut native_cpu_set as *mut _) };
        if rc != 0 {
            panic!("sched_getaffinity failed, rc: {rc}");
        }

        let cpu_set = CpuSet::from(native_cpu_set);
        cpu_set.get()
    }

    #[cfg(target_os = "nto")]
    {
        let mut native_cpu_set = NtoCpuSet::new([0; _]);
        let rc = unsafe {
            crate::ThreadCtl(
                crate::_NTO_TCTL_RUNMASK_GET_AND_SET_INHERIT as crate::c_int,
                (&mut native_cpu_set as *mut NtoCpuSet).cast(),
            )
        };
        if rc != 0 {
            panic!("ThreadCtl failed, rc: {rc}");
        }

        let cpu_set = CpuSet::from(native_cpu_set);
        cpu_set.get()
    }
}

#[cfg(test)]
mod tests {
    use crate::affinity::CpuSet;

    #[test]
    fn cpu_set_new_empty_succeeds() {
        let cpu_set = CpuSet::new(&[]);
        assert!(cpu_set.mask.iter().all(|x| *x == 0));
    }

    #[test]
    fn cpu_set_new_some_succeeds() {
        let cpu_set = CpuSet::new(&[0, 123, 1023]);
        let mut data_vec = cpu_set.mask.to_vec();

        // Test removes from `Vec`, start from the end.
        assert_eq!(data_vec.remove(127), 0x80);
        assert_eq!(data_vec.remove(15), 0x08);
        assert_eq!(data_vec.remove(0), 0x01);
    }

    #[test]
    fn cpu_set_new_full_succeeds() {
        let all_ids: Vec<_> = (0..1024).collect();
        let cpu_set = CpuSet::new(&all_ids);
        assert!(cpu_set.mask.iter().all(|x| *x == 0xFF));
    }

    #[test]
    #[should_panic(expected = "CPU ID provided to affinity exceeds max supported size, provided: 1024, max: 1023")]
    fn cpu_set_new_out_of_range() {
        let _ = CpuSet::new(&[0, 123, 1023, 1024]);
    }

    #[test]
    fn cpu_set_set_succeeds() {
        let mut cpu_set = CpuSet::new(&[]);
        cpu_set.set(&[0, 123, 1023]);
        let mut data_vec = cpu_set.mask.to_vec();

        // Test removes from `Vec`, start from the end.
        assert_eq!(data_vec.remove(127), 0x80);
        assert_eq!(data_vec.remove(15), 0x08);
        assert_eq!(data_vec.remove(0), 0x01);
    }

    #[test]
    #[should_panic(expected = "CPU ID provided to affinity exceeds max supported size, provided: 1024, max: 1023")]
    fn cpu_set_set_out_of_range() {
        let mut cpu_set = CpuSet::new(&[]);
        cpu_set.set(&[0, 123, 1023, 1024]);
    }

    #[test]
    fn cpu_set_get_succeeds() {
        let exp = vec![0, 123, 1023];
        let cpu_set = CpuSet::new(&exp);
        let got = cpu_set.get();
        assert_eq!(exp, got);
    }
}
