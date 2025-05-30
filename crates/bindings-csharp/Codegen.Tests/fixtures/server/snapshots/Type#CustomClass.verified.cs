﻿//HintName: CustomClass.cs
// <auto-generated />
#nullable enable

partial class CustomClass : System.IEquatable<CustomClass>, SpacetimeDB.BSATN.IStructuralReadWrite
{
    public void ReadFields(System.IO.BinaryReader reader)
    {
        IntField = BSATN.IntField.Read(reader);
        StringField = BSATN.StringField.Read(reader);
        NullableIntField = BSATN.NullableIntField.Read(reader);
        NullableStringField = BSATN.NullableStringField.Read(reader);
    }

    public void WriteFields(System.IO.BinaryWriter writer)
    {
        BSATN.IntField.Write(writer, IntField);
        BSATN.StringField.Write(writer, StringField);
        BSATN.NullableIntField.Write(writer, NullableIntField);
        BSATN.NullableStringField.Write(writer, NullableStringField);
    }

    object SpacetimeDB.BSATN.IStructuralReadWrite.GetSerializer()
    {
        return new BSATN();
    }

    public override string ToString() =>
        $"CustomClass {{ IntField = {SpacetimeDB.BSATN.StringUtil.GenericToString(IntField)}, StringField = {SpacetimeDB.BSATN.StringUtil.GenericToString(StringField)}, NullableIntField = {SpacetimeDB.BSATN.StringUtil.GenericToString(NullableIntField)}, NullableStringField = {SpacetimeDB.BSATN.StringUtil.GenericToString(NullableStringField)} }}";

    public readonly partial struct BSATN : SpacetimeDB.BSATN.IReadWrite<CustomClass>
    {
        internal static readonly SpacetimeDB.BSATN.I32 IntField = new();
        internal static readonly SpacetimeDB.BSATN.String StringField = new();
        internal static readonly SpacetimeDB.BSATN.ValueOption<
            int,
            SpacetimeDB.BSATN.I32
        > NullableIntField = new();
        internal static readonly SpacetimeDB.BSATN.RefOption<
            string,
            SpacetimeDB.BSATN.String
        > NullableStringField = new();

        public CustomClass Read(System.IO.BinaryReader reader)
        {
            var ___result = new CustomClass();
            ___result.ReadFields(reader);
            return ___result;
        }

        public void Write(System.IO.BinaryWriter writer, CustomClass value)
        {
            value.WriteFields(writer);
        }

        public SpacetimeDB.BSATN.AlgebraicType.Ref GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) =>
            registrar.RegisterType<CustomClass>(_ => new SpacetimeDB.BSATN.AlgebraicType.Product(
                new SpacetimeDB.BSATN.AggregateElement[]
                {
                    new(nameof(IntField), IntField.GetAlgebraicType(registrar)),
                    new(nameof(StringField), StringField.GetAlgebraicType(registrar)),
                    new(nameof(NullableIntField), NullableIntField.GetAlgebraicType(registrar)),
                    new(
                        nameof(NullableStringField),
                        NullableStringField.GetAlgebraicType(registrar)
                    )
                }
            ));

        SpacetimeDB.BSATN.AlgebraicType SpacetimeDB.BSATN.IReadWrite<CustomClass>.GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) => GetAlgebraicType(registrar);
    }

    public override int GetHashCode()
    {
        var ___hashIntField = IntField.GetHashCode();
        var ___hashStringField = StringField == null ? 0 : StringField.GetHashCode();
        var ___hashNullableIntField = NullableIntField.GetHashCode();
        var ___hashNullableStringField =
            NullableStringField == null ? 0 : NullableStringField.GetHashCode();
        return ___hashIntField
            ^ ___hashStringField
            ^ ___hashNullableIntField
            ^ ___hashNullableStringField;
    }

#nullable enable
    public bool Equals(CustomClass? that)
    {
        if (((object?)that) == null)
        {
            return false;
        }

        var ___eqIntField = this.IntField.Equals(that.IntField);
        var ___eqStringField =
            this.StringField == null
                ? that.StringField == null
                : this.StringField.Equals(that.StringField);
        var ___eqNullableIntField = this.NullableIntField.Equals(that.NullableIntField);
        var ___eqNullableStringField =
            this.NullableStringField == null
                ? that.NullableStringField == null
                : this.NullableStringField.Equals(that.NullableStringField);
        return ___eqIntField
            && ___eqStringField
            && ___eqNullableIntField
            && ___eqNullableStringField;
    }

    public override bool Equals(object? that)
    {
        if (that == null)
        {
            return false;
        }
        var that_ = that as CustomClass;
        if (((object?)that_) == null)
        {
            return false;
        }
        return Equals(that_);
    }

    public static bool operator ==(CustomClass? this_, CustomClass? that)
    {
        if (((object?)this_) == null || ((object?)that) == null)
        {
            return object.Equals(this_, that);
        }
        return this_.Equals(that);
    }

    public static bool operator !=(CustomClass? this_, CustomClass? that)
    {
        if (((object?)this_) == null || ((object?)that) == null)
        {
            return !object.Equals(this_, that);
        }
        return !this_.Equals(that);
    }
#nullable restore
} // CustomClass
