// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit 88dc3695d8bc55c081db4a5646a4968da7587405).

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct DeleteUniqueBoolArgs {
    pub b: bool,
}

impl From<DeleteUniqueBoolArgs> for super::Reducer {
    fn from(args: DeleteUniqueBoolArgs) -> Self {
        Self::DeleteUniqueBool { b: args.b }
    }
}

impl __sdk::InModule for DeleteUniqueBoolArgs {
    type Module = super::RemoteModule;
}

pub struct DeleteUniqueBoolCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `delete_unique_bool`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait delete_unique_bool {
    /// Request that the remote module invoke the reducer `delete_unique_bool` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_delete_unique_bool`] callbacks.
    fn delete_unique_bool(&self, b: bool) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `delete_unique_bool`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`DeleteUniqueBoolCallbackId`] can be passed to [`Self::remove_on_delete_unique_bool`]
    /// to cancel the callback.
    fn on_delete_unique_bool(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &bool) + Send + 'static,
    ) -> DeleteUniqueBoolCallbackId;
    /// Cancel a callback previously registered by [`Self::on_delete_unique_bool`],
    /// causing it not to run in the future.
    fn remove_on_delete_unique_bool(&self, callback: DeleteUniqueBoolCallbackId);
}

impl delete_unique_bool for super::RemoteReducers {
    fn delete_unique_bool(&self, b: bool) -> __sdk::Result<()> {
        self.imp.call_reducer("delete_unique_bool", DeleteUniqueBoolArgs { b })
    }
    fn on_delete_unique_bool(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &bool) + Send + 'static,
    ) -> DeleteUniqueBoolCallbackId {
        DeleteUniqueBoolCallbackId(self.imp.on_reducer(
            "delete_unique_bool",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::DeleteUniqueBool { b },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, b)
            }),
        ))
    }
    fn remove_on_delete_unique_bool(&self, callback: DeleteUniqueBoolCallbackId) {
        self.imp.remove_on_reducer("delete_unique_bool", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `delete_unique_bool`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_delete_unique_bool {
    /// Set the call-reducer flags for the reducer `delete_unique_bool` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn delete_unique_bool(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_delete_unique_bool for super::SetReducerFlags {
    fn delete_unique_bool(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("delete_unique_bool", flags);
    }
}
