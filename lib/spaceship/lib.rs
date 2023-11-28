use std::slice::Iter;
use uuid::Uuid;
use crate::FailureReason::*;
use crate::part::{CreatePartType, Part, PartType};
use crate::Purchase::{Failure, Success};

pub mod part;

// ---- Purchase ---- //
#[derive(Debug)]
pub enum Purchase<'s> {
    Success(),
    Failure(SpaceshipBuilder<'s>, FailureReason)
}

#[derive(Debug)]
pub enum FailureReason {
    TooLittleFunds,
    MissingParts(Vec<CreatePartType>),
    ElectricityDeficit,
    NotEnoughProtection
}

// ---- Spaceship  ---- //
#[derive(Debug)]
pub struct Spaceship {
    name: String,
    parts: Vec<Part>,
    id: Uuid,
    iteration: u16,
}
impl Spaceship {
    pub fn new(name: String) -> Self {
        Spaceship {
            name,
            parts: vec![],
            id: Uuid::new_v4(),
            iteration: 0,
        }
    }
    
    pub fn size(&self) -> u16 {
        self.parts.iter().fold(0, |acc, part| acc + part.size())
    }
}

// ---- Builder  ---- //

const BASE_COST: f32 = 1000.0;

#[derive(Debug)]
pub struct SpaceshipBuilder<'s> {
    ship: &'s mut Spaceship,
    costs: f32,
    iteration: u16,
    parts: Vec<Part>,
}

impl <'s> SpaceshipBuilder <'s> {
    pub fn costs(&self) -> f32 { self.costs }
    
    pub fn new(ship: &'s mut Spaceship) -> SpaceshipBuilder {
        ship.iteration += 1;
        let iteration = ship.iteration;
        Self {
            ship,
            costs: BASE_COST * (1. + 0.01 * iteration as f32),
            iteration,
            parts: vec![],
        }
    }

    pub fn add_part(&mut self, part_type: CreatePartType, size: u16, level: u16) {
        self.insert_part(Part::new(part_type, size, level, Uuid::new_v4()))
    }
    
    pub fn add_part_cost(&self, part_type: CreatePartType, size: u16, level: u16) -> f32 {
        self.insert_part_cost(&Part::new(part_type, size, level, Uuid::nil()))
    }
    
    pub fn insert_part(&mut self, part: Part) {
        self.costs += self.insert_part_cost(&part);
        self.parts.push(part)
    }
    pub fn insert_part_cost(&self, part: &Part) -> f32 {
        part.cost() + 0.01 * self.size() as f32
    }

    pub fn size(&self) -> u16 {
        self.ship.size() + self.parts.iter().fold(0, |acc, part| acc + part.size())
    }
    
    pub fn pop_part(&mut self) -> Option<Part> {
        let part = self.parts.pop()?;
        self.costs -= self.insert_part_cost(&part);
        Some(part)
    }
    
    pub fn remove_part(&mut self, id: Uuid) -> Option<Part> {
        let part = self.parts.remove(self.parts.iter().position(|p| p.id() == id)?);
        self.costs -= self.insert_part_cost(&part);
        Some(part)
    }
    
    pub fn parts_iter(&mut self) -> Iter<'_, Part> {
        self.parts.iter()
    }
    
    pub fn complete(mut self, funds: &mut f32) -> Purchase<'s> {
        if *funds > self.costs { 
            *funds -= self.costs;
            
            while let Some(part) = self.parts.pop() {
                self.ship.parts.push(part);
            }
            
            Success()
        }
        else { Failure(self, TooLittleFunds) }
    }
}
