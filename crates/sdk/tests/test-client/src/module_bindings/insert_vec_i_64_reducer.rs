// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit 88dc3695d8bc55c081db4a5646a4968da7587405).

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct InsertVecI64Args {
    pub n: Vec<i64>,
}

impl From<InsertVecI64Args> for super::Reducer {
    fn from(args: InsertVecI64Args) -> Self {
        Self::InsertVecI64 { n: args.n }
    }
}

impl __sdk::InModule for InsertVecI64Args {
    type Module = super::RemoteModule;
}

pub struct InsertVecI64CallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_vec_i64`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_vec_i_64 {
    /// Request that the remote module invoke the reducer `insert_vec_i64` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_vec_i_64`] callbacks.
    fn insert_vec_i_64(&self, n: Vec<i64>) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_vec_i64`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertVecI64CallbackId`] can be passed to [`Self::remove_on_insert_vec_i_64`]
    /// to cancel the callback.
    fn on_insert_vec_i_64(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &Vec<i64>) + Send + 'static,
    ) -> InsertVecI64CallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_vec_i_64`],
    /// causing it not to run in the future.
    fn remove_on_insert_vec_i_64(&self, callback: InsertVecI64CallbackId);
}

impl insert_vec_i_64 for super::RemoteReducers {
    fn insert_vec_i_64(&self, n: Vec<i64>) -> __sdk::Result<()> {
        self.imp.call_reducer("insert_vec_i64", InsertVecI64Args { n })
    }
    fn on_insert_vec_i_64(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &Vec<i64>) + Send + 'static,
    ) -> InsertVecI64CallbackId {
        InsertVecI64CallbackId(self.imp.on_reducer(
            "insert_vec_i64",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::InsertVecI64 { n },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, n)
            }),
        ))
    }
    fn remove_on_insert_vec_i_64(&self, callback: InsertVecI64CallbackId) {
        self.imp.remove_on_reducer("insert_vec_i64", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `insert_vec_i64`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_insert_vec_i_64 {
    /// Set the call-reducer flags for the reducer `insert_vec_i64` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn insert_vec_i_64(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_insert_vec_i_64 for super::SetReducerFlags {
    fn insert_vec_i_64(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("insert_vec_i64", flags);
    }
}
