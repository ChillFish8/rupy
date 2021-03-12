#![allow(unused)]

use slab::Slab;
use std::borrow::Borrow;


use crate::types::RawTypes;
use crate::functions::PyFunction;
use crate::constructors::PyClass;


/// A wrapper around type T with a reference counter.
///
/// This is used to track the amount of active and inactive objects
/// allocated on the slab of memory. If a reference count is 0 it means
/// it is no-longer used and can be dropped to free up a new slot on our
/// slab.
///
/// When a new value is created it's reference count is automatically 1
/// so does not need to be incremented when it's first made.
///
/// Note on memory safety:
///     This does NOT stop reference cycles, as it has no knowledge of any
///     context outside of it's reference count and it's wrapping value.
pub struct SlabValue<T> {
    value: T,
    references: usize,
}

impl<T> SlabValue<T> {
    /// Wraps a given value of type T with the struct.
    pub fn from(val: T) -> Self {
        Self {
            value: val,
            references: 1,
        }
    }

    /// Increments the reference count by +1
    fn increment_count(&mut self) {
        self.references += 1;
    }

    /// Increments the reference count by -1
    fn decrement_count(&mut self) -> bool  {
        self.references -= 1;

        return self.references == 0
    }

    pub fn inner(&self) -> &T {
        self.value.borrow()
    }
}


/// The runtime memory manager.
///
/// The manager is divided into three separate blocks of allocation:
///
/// objects:
///     Any constant values, objects etc...
///
/// functions:
///     These can be any type of function or method, these are all treated
///     as the same thing just with some default values set.
///
/// constructors:
///     Any class constructors that can be used to produce objects e.g.
///     how they're described to be made.
pub struct RuntimeMemory {
    /// RawTypes, objects, etc...
    objects: Slab<SlabValue<RawTypes>>,

    /// Any function definitions.
    functions: Slab<SlabValue<PyFunction>>,

    /// Any class constructors definition.
    constructors: Slab<SlabValue<PyClass>>,
}