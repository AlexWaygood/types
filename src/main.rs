use std::collections::HashSet;

enum ValidType {
    FunctionParameter(FunctionParameterType),
    FunctionReturn(FunctionReturnType),
    MethodParameter(MethodParameterType),
    MethodReturn(MethodReturnType),
    NonClassScopedVariable(NonClassScopedVariableType),
    ClassScopedVariable(ClassScopedVariableType),
    TypedDictScopedVariable(TypedDictScopedVariableType),
}

enum FunctionParameterType {
    Never,
    Instance(FunctionParameterInstanceTypeKind),
    Type(FunctionParameterTypeTypeKind),
}

enum FunctionParameterInstanceTypeKind {
    Class(ClassDef),
    Union(UnionType<NonClassScopedVariableKind>),
    TypeVar(TypeVarType),
    // e.g. `args` in `def f[*Ts](*args: *Ts): ...`
    UnpackedTypeVarTuple(UnpackedTypeVarTupleType),
    ParamSpecArgs(ParamSpecArgsType),
    ParamSpecKwargs(ParamSpecKwargsType),
    Callable(CallableType),
    Tuple(TupleType),
    Literal(LiteralType),
    NamedTuple(NamedTupleType),
    TypedDict(TypedDictType),
    Any,
    LiteralString,
}

enum FunctionParameterTypeTypeKind {
    Class(ClassDef),
    Union(UnionType<NonClassScopedVariableKind>),
    TypeVar(TypeVarType),
    NamedTuple(NamedTupleType),
    Any,
}

enum FunctionReturnType {
    Never,
    Instance(FunctionReturnInstanceTypeKind),
    Type(FunctionReturnTypeTypeKind),
    SpecialForm(FunctionReturnSpecialForm)
}

enum FunctionReturnUnionTypeKind {
    Instance(FunctionReturnInstanceTypeKind),
    Type(FunctionReturnTypeTypeKind),
}

enum FunctionReturnInstanceTypeKind {
    Class(ClassDef),
    Union(UnionType<FunctionReturnUnionTypeKind>),
    TypeVar(TypeVarType),
    Callable(CallableType),
    Tuple(TupleType),
    Literal(LiteralType),
    NamedTuple(NamedTupleType),
    TypedDict(TypedDictType),
    Any,
    LiteralString,
}

enum FunctionReturnSpecialForm {
    TypeIs(TypeIsType),
    TypeGuard(TypeGuardType),
}

enum FunctionReturnTypeTypeKind {
    Class(ClassDef),
    Union(UnionType<FunctionReturnUnionTypeKind>),
    TypeVar(TypeVarType),
    NamedTuple(NamedTupleType),
    Any,
}

enum MethodParameterType {
    Never,
    Instance(MethodParameterInstanceTypeKind),
    Type(MethodParameterTypeTypeKind),
}

enum MethodParameterInstanceTypeKind {
    Selff(SelfType),
    FunctionParameter(FunctionParameterInstanceTypeKind)
}

enum MethodParameterTypeTypeKind {
    Selff(SelfType),
    FunctionParameter(FunctionParameterTypeTypeKind),
}

enum MethodReturnType {
    Never,
    Instance(MethodReturnInstanceTypeKind),
    Type(MethodReturnTypeTypeKind),
}

enum MethodReturnInstanceTypeKind {
    Selff(SelfType),
    FunctionParameter(FunctionReturnInstanceTypeKind),
}

enum MethodReturnTypeTypeKind {
    Selff(SelfType),
    FunctionReturn(FunctionReturnTypeTypeKind),
}

struct NonClassScopedVariableType {
    final_qualifier: bool,
    kind: NonClassScopedVariableKind
}

enum NonClassScopedVariableKind {
    Never,
    Instance(NonClassScopedInstanceTypeKind),
    Type(NonClassScopedTypeTypeKind),
}

enum NonClassScopedInstanceTypeKind<T = NonClassScopedVariableKind> {
    Class(ClassDef),
    Union(UnionType<T>),
    Callable(CallableType),
    Tuple(TupleType),
    Literal(LiteralType),
    NamedTuple(NamedTupleType),
    TypedDict(TypedDictType),
    Any,
    LiteralString,
}

enum NonClassScopedTypeTypeKind<T = NonClassScopedVariableKind> {
    Class(ClassDef),
    Union(UnionType<T>),
    NamedTuple(NamedTupleType),
    Any,
}

struct ClassScopedVariableType {
    qualifier: ClassScopedVariableQualifier,
    kind: ClassScopedVariableKind
}

enum ClassScopedVariableKind {
    Never,
    Instance(ClassScopedInstanceTypeKind),
    Type(ClassScopedTypeTypeKind),
}

enum ClassScopedInstanceTypeKind {
    NonClassScopeSpecific(NonClassScopedInstanceTypeKind<ClassScopedInstanceTypeKind>),
    Selff(SelfType),
}

enum ClassScopedTypeTypeKind {
    NonClassScopeSpecific(NonClassScopedTypeTypeKind<ClassScopedTypeTypeKind>),
    Selff(SelfType),
}

enum ClassScopedVariableQualifier {
    None,
    Final,
    ClassVar,
    FinalClassVar,  // Only valid if it's a dataclass? (Carl knows best here ;)
}

struct TypedDictScopedVariableType {
    required: bool,
    kind: NonClassScopedVariableKind,
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
