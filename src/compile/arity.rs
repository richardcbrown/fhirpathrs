use super::ResourceNode;

pub enum Arity {
    AnyAtRoot,
    Expr,
}

pub fn get_input_for_arity<'a>(input: &'a ResourceNode<'a>, arity: Arity) -> ResourceNode<'a> {
    match arity {
        Arity::AnyAtRoot => {
            ResourceNode::new(input.data_root.clone(), None, input.data_root.clone())
        }
        _ => input.clone(),
    }
}
