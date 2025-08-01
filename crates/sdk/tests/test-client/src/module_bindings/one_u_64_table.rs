// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit 88dc3695d8bc55c081db4a5646a4968da7587405).

#![allow(unused, clippy::all)]
use super::one_u_64_type::OneU64;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `one_u64`.
///
/// Obtain a handle from the [`OneU64TableAccess::one_u_64`] method on [`super::RemoteTables`],
/// like `ctx.db.one_u_64()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.one_u_64().on_insert(...)`.
pub struct OneU64TableHandle<'ctx> {
    imp: __sdk::TableHandle<OneU64>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `one_u64`.
///
/// Implemented for [`super::RemoteTables`].
pub trait OneU64TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`OneU64TableHandle`], which mediates access to the table `one_u64`.
    fn one_u_64(&self) -> OneU64TableHandle<'_>;
}

impl OneU64TableAccess for super::RemoteTables {
    fn one_u_64(&self) -> OneU64TableHandle<'_> {
        OneU64TableHandle {
            imp: self.imp.get_table::<OneU64>("one_u64"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct OneU64InsertCallbackId(__sdk::CallbackId);
pub struct OneU64DeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for OneU64TableHandle<'ctx> {
    type Row = OneU64;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = OneU64> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = OneU64InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneU64InsertCallbackId {
        OneU64InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: OneU64InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = OneU64DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneU64DeleteCallbackId {
        OneU64DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: OneU64DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<OneU64>("one_u64");
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<OneU64>> {
    __sdk::TableUpdate::parse_table_update(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<OneU64>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}
