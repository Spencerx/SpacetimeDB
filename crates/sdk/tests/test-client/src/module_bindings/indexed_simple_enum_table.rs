// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit 88dc3695d8bc55c081db4a5646a4968da7587405).

#![allow(unused, clippy::all)]
use super::indexed_simple_enum_type::IndexedSimpleEnum;
use super::simple_enum_type::SimpleEnum;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `indexed_simple_enum`.
///
/// Obtain a handle from the [`IndexedSimpleEnumTableAccess::indexed_simple_enum`] method on [`super::RemoteTables`],
/// like `ctx.db.indexed_simple_enum()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.indexed_simple_enum().on_insert(...)`.
pub struct IndexedSimpleEnumTableHandle<'ctx> {
    imp: __sdk::TableHandle<IndexedSimpleEnum>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `indexed_simple_enum`.
///
/// Implemented for [`super::RemoteTables`].
pub trait IndexedSimpleEnumTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`IndexedSimpleEnumTableHandle`], which mediates access to the table `indexed_simple_enum`.
    fn indexed_simple_enum(&self) -> IndexedSimpleEnumTableHandle<'_>;
}

impl IndexedSimpleEnumTableAccess for super::RemoteTables {
    fn indexed_simple_enum(&self) -> IndexedSimpleEnumTableHandle<'_> {
        IndexedSimpleEnumTableHandle {
            imp: self.imp.get_table::<IndexedSimpleEnum>("indexed_simple_enum"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct IndexedSimpleEnumInsertCallbackId(__sdk::CallbackId);
pub struct IndexedSimpleEnumDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for IndexedSimpleEnumTableHandle<'ctx> {
    type Row = IndexedSimpleEnum;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = IndexedSimpleEnum> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = IndexedSimpleEnumInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> IndexedSimpleEnumInsertCallbackId {
        IndexedSimpleEnumInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: IndexedSimpleEnumInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = IndexedSimpleEnumDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> IndexedSimpleEnumDeleteCallbackId {
        IndexedSimpleEnumDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: IndexedSimpleEnumDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<IndexedSimpleEnum>("indexed_simple_enum");
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<IndexedSimpleEnum>> {
    __sdk::TableUpdate::parse_table_update(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<IndexedSimpleEnum>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}
