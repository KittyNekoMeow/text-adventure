#[derive(Debug, Clone, Copy, PartialEq)]
/// Different types of magic
pub enum MagicType {
    Fire,
    Ice,
    Acid,
    Arcane,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Different types of weapons
pub enum WeaponType {
    Sharp,
    Blunt,
    Magic(MagicType)
}