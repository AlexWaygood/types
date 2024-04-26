use std::collections::HashSet;

struct Type {
    kind: TypeKind,
    qualifiers: HashSet<TypeQualifier>,
}

enum TypeKind {
    Never,

    /// E.g.:
    /// - `x: int`
    /// - `x: int | str`
    /// - `x: T` (where `T` is a type variable)
    /// - `x: Any`
    Instance(InstanceTypeKind),

    /// E.g.
    /// - `x: type[int]`
    /// - `x: type[int | str]`
    /// - `x: type[T]` (where `T` is a type variable)
    /// - `x: type[Any]`
    Type(TypeTypeKind),
}

enum TypeQualifier {
    ClassVar,
    Final,
    Required,
    NotRequired,
}

enum InstanceTypeKind {
    Class(ClassDef),
    Union(UnionType),
    TypeVar(TypeVarType),
    // e.g. `args` in `def f[*Ts](*args: *Ts): ...`
    UnpackedTypeVarTuple(UnpackedTypeVarTupleType),
    ParamSpecArgs(ParamSpecArgsType),
    ParamSpecKwargs(ParamSpecKwargsType),
    Callable(CallableType),
    Tuple(TupleType),
    Selff(SelfType),
    Literal(LiteralType),
    TypeGuard(TypeGuardType),
    TypeIs(TypeIsType),
    NamedTuple(NamedTupleType),
    TypedDict(TypedDictType),
    Any,
    LiteralString,
}

enum TypeTypeKind {
    Class(ClassDef),
    Union(UnionType),
    TypeVar(TypeVarType),
    Selff(SelfType),
    NamedTuple(NamedTupleType),
    Any,
    // Pyright accepts `type[LiteralString]` and `type[Literal["foo"]]`
    // in annotations but these don't make any sense to me?
    // (Mypy doesn't support LiteralString yet.)
    // Both pyright and mypy accept `type[tuple[str, str]]`,
    // but that also doesn't make any sense to me.
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
struct TypeGuardType {}
struct TypeIsType {}
struct NamedTupleType {}
struct TypedDictType {}

struct UnionType {
    members: HashSet<Type>,
}
