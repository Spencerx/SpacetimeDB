// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit 88dc3695d8bc55c081db4a5646a4968da7587405).

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

use super::unit_struct_type::UnitStruct;

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct InsertOneUnitStructArgs {
    pub s: UnitStruct,
}

impl From<InsertOneUnitStructArgs> for super::Reducer {
    fn from(args: InsertOneUnitStructArgs) -> Self {
        Self::InsertOneUnitStruct { s: args.s }
    }
}

impl __sdk::InModule for InsertOneUnitStructArgs {
    type Module = super::RemoteModule;
}

pub struct InsertOneUnitStructCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_one_unit_struct`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_one_unit_struct {
    /// Request that the remote module invoke the reducer `insert_one_unit_struct` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_one_unit_struct`] callbacks.
    fn insert_one_unit_struct(&self, s: UnitStruct) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_one_unit_struct`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertOneUnitStructCallbackId`] can be passed to [`Self::remove_on_insert_one_unit_struct`]
    /// to cancel the callback.
    fn on_insert_one_unit_struct(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &UnitStruct) + Send + 'static,
    ) -> InsertOneUnitStructCallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_one_unit_struct`],
    /// causing it not to run in the future.
    fn remove_on_insert_one_unit_struct(&self, callback: InsertOneUnitStructCallbackId);
}

impl insert_one_unit_struct for super::RemoteReducers {
    fn insert_one_unit_struct(&self, s: UnitStruct) -> __sdk::Result<()> {
        self.imp
            .call_reducer("insert_one_unit_struct", InsertOneUnitStructArgs { s })
    }
    fn on_insert_one_unit_struct(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &UnitStruct) + Send + 'static,
    ) -> InsertOneUnitStructCallbackId {
        InsertOneUnitStructCallbackId(self.imp.on_reducer(
            "insert_one_unit_struct",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::InsertOneUnitStruct { s },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, s)
            }),
        ))
    }
    fn remove_on_insert_one_unit_struct(&self, callback: InsertOneUnitStructCallbackId) {
        self.imp.remove_on_reducer("insert_one_unit_struct", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `insert_one_unit_struct`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_insert_one_unit_struct {
    /// Set the call-reducer flags for the reducer `insert_one_unit_struct` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn insert_one_unit_struct(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_insert_one_unit_struct for super::SetReducerFlags {
    fn insert_one_unit_struct(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("insert_one_unit_struct", flags);
    }
}
