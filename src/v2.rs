mod packages;
pub use packages::*;

mod projects;
pub use projects::*;

/// utility function for calculating the number of result pages
///
/// c.f. https://doc.rust-lang.org/std/primitive.u32.html#method.div_ceil
fn num_pages(total_items: u32, items_per_page: u32) -> u32 {
    if total_items == 0 {
        return 1;
    }

    let div = total_items / items_per_page;
    let rem = total_items % items_per_page;

    if rem == 0 {
        div
    } else {
        div + 1
    }
}

#[cfg(test)]
mod tests {
    use super::num_pages;

    #[test]
    fn page_count() {
        assert_eq!(num_pages(0, 1), 1);
        assert_eq!(num_pages(1, 1), 1);
        assert_eq!(num_pages(2, 1), 2);
        assert_eq!(num_pages(3, 1), 3);
        assert_eq!(num_pages(4, 1), 4);
        assert_eq!(num_pages(5, 1), 5);

        assert_eq!(num_pages(0, 2), 1);
        assert_eq!(num_pages(1, 2), 1);
        assert_eq!(num_pages(2, 2), 1);
        assert_eq!(num_pages(3, 2), 2);
        assert_eq!(num_pages(4, 2), 2);
        assert_eq!(num_pages(5, 2), 3);

        assert_eq!(num_pages(0, 3), 1);
        assert_eq!(num_pages(1, 3), 1);
        assert_eq!(num_pages(2, 3), 1);
        assert_eq!(num_pages(3, 3), 1);
        assert_eq!(num_pages(4, 3), 2);
        assert_eq!(num_pages(5, 3), 2);

        assert_eq!(num_pages(0, 4), 1);
        assert_eq!(num_pages(1, 4), 1);
        assert_eq!(num_pages(2, 4), 1);
        assert_eq!(num_pages(3, 4), 1);
        assert_eq!(num_pages(4, 4), 1);
        assert_eq!(num_pages(5, 4), 2);

        assert_eq!(num_pages(0, 5), 1);
        assert_eq!(num_pages(1, 5), 1);
        assert_eq!(num_pages(2, 5), 1);
        assert_eq!(num_pages(3, 5), 1);
        assert_eq!(num_pages(4, 5), 1);
        assert_eq!(num_pages(5, 5), 1);
    }
}
