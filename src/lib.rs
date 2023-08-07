use eyre::{eyre, Report};
use itertools::ProcessResults;

use std::env;
use std::fs;
use std::iter::Sum;
use std::path::{Path, PathBuf};

pub type Result<T> = eyre::Result<T>;

pub fn setup(input_file: &str) -> Result<Vec<u8>> {
    env::set_var("RUST_BACKTRACE", "full");
    color_eyre::install().unwrap_or_default();

    let mut path = PathBuf::from("inputs").join(Path::new(input_file).file_stem().value()?);
    path.set_extension("txt");
    Ok(fs::read(path)?)
}

pub trait OptionExt<T> {
    fn value(self) -> Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn value(self) -> Result<T> {
        self.ok_or_else(|| eyre!("Error: no value"))
    }
}

pub trait IteratorExt: Iterator {
    fn try_process<F, T, R>(self, processor: F) -> Result<R>
    where
        Self: Sized + IntoIterator<Item = Result<T>>,
        F: FnOnce(ProcessResults<Self::IntoIter, Report>) -> R,
    {
        itertools::process_results(self, processor)
    }

    fn try_find<P>(&mut self, mut predicate: P) -> Result<Option<Self::Item>>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> Result<bool>,
    {
        self.find_map(|x| match predicate(&x) {
            Ok(false) => None,
            Ok(true) => Some(Ok(x)),
            Err(e) => Some(Err(e)),
        })
        .transpose()
    }

    fn try_sum<T>(self) -> Result<T>
    where
        Self: Sized,
        Result<T>: Sum<Self::Item>,
    {
        self.sum()
    }
}

impl<T: ?Sized + Iterator> IteratorExt for T {}
