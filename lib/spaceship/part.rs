use uuid::Uuid;


#[derive(Debug)]
pub struct Part {
    id: Uuid,
    part_type: PartType,
    size: u16,
    level: u16,
    
    health: f32,
    cost: f32,
    electricity: f32,
}

impl Part {
    pub fn id(&self) -> Uuid { self.id }
    pub fn part_type(&self) -> &PartType { &self.part_type }
    pub fn size(&self) -> u16 { self.size }
    pub fn level(&self) -> u16 { self.level }
    pub fn health(&self) -> f32 { self.health }
    pub fn cost(&self) -> f32 { self.cost }
    pub fn electricity(&self) -> f32 { self.electricity }
    
    pub(crate) fn new(part_type: CreatePartType, size: u16, level: u16, id: Uuid) -> Self {
        let fsize = size as f32;
        let flevel = level as f32;
        
        match part_type {
            CreatePartType::Hull => Part {
                part_type: PartType::Hull(Hull {
                    protection: fsize * 0.6 * flevel * 0.4,
                }),
                size, level, id,
                health: 0.4 * fsize * flevel,
                electricity: -0.1 * fsize * flevel,
                cost: 100.0 + 0.02 * fsize + 50.0 * flevel,
            },
            CreatePartType::Cargo => Part {
                part_type: PartType::Cargo(Cargo {
                    capacity: (size + 10 * level)
                }),
                size, level, id,
                health: 0.1 * fsize * flevel,
                electricity: -0.2 * fsize * flevel,
                cost: 50.0 + 0.08 * fsize + 100.0 * flevel,
            },
            CreatePartType::Cockpit => Part {
                part_type: PartType::Cockpit(()),
                size, level, id,
                health: 0.8 * fsize * flevel,
                electricity: -0.8 * fsize * flevel,
                cost: 100.0 + 0.1 * fsize + 100.0 * flevel,
            },
            CreatePartType::SolarPanels => Part {
                part_type: PartType::SolarPanels(()),
                size, level, id,
                health: 0.8 * fsize * flevel,
                electricity: 1.2 * fsize * flevel,
                cost: 100.0 + 0.1 * fsize + 100.0 * flevel,
            },
            CreatePartType::LivingQuarters => Part {
                part_type: PartType::LivingQuarters(LivingQuarters {
                    capacity: (size + 10 * level)
                }),
                size, level, id,
                health: 0.8 * fsize * flevel,
                electricity: -0.8 * fsize * flevel,
                cost: 100.0 + 0.1 * fsize + 100.0 * flevel,
            },
        }
    }
}


#[derive(Debug)]
pub enum CreatePartType {
    Hull,
    Cargo,
    Cockpit,
    SolarPanels,
    LivingQuarters,
}

#[derive(Debug)]
pub struct Hull {
    protection: f32
}
#[derive(Debug)]
pub struct Cargo {
    capacity: u16
}
pub type Cockpit = ();
pub type SolarPanels = ();
#[derive(Debug)]
pub struct LivingQuarters {
    capacity: u16
}


#[derive(Debug)]
pub enum PartType {
    Hull(Hull),
    Cargo(Cargo),
    Cockpit(Cockpit),
    SolarPanels(SolarPanels),
    LivingQuarters(LivingQuarters),
}

impl PartType {
    pub fn hull(self) -> Option<Hull> {
        match self { PartType::Hull(t) => Some(t), _ => None }
    }
    pub fn cargo(self) -> Option<Cargo> {
        match self { PartType::Cargo(t) => Some(t), _ => None }
    }
    pub fn cargo_ref(&self) -> Option<&Cargo> {
        match self { PartType::Cargo(t) => Some(t), _ => None }
    }
    pub fn is_cargo(&self) -> bool {
        match self { PartType::Cargo(_) => true, _ => false }
    }
}