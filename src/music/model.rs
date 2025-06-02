use crate::music::model::Step::{Half, Whole};
use Note::*;

#[derive(Clone, Copy, Debug)]
pub enum Step {
    Half,
    Whole,
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum Note {
    A,
    As,
    B,
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
}

impl Note {
    fn next(&self, step: &Step) -> Note {
        match step {
            Half => match self {
                A => As,
                As => B,
                B => C,
                C => Cs,
                Cs => D,
                D => Ds,
                Ds => E,
                E => F,
                F => Fs,
                Fs => G,
                G => Gs,
                Gs => A,
            },
            Whole => match self {
                A => B,
                As => C,
                B => Cs,
                C => D,
                Cs => Ds,
                D => E,
                Ds => F,
                E => Fs,
                F => G,
                Fs => Gs,
                G => A,
                Gs => As,
            },
        }
    }
}

pub trait Scale: Send + Sync + 'static {
    fn size(&self) -> u8 {
        self.steps().len() as u8
    }
    fn steps(&self) -> Vec<Step>;

    fn get(&self, index: u8) -> Note {
        let index = index % self.size();
        let mut result = self.root().clone();

        let steps = &self.steps()[0..index as usize];

        for step in steps.iter() {
            result = result.next(step);
        }

        result
    }

    fn root(&self) -> &Note;
}
pub struct NaturalMinorScale {
    root: Note,
}

impl NaturalMinorScale {
    pub fn new(root: Note) -> NaturalMinorScale {
        NaturalMinorScale { root }
    }
}

impl Scale for NaturalMinorScale {
    fn steps(&self) -> Vec<Step> {
        vec![Whole, Half, Whole, Whole, Half, Whole, Whole]
    }

    fn root(&self) -> &Note {
        &self.root
    }
}
