use bon::Builder;
use derive_more::{Display, From};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct Name(String);

impl Name {
    pub fn new(raw: &str) -> Result<Self, GameNameEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(GameNameEmptyError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

#[derive(Clone, Debug, Error)]
#[error("game name cannot be empty")]
pub struct GameNameEmptyError;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct Platform(String);

impl Platform {
    pub fn new(raw: &str) -> Result<Self, GamePlatformEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(GamePlatformEmptyError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

#[derive(Clone, Debug, Error)]
#[error("game platform cannot be empty")]
pub struct GamePlatformEmptyError;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Display)]
pub struct Image(String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Display)]
pub struct Tooltip(String);

#[expect(dead_code)]
pub struct Game {
    name: Name,
    platform: Platform,
    large_image: Option<Image>,
    large_tooltip: Option<Tooltip>,
    small_image: Option<Image>,
    small_tooltip: Option<Tooltip>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Builder)]
pub struct GameCreateRequest {
    name: Name,
    platform: Platform,
    large_image: Option<Image>,
    large_tooltip: Option<Tooltip>,
    small_image: Option<Image>,
    small_tooltip: Option<Tooltip>,
}

impl GameCreateRequest {
    pub fn name(&self) -> &Name {
        &self.name
    }
    
    pub fn platform(&self) -> &Platform {
        &self.platform
    }
    
    pub fn large_image(&self) -> Option<&Image> {
        self.large_image.as_ref()
    }
    
    pub fn large_tooltip(&self) -> Option<&Tooltip> {
        self.large_tooltip.as_ref()
    }
    
    pub fn small_image(&self) -> Option<&Image> {
        self.small_image.as_ref()
    }
    
    pub fn small_tooltip(&self) -> Option<&Tooltip> {
        self.small_tooltip.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct GameDeleteRequest {
    name: Name,
}

impl GameDeleteRequest {
    pub fn name(&self) -> &Name {
        &self.name
    }
}

/// `GameRepository` represents a store of `Game` data.
pub trait GameRepository {
    fn create_game(&self, req: &GameCreateRequest) -> Result<Game, GameCreateError>;
    fn delete_game(&self, req: &GameDeleteRequest) -> Result<(), GameDeleteError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum GameCreateError {
    #[error("game with name {game_name} already exists")]
    Duplicate { game_name: Name },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum GameDeleteError {
    #[error("game with name {game_name} does not exist")]
    NotFound { game_name: Name },
}

pub trait GameService {
    fn create_game(&self, req: &GameCreateRequest) -> Result<Game, GameCreateError>;
    fn delete_game(&self, req: &GameDeleteRequest) -> Result<(), GameDeleteError>;
}