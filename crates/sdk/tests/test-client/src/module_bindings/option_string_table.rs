// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit 88dc3695d8bc55c081db4a5646a4968da7587405).

#![allow(unused, clippy::all)]
use super::option_string_type::OptionString;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `option_string`.
///
/// Obtain a handle from the [`OptionStringTableAccess::option_string`] method on [`super::RemoteTables`],
/// like `ctx.db.option_string()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.option_string().on_insert(...)`.
pub struct OptionStringTableHandle<'ctx> {
    imp: __sdk::TableHandle<OptionString>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `option_string`.
///
/// Implemented for [`super::RemoteTables`].
pub trait OptionStringTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`OptionStringTableHandle`], which mediates access to the table `option_string`.
    fn option_string(&self) -> OptionStringTableHandle<'_>;
}

impl OptionStringTableAccess for super::RemoteTables {
    fn option_string(&self) -> OptionStringTableHandle<'_> {
        OptionStringTableHandle {
            imp: self.imp.get_table::<OptionString>("option_string"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct OptionStringInsertCallbackId(__sdk::CallbackId);
pub struct OptionStringDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for OptionStringTableHandle<'ctx> {
    type Row = OptionString;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = OptionString> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = OptionStringInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OptionStringInsertCallbackId {
        OptionStringInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: OptionStringInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = OptionStringDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OptionStringDeleteCallbackId {
        OptionStringDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: OptionStringDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<OptionString>("option_string");
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<OptionString>> {
    __sdk::TableUpdate::parse_table_update(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<OptionString>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}
