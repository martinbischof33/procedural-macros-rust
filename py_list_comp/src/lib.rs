// comp: mapping for_if_clause
// mapping: expression
// for_if_clause:
//     | for' pattern 'in' expression ('if' expression)*
// pattern: name (, name) *
//

struct Comp {
    mapping: Mapping,
    for_if_clause: ForIfClause,
}

#[derive(Debug)]
struct Mapping(syn::Expr);

struct ForIfClause {}
