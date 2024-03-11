use std::{fmt::Display, cmp::min, cmp::max};

use rand::{thread_rng, seq::SliceRandom};

pub struct Game {
    glasses: Vec<Glass>,
    selected: Option<u8>,
}

impl Game {
    pub fn new(glasses: Vec<Glass>) -> Self {
        Self {
            glasses,
            selected: None,
        }
    }

    pub fn create(glass_size: u8, glass_count: u8) -> Self {
        let colors = Ball::get_colors();
        let mut balls = vec![];
        
        let glass_size = max(4, glass_size);

        for color in &colors[0..min(glass_count - 2, colors.len() as u8) as usize] {
            for _ in 0..glass_size {
                balls.push(*color);
            }
        }
        
        let mut rng = thread_rng();
        balls.shuffle(&mut rng);

        let mut glasses = vec![];

        for chunk in balls.chunks(glass_size as usize) {
            glasses.push(Glass {
                size: glass_size,
                balls: chunk.to_vec(),
            })
        }

        glasses.push(Glass { size: glass_size, balls: vec![] });
        glasses.push(Glass { size: glass_size, balls: vec![] });

        return Self::new(glasses);
    }
    
    pub fn is_swap_possible(&self, target: u8) -> Result<(), String> {
        if self.selected.is_none() {
            return Err("No Glass selected".to_owned());
        }

        if self.selected.unwrap() == target {
            return Err("Cannot swap into same glass".to_owned());
        }

        let selected = self.glasses.get(self.selected.unwrap() as usize);
        let target = self.glasses.get(target as usize);

        if target.is_none() {
            return Err("Target glass does not exist".to_owned());
        }

        let target = target.unwrap();
        let selected = selected.unwrap();

        if selected.is_empty() {
            return Err("Selected Glass is empty".to_owned());
        }

        if target.is_full() {
            return Err("Target Glass is full".to_owned());
        }

        if target.get_top().is_some() && selected.get_top().unwrap() != target.get_top().unwrap() {
            return Err("Mismatched balls in glasses".to_owned());
        }

        return Ok(());
    }

    pub fn select_glass(&mut self, target: u8) {
        if self.selected.is_some() && self.selected.unwrap() == target {
            self.selected = None;
            
            return;
        }

        self.selected = Some(target);
    }

    pub fn swap_glasses(&mut self, target: u8) {
        if self.is_swap_possible(target).is_err() {
            return;
        }

        let selected = self.glasses.get(self.selected.unwrap() as usize).unwrap();
        
        let swapable_balls = selected.balls.iter().rev().map_while(|ball| {
            if ball == selected.balls.last().unwrap() {
                return Some(());
            } else {
                return None;
            }
        }).count();

        let mut balls: Vec<Ball> = vec![];
        {
            let selected = self.glasses.get_mut(self.selected.unwrap() as usize).unwrap();
            for _ in 0..swapable_balls {
                balls.push(selected.pop().unwrap());
            }
        }

        let target = self.glasses.get_mut(target as usize).unwrap();

        for ball in balls {
            if target.is_full() {
                break;
            }
            
            target.push(ball).unwrap();
        }
    }

    pub fn is_completed(&self) -> bool {
        return self.glasses.iter().all(|glass| glass.is_completed());
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for glass in &self.glasses {
            write!(f, "{}\n", glass)?;
        };

        Ok(())
    }
}

#[derive(Clone)]
pub struct Glass {
    size: u8,
    balls: Vec<Ball>,
}
impl Glass {
    pub fn new(size: u8, balls: Vec<Ball>) -> Self {
        Self { size, balls }
    }

    pub fn is_full(&self) -> bool {
        return self.balls.len() == self.size.into();
    }

    pub fn is_empty(&self) -> bool {
        return self.balls.is_empty();
    }

    pub fn get_top(&self) -> Option<&Ball> {
       return self.balls.last();
    }

    pub fn push(&mut self, ball: Ball) -> Result<(), String> {
        if !self.is_full() {
            println!("{}", self.balls.len());
            println!("{}", self.is_full());

            self.balls.push(ball);

            return Ok(());
        } else {
            return Err("Glass is full".to_owned());
        }

    }

    pub fn pop(&mut self) -> Result<Ball, String> {
        if let Some(ball) = self.balls.pop() {
            return Ok(ball);
        } else {
            return Err("Glass is empty".to_owned());
        }

    }

    pub fn get_available_space(&self) -> u8 {
        return self.size - self.balls.len() as u8;
    }

    pub fn is_completed(&self) -> bool {
        return self.is_full() && self.balls.iter().all(|ball| ball == self.balls.last().unwrap());
    }
}

impl Display for Glass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..self.size).rev() {
            match self.balls.get(i as usize) {
                Some(ball) => write!(f, "|{}|", ball)?,
                None => write!(f, "| |")?,
            };

