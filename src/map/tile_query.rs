use super::grid::Grid;

pub trait GridFilter<T: Default + Clone> {
    fn is_match(&self, position: (usize, usize), item: &T) -> bool;
}

pub struct GridQuery<'a, T: Default + Clone> {
    filters: Vec<&'a dyn GridFilter<T>>,
}

impl<'a, T: Default + Clone> GridQuery<'a, T> {
    pub fn new() -> Self {
        GridQuery { filters: vec![] }
    }

    pub fn add_filter(mut self, filter: &'a dyn GridFilter<T>) -> Self {
        self.filters.push(filter);

        self
    }

    pub fn query<'b>(&self, grid: &'b Grid<T>) -> Vec<(usize, &'b T)> {
        grid.items
            .iter()
            .enumerate()
            .filter(|&(index, item)| {
                let position = (index % grid.width, index / grid.width);

                self.filters
                    .iter()
                    .all(|filter| filter.is_match(position, item))
            })
            .collect()
    }
}
