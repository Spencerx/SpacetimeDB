// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit 88dc3695d8bc55c081db4a5646a4968da7587405).

#![allow(unused, clippy::all)]
use super::every_primitive_struct_type::EveryPrimitiveStruct;
use super::option_every_primitive_struct_type::OptionEveryPrimitiveStruct;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `option_every_primitive_struct`.
///
/// Obtain a handle from the [`OptionEveryPrimitiveStructTableAccess::option_every_primitive_struct`] method on [`super::RemoteTables`],
/// like `ctx.db.option_every_primitive_struct()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.option_every_primitive_struct().on_insert(...)`.
pub struct OptionEveryPrimitiveStructTableHandle<'ctx> {
    imp: __sdk::TableHandle<OptionEveryPrimitiveStruct>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `option_every_primitive_struct`.
///
/// Implemented for [`super::RemoteTables`].
pub trait OptionEveryPrimitiveStructTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`OptionEveryPrimitiveStructTableHandle`], which mediates access to the table `option_every_primitive_struct`.
    fn option_every_primitive_struct(&self) -> OptionEveryPrimitiveStructTableHandle<'_>;
}

impl OptionEveryPrimitiveStructTableAccess for super::RemoteTables {
    fn option_every_primitive_struct(&self) -> OptionEveryPrimitiveStructTableHandle<'_> {
        OptionEveryPrimitiveStructTableHandle {
            imp: self
                .imp
                .get_table::<OptionEveryPrimitiveStruct>("option_every_primitive_struct"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct OptionEveryPrimitiveStructInsertCallbackId(__sdk::CallbackId);
pub struct OptionEveryPrimitiveStructDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for OptionEveryPrimitiveStructTableHandle<'ctx> {
    type Row = OptionEveryPrimitiveStruct;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = OptionEveryPrimitiveStruct> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = OptionEveryPrimitiveStructInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OptionEveryPrimitiveStructInsertCallbackId {
        OptionEveryPrimitiveStructInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: OptionEveryPrimitiveStructInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = OptionEveryPrimitiveStructDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OptionEveryPrimitiveStructDeleteCallbackId {
        OptionEveryPrimitiveStructDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: OptionEveryPrimitiveStructDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<OptionEveryPrimitiveStruct>("option_every_primitive_struct");
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<OptionEveryPrimitiveStruct>> {
    __sdk::TableUpdate::parse_table_update(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<OptionEveryPrimitiveStruct>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}
