use crate::{ast::node::dec::Dec, types::Type, types::*};
use paste::paste;

macro_rules! gen_visit {
    ($node: ident, $type: ident, $($mutability:ident)?) => {
        paste! {
            fn [<visit_ $node _type>](&mut self, declared: &$($mutability)? Option<Box<Dec>>, node: &$($mutability)? $type) -> Option<T> {
                let _ = node;
                let _ = declared;
                None
            }
        }
    };
}

macro_rules! gen_match {
    ($self: ident, $declared: ident, $type:ident, $ltype:ident, $($node: ident, $lnode:ident),+) => {

        paste! {
        match $ltype {
            $(
                $type::$node { .. } => $self.[<visit_ $lnode _type>]($declared, $ltype),
            )+
        }
        }
    };
}

macro_rules! gen_visitor {
    ($visitor_trait_name:ident, $($mutability:ident)? ) => {
        trait $visitor_trait_name<T> {
            fn visit(&mut self, node: &$($mutability)? Type) -> Option<T> {
                match node {
                    Type::Basic { declared, kind } => self.visit_basic(declared, kind),
                    Type::Derived { declared, kind } => self.visit_derived(declared, kind),
                    Type::ComplexType { declared, kind } => self.visit_complex(declared, kind),
                    Type::Unresolved { name } => self.visit_unresolved(name),
                    Type::Error => self.visit_error(),
                }
            }

            fn visit_basic(&mut self, declared: &$($mutability)? Option<Box<Dec>>, kind: &$($mutability)? BasicType) -> Option<T> {
                gen_match!(
                    self, declared, BasicType, kind, Void, void, Byte, byte, Sint, sint, Usint,
                    usint, Int, int, Uint, uint, Float, float, Lint, lint, Ulint, ulint, Chan,
                    chan, Poly, poly
                );
                None
            }

            gen_visit!(void, BasicType,$($mutability)?);
            gen_visit!(byte, BasicType,$($mutability)?);
            gen_visit!(sint, BasicType,$($mutability)?);
            gen_visit!(usint, BasicType,$($mutability)?);
            gen_visit!(int, BasicType,$($mutability)?);
            gen_visit!(uint, BasicType,$($mutability)?);
            gen_visit!(float, BasicType,$($mutability)?);
            gen_visit!(lint, BasicType,$($mutability)?);
            gen_visit!(ulint, BasicType,$($mutability)?);
            gen_visit!(chan, BasicType,$($mutability)?);
            gen_visit!(poly, BasicType,$($mutability)?);

            fn visit_derived(
                &mut self,
                declared: &$($mutability)? Option<Box<Dec>>,
                kind: &$($mutability)? DerivedType,
            ) -> Option<T> {
                match kind {
                    DerivedType::Pointer(_) => self.visit_pointer_type(declared, kind),
                    DerivedType::Function(_) => self.visit_function_type(declared, kind),
                    DerivedType::Method(_) => self.visit_method_type(declared, kind),
                    DerivedType::Array(_) => self.visit_array_type(declared, kind),
                }
            }

            gen_visit!(pointer, DerivedType,$($mutability)?);
            gen_visit!(function, DerivedType,$($mutability)?);
            gen_visit!(method, DerivedType,$($mutability)?);
            gen_visit!(array, DerivedType,$($mutability)?);

            fn visit_complex(
                &mut self,
                declared: &$($mutability)? Option<Box<Dec>>,
                kind: &$($mutability)? ComplexType,
            ) -> Option<T> {
                match kind {
                    ComplexType::Enum(_) => self.visit_enum_type(declared, kind),
                    ComplexType::Aggr(_) => self.visit_aggr_type(declared, kind),
                    ComplexType::Adt(_) => self.visit_adt_type(declared, kind),
                    ComplexType::Union(_) => self.visit_union_type(declared, kind),
                    ComplexType::Tuple(_) => self.visit_tuple_type(declared, kind),
                }
            }

            gen_visit!(enum, ComplexType,$($mutability)?);
            gen_visit!(aggr, ComplexType,$($mutability)?);
            gen_visit!(adt, ComplexType,$($mutability)?);
            gen_visit!(union, ComplexType,$($mutability)?);
            gen_visit!(tuple, ComplexType,$($mutability)?);

            fn visit_unresolved(&mut self, _name: &$($mutability)? String) -> Option<T> {
                None
            }

            fn visit_error(&mut self) -> Option<T> {
                None
            }
        }
    };
}

gen_visitor!(ImmutableTypeVisitor,);
gen_visitor!(MutableTypeVisitor, mut);
