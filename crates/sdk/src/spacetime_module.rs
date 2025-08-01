//! Interfaces used by per-module codegen.
//!
//! This module is internal, and may incompatibly change without warning.

use crate::{
    callbacks::DbCallbacks,
    client_cache::ClientCache,
    db_connection::DbContextImpl,
    subscription::{OnEndedCallback, SubscriptionHandleImpl},
    Event, ReducerEvent,
    __codegen::InternalError,
    compression::maybe_decompress_cqu,
};
use bytes::Bytes;
use spacetimedb_client_api_messages::websocket::{self as ws, RowListLen as _};
use spacetimedb_lib::{bsatn, de::DeserializeOwned};
use std::fmt::Debug;

/// Marker trait for any item defined in a module,
/// to conveniently get the types of various per-module things.
pub trait InModule {
    /// Unit type which represents the module itself.
    type Module: SpacetimeModule;
}

/// Each module's codegen will define a unit struct which implements this trait,
/// with associated type links to various other generated types.
pub trait SpacetimeModule: Send + Sync + 'static {
    /// [`crate::DbContext`] implementor which exists in the global scope.
    type DbConnection: DbConnection<Module = Self>;

    /// [`crate::DbContext`] implementor passed to row callbacks.
    type EventContext: EventContext<Module = Self>;

    /// [`crate::DbContext`] implementor passed to reducer callbacks.
    type ReducerEventContext: ReducerEventContext<Module = Self>;

    /// [`crate::DbContext`] implementor passed to subscription on-applied and on-removed callbacks.
    type SubscriptionEventContext: SubscriptionEventContext<Module = Self>;

    /// [`crate::DbContext`] implementor passed to subscription and connection error callbacks.
    type ErrorContext: ErrorContext<Module = Self>;

    /// Enum with variants for each reducer's args; will be contained in [`crate::ReducerEvent`].
    type Reducer: Reducer<Module = Self>;

    /// Return type of [`crate::DbContext::db`].
    type DbView: InModule<Module = Self> + Send + 'static;

    /// Return type of [`crate::DbContext::reducers`].
    type Reducers: InModule<Module = Self> + Send + 'static;

    /// Return type of [`crate::DbContext::set_reducer_flags`].
    type SetReducerFlags: InModule<Module = Self> + Send + 'static;

    /// Parsed and typed analogue of [`crate::ws::DatabaseUpdate`].
    type DbUpdate: DbUpdate<Module = Self>;

    /// The result of applying `Self::DbUpdate` to the client cache.
    type AppliedDiff<'r>: AppliedDiff<'r, Module = Self>;

    /// Module-specific `SubscriptionHandle` type, representing an ongoing incremental subscription to a query.
    type SubscriptionHandle: SubscriptionHandle<Module = Self>;

    /// Called when constructing a [`Self::DbConnection`] on the new connection's [`ClientCache`]
    /// to pre-register tables defined by the module, including their indices.
    fn register_tables(client_cache: &mut ClientCache<Self>);
}

/// Implemented by the autogenerated `DbUpdate` type,
/// which is a parsed and typed analogue of [`crate::ws::DatabaseUpdate`].
pub trait DbUpdate:
    TryFrom<ws::DatabaseUpdate<ws::BsatnFormat>, Error = crate::Error> + InModule + Send + 'static
where
    Self::Module: SpacetimeModule<DbUpdate = Self>,
{
    fn apply_to_client_cache(
        &self,
        cache: &mut ClientCache<Self::Module>,
    ) -> <Self::Module as SpacetimeModule>::AppliedDiff<'_>;

    fn parse_update(update: ws::DatabaseUpdate<ws::BsatnFormat>) -> crate::Result<Self> {
        Self::try_from(update).map_err(|source| {
            InternalError::failed_parse(std::any::type_name::<Self>(), "DatabaseUpdate")
                .with_cause(source)
                .into()
        })
    }
}

/// Implemented by the autogenerated `AppliedDiff` type,
/// which is derived from applying [`DbUpdate`] to the client cache.
pub trait AppliedDiff<'r>: InModule + Send
where
    Self::Module: SpacetimeModule<AppliedDiff<'r> = Self>,
{
    fn invoke_row_callbacks(
        &self,
        event: &<<Self as InModule>::Module as SpacetimeModule>::EventContext,
        callbacks: &mut DbCallbacks<Self::Module>,
    );
}

/// Implemented by the autogenerated `DbConnection` type,
/// which is a [`crate::DbContext`] implementor that SDK users construct.
pub trait DbConnection: InModule + Send + 'static
where
    Self::Module: SpacetimeModule<DbConnection = Self>,
{
    /// Called by [`crate::db_connection::DbConnectionBuilder::build`]
    /// to wrap a `DbConnectionImpl` into the codegen-defined type.
    fn new(imp: DbContextImpl<Self::Module>) -> Self;
}

/// Implemented each the autogenerated `FooContext` type,
/// which is a [`crate::DbContext`] implementor automatically passed to callbacks.
pub trait AbstractEventContext: InModule + Send + 'static {
    type Event;
    /// Get a reference to the [`Event`] contained in this `EventContext`.
    fn event(&self) -> &Self::Event;

    /// Used to construct an `EventContext` which can be passed to callbacks.
    fn new(imp: DbContextImpl<Self::Module>, event: Self::Event) -> Self;
}

