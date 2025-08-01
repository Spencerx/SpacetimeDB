// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit 88dc3695d8bc55c081db4a5646a4968da7587405).

#![allow(unused, clippy::all)]
use super::unique_connection_id_type::UniqueConnectionId;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `unique_connection_id`.
///
/// Obtain a handle from the [`UniqueConnectionIdTableAccess::unique_connection_id`] method on [`super::RemoteTables`],
/// like `ctx.db.unique_connection_id()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.unique_connection_id().on_insert(...)`.
pub struct UniqueConnectionIdTableHandle<'ctx> {
    imp: __sdk::TableHandle<UniqueConnectionId>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `unique_connection_id`.
///
/// Implemented for [`super::RemoteTables`].
pub trait UniqueConnectionIdTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`UniqueConnectionIdTableHandle`], which mediates access to the table `unique_connection_id`.
    fn unique_connection_id(&self) -> UniqueConnectionIdTableHandle<'_>;
}

impl UniqueConnectionIdTableAccess for super::RemoteTables {
    fn unique_connection_id(&self) -> UniqueConnectionIdTableHandle<'_> {
        UniqueConnectionIdTableHandle {
            imp: self.imp.get_table::<UniqueConnectionId>("unique_connection_id"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct UniqueConnectionIdInsertCallbackId(__sdk::CallbackId);
pub struct UniqueConnectionIdDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for UniqueConnectionIdTableHandle<'ctx> {
    type Row = UniqueConnectionId;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = UniqueConnectionId> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = UniqueConnectionIdInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueConnectionIdInsertCallbackId {
        UniqueConnectionIdInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: UniqueConnectionIdInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = UniqueConnectionIdDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueConnectionIdDeleteCallbackId {
        UniqueConnectionIdDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: UniqueConnectionIdDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<UniqueConnectionId>("unique_connection_id");
    _table.add_unique_constraint::<__sdk::ConnectionId>("a", |row| &row.a);
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<UniqueConnectionId>> {
    __sdk::TableUpdate::parse_table_update(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<UniqueConnectionId>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}

/// Access to the `a` unique index on the table `unique_connection_id`,
/// which allows point queries on the field of the same name
/// via the [`UniqueConnectionIdAUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.unique_connection_id().a().find(...)`.
pub struct UniqueConnectionIdAUnique<'ctx> {
    imp: __sdk::UniqueConstraintHandle<UniqueConnectionId, __sdk::ConnectionId>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> UniqueConnectionIdTableHandle<'ctx> {
    /// Get a handle on the `a` unique index on the table `unique_connection_id`.
    pub fn a(&self) -> UniqueConnectionIdAUnique<'ctx> {
        UniqueConnectionIdAUnique {
            imp: self.imp.get_unique_constraint::<__sdk::ConnectionId>("a"),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> UniqueConnectionIdAUnique<'ctx> {
    /// Find the subscribed row whose `a` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &__sdk::ConnectionId) -> Option<UniqueConnectionId> {
        self.imp.find(col_val)
    }
}
