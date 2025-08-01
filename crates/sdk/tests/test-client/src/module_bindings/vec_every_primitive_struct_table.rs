// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit 88dc3695d8bc55c081db4a5646a4968da7587405).

#![allow(unused, clippy::all)]
use super::every_primitive_struct_type::EveryPrimitiveStruct;
use super::vec_every_primitive_struct_type::VecEveryPrimitiveStruct;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `vec_every_primitive_struct`.
///
/// Obtain a handle from the [`VecEveryPrimitiveStructTableAccess::vec_every_primitive_struct`] method on [`super::RemoteTables`],
/// like `ctx.db.vec_every_primitive_struct()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.vec_every_primitive_struct().on_insert(...)`.
pub struct VecEveryPrimitiveStructTableHandle<'ctx> {
    imp: __sdk::TableHandle<VecEveryPrimitiveStruct>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `vec_every_primitive_struct`.
///
/// Implemented for [`super::RemoteTables`].
pub trait VecEveryPrimitiveStructTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`VecEveryPrimitiveStructTableHandle`], which mediates access to the table `vec_every_primitive_struct`.
    fn vec_every_primitive_struct(&self) -> VecEveryPrimitiveStructTableHandle<'_>;
}

impl VecEveryPrimitiveStructTableAccess for super::RemoteTables {
    fn vec_every_primitive_struct(&self) -> VecEveryPrimitiveStructTableHandle<'_> {
        VecEveryPrimitiveStructTableHandle {
            imp: self
                .imp
                .get_table::<VecEveryPrimitiveStruct>("vec_every_primitive_struct"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct VecEveryPrimitiveStructInsertCallbackId(__sdk::CallbackId);
pub struct VecEveryPrimitiveStructDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for VecEveryPrimitiveStructTableHandle<'ctx> {
    type Row = VecEveryPrimitiveStruct;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = VecEveryPrimitiveStruct> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = VecEveryPrimitiveStructInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecEveryPrimitiveStructInsertCallbackId {
        VecEveryPrimitiveStructInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: VecEveryPrimitiveStructInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = VecEveryPrimitiveStructDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecEveryPrimitiveStructDeleteCallbackId {
        VecEveryPrimitiveStructDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: VecEveryPrimitiveStructDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<VecEveryPrimitiveStruct>("vec_every_primitive_struct");
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<VecEveryPrimitiveStruct>> {
    __sdk::TableUpdate::parse_table_update(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<VecEveryPrimitiveStruct>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}
