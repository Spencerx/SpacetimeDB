// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit 88dc3695d8bc55c081db4a5646a4968da7587405).

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct UpdateUniqueBoolArgs {
    pub b: bool,
    pub data: i32,
}

impl From<UpdateUniqueBoolArgs> for super::Reducer {
    fn from(args: UpdateUniqueBoolArgs) -> Self {
        Self::UpdateUniqueBool {
            b: args.b,
            data: args.data,
        }
    }
}

impl __sdk::InModule for UpdateUniqueBoolArgs {
    type Module = super::RemoteModule;
}

pub struct UpdateUniqueBoolCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `update_unique_bool`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait update_unique_bool {
    /// Request that the remote module invoke the reducer `update_unique_bool` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_update_unique_bool`] callbacks.
    fn update_unique_bool(&self, b: bool, data: i32) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `update_unique_bool`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`UpdateUniqueBoolCallbackId`] can be passed to [`Self::remove_on_update_unique_bool`]
    /// to cancel the callback.
    fn on_update_unique_bool(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &bool, &i32) + Send + 'static,
    ) -> UpdateUniqueBoolCallbackId;
    /// Cancel a callback previously registered by [`Self::on_update_unique_bool`],
    /// causing it not to run in the future.
    fn remove_on_update_unique_bool(&self, callback: UpdateUniqueBoolCallbackId);
}

impl update_unique_bool for super::RemoteReducers {
    fn update_unique_bool(&self, b: bool, data: i32) -> __sdk::Result<()> {
        self.imp
            .call_reducer("update_unique_bool", UpdateUniqueBoolArgs { b, data })
    }
    fn on_update_unique_bool(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &bool, &i32) + Send + 'static,
    ) -> UpdateUniqueBoolCallbackId {
        UpdateUniqueBoolCallbackId(self.imp.on_reducer(
            "update_unique_bool",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::UpdateUniqueBool { b, data },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, b, data)
            }),
        ))
    }
    fn remove_on_update_unique_bool(&self, callback: UpdateUniqueBoolCallbackId) {
        self.imp.remove_on_reducer("update_unique_bool", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `update_unique_bool`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_update_unique_bool {
    /// Set the call-reducer flags for the reducer `update_unique_bool` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn update_unique_bool(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_update_unique_bool for super::SetReducerFlags {
    fn update_unique_bool(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("update_unique_bool", flags);
    }
}
