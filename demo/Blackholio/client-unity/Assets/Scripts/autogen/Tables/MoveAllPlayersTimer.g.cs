// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit ).

#nullable enable

using System;
using SpacetimeDB.BSATN;
using SpacetimeDB.ClientApi;
using System.Collections.Generic;
using System.Runtime.Serialization;

namespace SpacetimeDB.Types
{
    public sealed partial class RemoteTables
    {
        public sealed class MoveAllPlayersTimerHandle : RemoteTableHandle<EventContext, MoveAllPlayersTimer>
        {
            protected override string RemoteTableName => "move_all_players_timer";

            public sealed class ScheduledIdUniqueIndex : UniqueIndexBase<ulong>
            {
                protected override ulong GetKey(MoveAllPlayersTimer row) => row.ScheduledId;

                public ScheduledIdUniqueIndex(MoveAllPlayersTimerHandle table) : base(table) { }
            }

            public readonly ScheduledIdUniqueIndex ScheduledId;

            internal MoveAllPlayersTimerHandle(DbConnection conn) : base(conn)
            {
                ScheduledId = new(this);
            }

            protected override object GetPrimaryKey(MoveAllPlayersTimer row) => row.ScheduledId;
        }

        public readonly MoveAllPlayersTimerHandle MoveAllPlayersTimer;
    }
}
