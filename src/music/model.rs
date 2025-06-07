use crate::music::model::Step::{Half, Whole};
use Note::*;

#[derive(Clone, Copy, Debug)]
pub enum Step {
    Half,
    Whole,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
        self.steps().len() as u8 + 1
    }
    fn steps(&self) -> Vec<Step>;

    /// Return the note in the scale given by the index. The index is 1-based, since the 0 is not
    /// easy to hit with manually set notes.
    ///
    /// The index is also calculated with its mod, so higher indexes are possible.
    ///
    /// # Example
    ///
    /// |note by index|||||||||
    /// |-|-|-|-|-|-|-|-|-|
    /// |A minor scale| A| B| C| D| E| F| G| A|
    /// |index| 0/1| 2| 3| 4| 5| 6| 7| 8/0|
    fn get(&self, index: u8) -> Note {
        let index = index.saturating_sub(1) % self.size();
        let mut result = *self.root();

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
        // last step does not go to root again!
        vec![Whole, Half, Whole, Whole, Half, Whole]
    }

    fn root(&self) -> &Note {
        &self.root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let scale = NaturalMinorScale::new(A);
        assert_eq!(A, scale.get(0));
        assert_eq!(A, scale.get(1));
        assert_eq!(B, scale.get(2));
        assert_eq!(C, scale.get(3));
        assert_eq!(D, scale.get(4));
        assert_eq!(E, scale.get(5));
        assert_eq!(F, scale.get(6));
        assert_eq!(G, scale.get(7));
        assert_eq!(A, scale.get(8));
        assert_eq!(B, scale.get(9));
        assert_eq!(C, scale.get(10));
        assert_eq!(D, scale.get(11));
        assert_eq!(E, scale.get(12));
        assert_eq!(F, scale.get(13));
        assert_eq!(G, scale.get(14));
        assert_eq!(A, scale.get(15));
    }
}