            write!(f, "\n")?;
        }

        write!(f, "\\_/\n")?;

        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Ball {
    RED,
    GREEN,
    BLUE,
    YELLOW,
    MAGENTA,
    CYAN,
    PINK,
    ORANGE,
    SHITTYCOLORIDK,
}

impl Ball {
    pub fn get_colors() -> [Ball; 9] {
        return [
            Self::RED,
            Self::GREEN,
            Self::BLUE,
            Self::YELLOW,
            Self::MAGENTA,
            Self::CYAN,
            Self::PINK,
            Self::ORANGE,
            Self::SHITTYCOLORIDK,
        ]
    }
}

impl Display for Ball {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Ball::RED => "R",
            Ball::GREEN => "G",
            Ball::BLUE => "B",
            Ball::YELLOW => "Y",
            Ball::MAGENTA => "M",
            Ball::CYAN => "C",
            Ball::PINK => "P",
            Ball::ORANGE => "O",
            Ball::SHITTYCOLORIDK => "S",
        };

        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn glass_empty() {
        let glass = Glass { size: 4, balls: vec![] };

        assert_eq!(0, glass.balls.len());
        assert_eq!(true, glass.is_empty());
        assert_eq!(false, glass.is_full());
    }

    #[test]
    fn glass_full() {
        let glass = Glass { size: 4, balls: vec![Ball::RED, Ball::RED, Ball::RED, Ball::RED] };

        assert_eq!(4, glass.balls.len());
        assert_eq!(false, glass.is_empty());
        assert_eq!(true, glass.is_full());
    }

    #[test]
    fn glass_push() {
        let mut glass_full = Glass { size: 4, balls: vec![Ball::RED, Ball::RED, Ball::RED, Ball::RED] };
        let mut glass_empty = Glass { size: 4, balls: vec![] };

        assert!(glass_full.push(Ball::RED).is_err());
        assert!(glass_empty.push(Ball::RED).is_ok());
    }

    #[test]
    fn glass_pop() {
        let mut glass_full = Glass { size: 4, balls: vec![Ball::RED, Ball::RED, Ball::RED, Ball::RED] };
        let mut glass_empty = Glass { size: 4, balls: vec![] };

        assert!(glass_full.pop().is_ok());
        assert!(glass_empty.pop().is_err());
    }
    
    #[test]
    fn glass_top() {
        let glass_full = Glass { size: 4, balls: vec![Ball::RED, Ball::RED, Ball::RED, Ball::RED] };
        let glass_empty = Glass { size: 4, balls: vec![] };

        assert!(glass_empty.get_top().is_none());

        assert!(glass_full.get_top().is_some());
        assert!(*glass_full.get_top().unwrap() == Ball::RED);
    }

    #[test]
    fn glass_completed() {
        let glass_full = Glass { size: 4, balls: vec![Ball::RED, Ball::RED, Ball::RED, Ball::RED] };
        let glass_empty = Glass { size: 4, balls: vec![] };
        let glass_full_2 = Glass { size: 4, balls: vec![Ball::RED, Ball::RED, Ball::RED, Ball::GREEN] };

        assert!(glass_full.is_completed());
        assert!(!glass_empty.is_completed());
        assert!(!glass_full_2.is_completed());
    }

    #[test]
    fn swap_full_to_empty() {
        let glass_full = Glass { size: 4, balls: vec![Ball::RED, Ball::RED, Ball::RED, Ball::RED] };
        let glass_empty = Glass { size: 4, balls: vec![] };

        let mut game = Game { glasses: vec![glass_full, glass_empty], selected: None };

        game.select_glass(0);

        game.swap_glasses(1);

        assert!(game.glasses.get(0).unwrap().is_empty());
        assert!(game.glasses.get(1).unwrap().is_full());
    }

    #[test]
    fn swap_empty_to_full() {
        let glass_full = Glass { size: 4, balls: vec![Ball::RED, Ball::RED, Ball::RED, Ball::RED] };
        let glass_empty = Glass { size: 4, balls: vec![] };

        let mut game = Game { glasses: vec![glass_full, glass_empty], selected: None };

        game.select_glass(1);

        game.swap_glasses(0);

        assert!(game.glasses.get(1).unwrap().is_empty());
        assert!(game.glasses.get(0).unwrap().is_full());
    }

    #[test]
    fn swap_same_glass() {
        let glass = Glass { size: 4, balls: vec![Ball::RED, Ball::RED, Ball::RED, Ball::RED] };

        let mut game = Game { glasses: vec![glass], selected: None };

        game.select_glass(0);

        game.swap_glasses(0);

        assert!(game.glasses.get(0).unwrap().is_full());
    }
}
