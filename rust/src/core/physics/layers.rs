// Physics layer system for collision filtering

/// Physics layer constants (0-31)
pub struct PhysicsLayers;

impl PhysicsLayers {
    pub const DEFAULT: u32 = 0;
    pub const PLAYER: u32 = 1;
    pub const ENEMY: u32 = 2;
    pub const PROJECTILE: u32 = 3;
    pub const ENVIRONMENT: u32 = 4;
    pub const TRIGGER: u32 = 5;
    pub const UI: u32 = 6;
    pub const PICKUP: u32 = 7;
}

/// Create a collision mask from a list of layers
pub fn create_mask(layers: &[u32]) -> u32 {
    let mut mask = 0u32;
    for &layer in layers {
        if layer < 32 {
            mask |= 1 << layer;
        }
    }
    mask
}

/// Create a mask that collides with all layers
pub fn all() -> u32 {
    0xFFFFFFFF
}

/// Create a mask that collides with no layers
pub fn none() -> u32 {
    0
}

/// Check if two layers should collide based on their masks
pub fn should_collide(layer_a: u32, mask_a: u32, layer_b: u32, mask_b: u32) -> bool {
    let layer_bit_a = 1 << layer_a;
    let layer_bit_b = 1 << layer_b;

    // Check if layer_a is in mask_b AND layer_b is in mask_a
    (mask_b & layer_bit_a) != 0 && (mask_a & layer_bit_b) != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_mask() {
        let mask = create_mask(&[0, 2, 4]);
        assert_eq!(mask, 0b10101);
    }

    #[test]
    fn test_should_collide() {
        let player_layer = PhysicsLayers::PLAYER;
        let enemy_layer = PhysicsLayers::ENEMY;

        // Player collides with enemies
        let player_mask = create_mask(&[PhysicsLayers::ENEMY, PhysicsLayers::ENVIRONMENT]);
        // Enemy collides with players
        let enemy_mask = create_mask(&[PhysicsLayers::PLAYER, PhysicsLayers::PROJECTILE]);

        assert!(should_collide(player_layer, player_mask, enemy_layer, enemy_mask));
    }

    #[test]
    fn test_should_not_collide() {
        let player_layer = PhysicsLayers::PLAYER;
        let pickup_layer = PhysicsLayers::PICKUP;

        // Player doesn't collide with pickups
        let player_mask = create_mask(&[PhysicsLayers::ENEMY]);
        // Pickup doesn't collide with players
        let pickup_mask = create_mask(&[PhysicsLayers::ENVIRONMENT]);

        assert!(!should_collide(player_layer, player_mask, pickup_layer, pickup_mask));
    }
}
