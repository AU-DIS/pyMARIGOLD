#[cfg(test)]
mod tests {
    use crate::kmeans::KmeansStrategy;
    use crate::marigold::MARIGOLDStrategy;

    #[test]
    fn squared_distance_zero() {
        let mar = MARIGOLDStrategy;
        assert_eq!(mar.run(&[1., 1., 1., 1.], &[2., 2., 2., 2.],1,4,1,100), 2.); //TODO: Placeholder test
    }
}
