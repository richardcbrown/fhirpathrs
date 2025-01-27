use super::ResourceNode;

pub enum Arity {
    AnyAtRoot,
    Expr,
}

pub fn get_input_for_arity<'a>(input: &'a ResourceNode<'a>, arity: Arity) -> ResourceNode<'a> {
    match arity {
        Arity::AnyAtRoot => ResourceNode::from_node(input, input.data_root.clone()),
        _ => input.clone(),
    }
}
