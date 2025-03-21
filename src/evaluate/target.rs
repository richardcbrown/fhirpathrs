use super::ResourceNode;

pub enum Target {
    AnyAtRoot,
    Expr,
}

pub fn get_input_for_target<'a>(input: &'a ResourceNode<'a>, target: Target) -> ResourceNode<'a> {
    match target {
        Target::AnyAtRoot => ResourceNode::from_node(input, input.data_root.clone()),
        _ => input.clone(),
    }
}