/// [`AbstractEventContext`] subtrait for row callbacks.
pub trait EventContext:
    AbstractEventContext<Event = Event<<<Self as InModule>::Module as SpacetimeModule>::Reducer>>
where
    Self::Module: SpacetimeModule<EventContext = Self>,
{
}

/// [`AbstractEventContext`] subtrait for reducer callbacks.
pub trait ReducerEventContext:
    AbstractEventContext<Event = ReducerEvent<<<Self as InModule>::Module as SpacetimeModule>::Reducer>>
where
    Self::Module: SpacetimeModule<ReducerEventContext = Self>,
{
}

/// [`AbstractEventContext`] subtrait for subscription applied and removed callbacks.
pub trait SubscriptionEventContext: AbstractEventContext<Event = ()>
where
    Self::Module: SpacetimeModule<SubscriptionEventContext = Self>,
{
}

/// [`AbstractEventContext`] subtrait for subscription and connection error callbacks.
pub trait ErrorContext: AbstractEventContext<Event = Option<crate::Error>>
where
    Self::Module: SpacetimeModule<ErrorContext = Self>,
{
}

/// Implemented by the autogenerated `Reducer` enum,
/// which has a variant for each reducer defined by the module.
/// This will be the type parameter to [`Event`] and [`crate::ReducerEvent`].
pub trait Reducer:
    InModule
    + TryFrom<ws::ReducerCallInfo<ws::BsatnFormat>, Error = crate::Error>
    + std::fmt::Debug
    + Clone
    + Send
    + 'static
where
    Self::Module: SpacetimeModule<Reducer = Self>,
{
    /// Get the string name of the reducer variant stored in this instance.
    ///
    /// Used by [`crate::callbacks::ReducerCallbacks::invoke_on_reducer`] to determine which callback to run.
    fn reducer_name(&self) -> &'static str;
}

pub trait SubscriptionHandle: InModule + Clone + Send + 'static
where
    Self::Module: SpacetimeModule<SubscriptionHandle = Self>,
{
    fn new(imp: SubscriptionHandleImpl<Self::Module>) -> Self;
    fn is_ended(&self) -> bool;

    fn is_active(&self) -> bool;

    /// Unsubscribe from the query controlled by this `SubscriptionHandle`,
    /// then run `on_end` when its rows are removed from the client cache.
    /// Returns an error if the subscription is already ended,
    /// or if unsubscribe has already been called.
    fn unsubscribe_then(self, on_end: OnEndedCallback<Self::Module>) -> crate::Result<()>;

    /// Unsubscribe from the query controlled by this `SubscriptionHandle`.
    /// Returns an error if the subscription is already ended,
    /// or if unsubscribe has already been called.
    fn unsubscribe(self) -> crate::Result<()>;
}

pub struct WithBsatn<Row> {
    pub bsatn: Bytes,
    pub row: Row,
}

pub struct TableUpdate<Row> {
    pub inserts: Vec<WithBsatn<Row>>,
    pub deletes: Vec<WithBsatn<Row>>,
}

impl<Row> Default for TableUpdate<Row> {
    fn default() -> Self {
        Self {
            inserts: Default::default(),
            deletes: Default::default(),
        }
    }
}

impl<Row> TableUpdate<Row> {
    pub fn append(&mut self, mut other: TableUpdate<Row>) {
        self.inserts.append(&mut other.inserts);
        self.deletes.append(&mut other.deletes);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.inserts.is_empty() && self.deletes.is_empty()
    }
}

impl<Row: DeserializeOwned + Debug> TableUpdate<Row> {
    /// Parse `raw_updates` into a [`TableUpdate`].
    pub fn parse_table_update(raw_updates: ws::TableUpdate<ws::BsatnFormat>) -> crate::Result<TableUpdate<Row>> {
        let mut inserts = Vec::new();
        let mut deletes = Vec::new();
        for update in raw_updates.updates {
            let update = maybe_decompress_cqu(update);
            Self::parse_from_row_list(&mut deletes, &update.deletes)?;
            Self::parse_from_row_list(&mut inserts, &update.inserts)?;
        }
        Ok(Self { inserts, deletes })
    }

    fn parse_from_row_list(sink: &mut Vec<WithBsatn<Row>>, raw_rows: &ws::BsatnRowList) -> crate::Result<()> {
        sink.reserve(raw_rows.len());
        for raw_row in raw_rows {
            sink.push(Self::parse_row(raw_row)?);
        }
        Ok(())
    }

    fn parse_row(bytes: Bytes) -> crate::Result<WithBsatn<Row>> {
        let parsed = bsatn::from_slice::<Row>(&bytes).map_err(|source| {
            InternalError::failed_parse(std::any::type_name::<Row>(), "row data").with_cause(source)
        })?;
        Ok(WithBsatn {
            bsatn: bytes,
            row: parsed,
        })
    }
}

pub fn parse_reducer_args<Args: DeserializeOwned>(reducer_name: &'static str, args: &[u8]) -> crate::Result<Args> {
    bsatn::from_slice::<Args>(args).map_err(|source| {
        InternalError::failed_parse(std::any::type_name::<Args>(), reducer_name)
            .with_cause(source)
            .into()
    })
}
