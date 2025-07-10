use super::ResourceNode;

pub enum Target {
    AnyAtRoot,
    Expr,
}

pub fn get_input_for_target<'a, 'b>(input: &'a ResourceNode<'a, 'b>, target: Target) -> ResourceNode<'a, 'b> {
    match target {
        Target::AnyAtRoot => ResourceNode::from_node(input, input.data_root.clone()),
        _ => input.clone(),
    }
}
