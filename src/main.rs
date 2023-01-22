mod histogram {
    pub trait Histogram {
        fn width(&self) -> usize;
        fn height_at(&self, horizontal_position: usize) -> i32;
    }
}

mod square_search {
    use crate::histogram::Histogram;
    use std::cmp;

    pub fn compute_area_of_largest_rectangle<H: Histogram>(histogram: &H) -> i32 {
        let searcher = LargestRectangleSearcher::new(histogram);
        searcher.compute_area_of_largest_rectangle()
    }

    struct LargestRectangleSearcher<'a, H: Histogram> {
        histogram: &'a H,
        recorded_bars_of_increasing_height: Vec<i32>,
    }

    impl<'a, H: Histogram> LargestRectangleSearcher<'a, H> {
        fn new(histogram: &'a H) -> Self {
            Self {
                histogram,
                recorded_bars_of_increasing_height: vec![-1],
            }
        }

        fn compute_area_of_largest_rectangle(mut self) -> i32 {
            let mut area_of_largest_rectangle = 0;
            for x_pos in 0..self.width() + 1 {
                if self.new_bar_is_not_lower(x_pos) {
                    self.adjust_recorded_bars_of_increasing_height(x_pos);
                } else {
                    area_of_largest_rectangle = cmp::max(
                        area_of_largest_rectangle,
                        self.compute_area_of_largest_rectangle_impl(x_pos),
                    );
                }
            }
            area_of_largest_rectangle
        }

        fn height_at(&self, x_pos: i32) -> i32 {
            assert!(x_pos >= -1);
            assert!(x_pos <= self.width());
            if x_pos >= 0 && x_pos < self.width() {
                self.histogram.height_at(x_pos as usize)
            } else {
                0
            }
        }

        fn width(&self) -> i32 {
            self.histogram.width() as i32
        }

        fn compute_area_of_largest_rectangle_impl(&mut self, x_pos: i32) -> i32 {
            assert!(!self.recorded_bars_of_increasing_height.is_empty());
            let current_bar_height = self.height_at(x_pos);
            let mut area_of_largest_rectangle = 0;
            while self.height_of_last_recorded_bar() > current_bar_height {
                area_of_largest_rectangle = cmp::max(
                    area_of_largest_rectangle,
                    self.compute_area_of_rectangle_at_last_recorded_bar(x_pos),
                );
                self.recorded_bars_of_increasing_height.pop();
            }
            self.adjust_recorded_bars_of_increasing_height(x_pos);
            area_of_largest_rectangle
        }

        fn height_of_last_recorded_bar(&self) -> i32 {
            self.height_at(last_element(&self.recorded_bars_of_increasing_height))
        }

        fn compute_area_of_rectangle_at_last_recorded_bar(&self, x_pos: i32) -> i32 {
            assert!(self.recorded_bars_of_increasing_height.len() >= 2);
            let width = x_pos - second_last_element(&self.recorded_bars_of_increasing_height) - 1;
            let height = self.height_of_last_recorded_bar();
            (width as i32) * height
        }

        fn new_bar_is_not_lower(&self, x_pos: i32) -> bool {
            self.new_bar_is_higher(x_pos) || self.new_bar_is_same_size(x_pos)
        }

        fn adjust_recorded_bars_of_increasing_height(&mut self, x_pos: i32) {
            assert!(self.new_bar_is_not_lower(x_pos));
            if self.new_bar_is_higher(x_pos) {
                self.recorded_bars_of_increasing_height.push(x_pos);
            } else {
                replace_last_element(&mut self.recorded_bars_of_increasing_height, x_pos);
            }
        }

        fn new_bar_is_higher(&self, new_x_pos: i32) -> bool {
            assert!(!self.recorded_bars_of_increasing_height.is_empty());
            self.height_at(new_x_pos) > self.height_of_last_recorded_bar()
        }

        fn new_bar_is_same_size(&self, new_x_pos: i32) -> bool {
            assert!(!self.recorded_bars_of_increasing_height.is_empty());
            self.height_at(new_x_pos) == self.height_of_last_recorded_bar()
        }
    }

    fn last_element(ints: &Vec<i32>) -> i32 {
        assert!(!ints.is_empty());
        *ints.last().unwrap()
    }

    fn second_last_element(ints: &Vec<i32>) -> i32 {
        assert!(ints.len() >= 2);
        ints[ints.len() - 2]
    }

    fn replace_last_element(ints: &mut Vec<i32>, new_last_element: i32) {
        assert!(!ints.is_empty());
        ints.pop();
        ints.push(new_last_element);
    }
}

mod histogram_concrete {
    use crate::histogram::Histogram;

    pub struct ConcreteHistogram {
        bars: Vec<i32>,
    }

    impl ConcreteHistogram {
        pub fn new(bars: Vec<i32>) -> Self {
            Self { bars }
        }
    }

    impl Histogram for ConcreteHistogram {
        fn height_at(&self, horizontal_position: usize) -> i32 {
            self.bars[horizontal_position]
        }

        fn width(&self) -> usize {
            self.bars.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::histogram_concrete::ConcreteHistogram;
    use crate::square_search;

    #[test]
    fn test_two_bar_histogram() {
        let histogram = ConcreteHistogram::new(vec![2, 3]);
        let area_of_largest_rectangle =
            square_search::compute_area_of_largest_rectangle(&histogram);
        assert_eq!(area_of_largest_rectangle, 4);
    }

    #[test]
    fn test_constant_histogram() {
        let histogram = ConcreteHistogram::new(vec![1, 1, 1]);
        let area_of_largest_rectangle =
            square_search::compute_area_of_largest_rectangle(&histogram);
        assert_eq!(area_of_largest_rectangle, 3);
    }

    #[test]
    fn test_histogram_with_insignificant_peak() {
        let histogram = ConcreteHistogram::new(vec![1, 2, 1]);
        let area_of_largest_rectangle =
            square_search::compute_area_of_largest_rectangle(&histogram);
        assert_eq!(area_of_largest_rectangle, 3);
    }

    #[test]
    fn test_histogram_with_trough() {
        let histogram = ConcreteHistogram::new(vec![2, 1, 2]);
        let area_of_largest_rectangle =
            square_search::compute_area_of_largest_rectangle(&histogram);
        assert_eq!(area_of_largest_rectangle, 3);
    }

    #[test]
    fn test_histogram_with_significant_peak() {
        let histogram = ConcreteHistogram::new(vec![1, 4, 1]);
        let area_of_largest_rectangle =
            square_search::compute_area_of_largest_rectangle(&histogram);
        assert_eq!(area_of_largest_rectangle, 4);
    }

    #[test]
    fn test_leetcode_example() {
        let histogram = ConcreteHistogram::new(vec![2, 1, 5, 6, 2, 3]);
        let area_of_largest_rectangle =
            square_search::compute_area_of_largest_rectangle(&histogram);
        assert_eq!(area_of_largest_rectangle, 10);
    }
}

fn main() {
    let histogram = histogram_concrete::ConcreteHistogram::new(vec![2, 1, 5, 6, 2, 3]);
    let area_of_largest_rectangle = square_search::compute_area_of_largest_rectangle(&histogram);
    assert_eq!(area_of_largest_rectangle, 10);
}
