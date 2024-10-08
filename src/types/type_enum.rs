use crate::types::{Node, ConveyorBelt, Miner, Machine};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Node(Node),
    ConveyorBelt(ConveyorBelt),
    Miner(Miner),
    Machine(Machine),
    Nothing,
}

impl Type {
    pub fn get_type(&self) -> &str {
        match self {
            Type::Miner(miner) => &miner.name,
            Type::Node(node) => &node.name,
            Type::Machine(machine) => &machine.name,
            Type::Node(node) => &node.name,
            Type::ConveyorBelt(_) => "Conveyor Belt",
            Type::Nothing => "Nothing",
        }
    }
}
