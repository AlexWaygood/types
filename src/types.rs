use std::collections::HashSet;

#[allow(clippy::enum_variant_names)]
enum Type {
    Unknown,
    Never,

    /// E.g. `x: MyClass` in the global scope
    ModuleInstance(AnywhereInstanceType),

    /// E.g. `x: type[MyClass]` in the global scope
    ModuleType(AnywhereTypeType),

    /// E.g. `x: int | str` in the global scope
    ModuleInstanceUnion(UnionType<AnywhereUnion>),

    /// E.g. `x: type[int | str]` in the global scope
    ModuleTypeUnion(UnionType<AnywhereTypeType>),

    /// E.g. `x` or `y` in:
    ///
    /// ```py
    /// def foo(x: T) -> T:
    ///     y: MyClass
    /// ```
    FunctionInstance(FunctionInstanceType),

    /// E.g. `x` or `y` in:
    ///
    /// ```py
    /// def foo[T](x: type[T]) -> T:
    ///     y: type[MyClass]
    /// ```
    FunctionType(FunctionTypeType),

    /// E.g. `x` or `y` in:
    ///
    /// ```py
    /// def foo(x: int | str):
    ///     y: str | bytes
    /// ```
    FunctionInstanceUnion(UnionType<FunctionUnion>),

    /// E.g. `x` or `y` in:
    ///
    /// ```py
    /// def foo(x: type[int | str]):
    ///     y: type[str | bytes]
    /// ```
    FunctionTypeUnion(UnionType<FunctionTypeType>),

    /// E.g.
    ///
    /// ```py
    /// class Foo:
    ///     def bar(self: Self): ...
    /// ```
    MethodInstance(MethodInstanceType),

    /// E.g.
    ///
    /// ```py
    /// class Foo:
    ///     @classmethod
    ///     def bar(cls: type[Self]): ...
    /// ```
    MethodType(MethodTypeType),

    /// E.g.
    ///
    /// ```py
    /// class Foo:
    ///     def bar(self: Self | str): ...
    /// ```
    MethodInstanceUnion(UnionType<MethodUnion>),

    /// E.g.
    ///
    /// ```py
    /// class Foo:
    ///     @classmethod
    ///     def bar(cls: type[Self | str]): ...
    /// ```
    MethodTypeUnion(UnionType<MethodTypeType>),
}

enum AnywhereUnion {
    AnywhereInstance(AnywhereInstanceType),
    AnywhereType(AnywhereTypeType),
}

enum AnywhereInstanceType {
    Class(ClassDef),
    Callable(CallableType),
    Tuple(TupleType),
    Literal(LiteralType),
    NamedTuple(NamedTupleType),
    TypedDict(TypedDictType),
    Any,
    LiteralString,
}

enum AnywhereTypeType {
    Class(ClassDef),
    NamedTuple(NamedTupleType),
    Any,
}

enum FunctionUnion {
    AnywhereInstance(AnywhereInstanceType),
    TypeVar(TypeVarType),
    FunctionType(FunctionTypeType),
}

enum TypeVarLikeType {
    TypeVar(TypeVarType),
    UnpackedTypeVarTuple(UnpackedTypeVarTupleType),
    ParamSpecArgs(ParamSpecArgsType),
    ParamSpecKwargs(ParamSpecKwargsType),
}

enum FunctionInstanceType {
    AnywhereInstance(AnywhereInstanceType),
    Variable(TypeVarLikeType),
}

enum FunctionTypeType {
    AnywhereType(AnywhereTypeType),
    TypeVar(TypeVarType),
}

enum MethodUnion {
    AnywhereInstance(AnywhereInstanceType),
    TypeVar(TypeVarType),
    SelfT(SelfType),
    MethodType(MethodTypeType),
}

enum MethodInstanceType {
    AnywhereInstance(AnywhereInstanceType),
    Variable(TypeVarLikeType),
    SelfT(SelfType),
}

enum MethodTypeType {
    AnywhereType(AnywhereTypeType),
    TypeVar(TypeVarType),
    SelfT(SelfType),
}

enum CallableType {
    /// Callable types created using `{typing, collections.abc}.Callable`
    Abstract(AbstractCallableType),
    /// Callable types created using `def` statements or lambda expressions
    Specific(SpecificCallableType),
}

// or we could have:
// ```rs
// enum ClassDef {
//     Nominal(NominalClassDef),
//     Protocol(ProtocolClassDef),
// }
// ```
struct ClassDef {
    protocol: bool,
    // etc.
}

struct TypeVarType {}
struct UnpackedTypeVarTupleType {}
struct ParamSpecArgsType {}
struct ParamSpecKwargsType {}
struct AbstractCallableType {}
struct SpecificCallableType {}
struct TupleType {}
struct SelfType {}
struct LiteralType {}
struct NamedTupleType {}

// Or should we have a `StructuralType` enum?
// ```rs
// enum StructuralType {
//     Protocol,
//     TypedDict,
// }
struct TypedDictType {}

struct UnionType<T> {
    members: HashSet<T>,
}
