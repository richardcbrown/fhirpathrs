use super::ResourceNode;

pub enum Arity {
    AnyAtRoot,
    Expr,
}

pub fn get_input_for_arity<'a>(input: &'a ResourceNode<'a>, arity: Arity) -> ResourceNode<'a> {
    match arity {
        Arity::AnyAtRoot => ResourceNode {
            data_root: input.data_root.clone(),
            data: Some(input.data_root.clone()),
            parent_node: None,
        },
        _ => input.clone(),
    }
}
