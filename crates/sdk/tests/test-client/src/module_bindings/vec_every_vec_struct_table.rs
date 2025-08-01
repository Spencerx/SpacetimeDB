// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit 88dc3695d8bc55c081db4a5646a4968da7587405).

#![allow(unused, clippy::all)]
use super::every_vec_struct_type::EveryVecStruct;
use super::vec_every_vec_struct_type::VecEveryVecStruct;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `vec_every_vec_struct`.
///
/// Obtain a handle from the [`VecEveryVecStructTableAccess::vec_every_vec_struct`] method on [`super::RemoteTables`],
/// like `ctx.db.vec_every_vec_struct()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.vec_every_vec_struct().on_insert(...)`.
pub struct VecEveryVecStructTableHandle<'ctx> {
    imp: __sdk::TableHandle<VecEveryVecStruct>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `vec_every_vec_struct`.
///
/// Implemented for [`super::RemoteTables`].
pub trait VecEveryVecStructTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`VecEveryVecStructTableHandle`], which mediates access to the table `vec_every_vec_struct`.
    fn vec_every_vec_struct(&self) -> VecEveryVecStructTableHandle<'_>;
}

impl VecEveryVecStructTableAccess for super::RemoteTables {
    fn vec_every_vec_struct(&self) -> VecEveryVecStructTableHandle<'_> {
        VecEveryVecStructTableHandle {
            imp: self.imp.get_table::<VecEveryVecStruct>("vec_every_vec_struct"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct VecEveryVecStructInsertCallbackId(__sdk::CallbackId);
pub struct VecEveryVecStructDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for VecEveryVecStructTableHandle<'ctx> {
    type Row = VecEveryVecStruct;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = VecEveryVecStruct> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = VecEveryVecStructInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecEveryVecStructInsertCallbackId {
        VecEveryVecStructInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: VecEveryVecStructInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = VecEveryVecStructDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecEveryVecStructDeleteCallbackId {
        VecEveryVecStructDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: VecEveryVecStructDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<VecEveryVecStruct>("vec_every_vec_struct");
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<VecEveryVecStruct>> {
    __sdk::TableUpdate::parse_table_update(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<VecEveryVecStruct>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}
