// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit ).

#nullable enable

using System;
using System.Collections.Generic;
using System.Runtime.Serialization;

namespace SpacetimeDB.Types
{
    [SpacetimeDB.Type]
    [DataContract]
    public sealed partial class DbVector2
    {
        [DataMember(Name = "x")]
        public float X;
        [DataMember(Name = "y")]
        public float Y;

        public DbVector2(
            float X,
            float Y
        )
        {
            this.X = X;
            this.Y = Y;
        }

        public DbVector2()
        {
        }
    }
}
