#[cfg(test)]
mod tests {
    use crate::kmeans_utils::{euclidian_distance, squared_distance, recalculate};
    #[test]
    fn squared_distance_zero() {
        assert_eq!(
            squared_distance::<f64>(&[0., 0., 0., 0.], &[0., 0., 0., 0.]),
            0.
        );
    }
    #[test]
    fn squared_distance_zero_single() {
        assert_eq!(squared_distance::<f64>(&[0.], &[0.]), 0.);
    }
    #[test]
    fn squared_distance_4d() {
        assert_eq!(
            squared_distance::<f64>(&[1., 2., 3., 4.], &[2., 2., 2., 2.]),
            6.
        );
    }
    #[test]
    fn squared_distance_single() {
        assert_eq!(squared_distance::<f64>(&[1.], &[2.]), 1.);
    }

    #[test]
    fn euclidian_distance_zero() {
        assert_eq!(
            euclidian_distance::<f64>(&[0., 0., 0., 0.], &[0., 0., 0., 0.]),
            0.
        );
    }
    #[test]
    fn euclidian_distance_zero_single() {
        assert_eq!(euclidian_distance::<f64>(&[0.], &[0.]), 0.);
    }
    #[test]
    fn euclidian_distance_4d() {
        assert_eq!(
            euclidian_distance::<f64>(&[1., 2., 3., 4.], &[2., 2., 2., 2.]),
            f64::sqrt(6.)
        );
    }
    #[test]
    fn euclidian_distance_single() {
        assert_eq!(euclidian_distance::<f64>(&[1.], &[2.]), 1.);
    }
    #[test]
    fn squared_sqrt_eq_euclidian() {
        assert_eq!(
            euclidian_distance::<f64>(&[1., 2., 3., 4.], &[2., 2., 2., 2.]),
            f64::sqrt(squared_distance::<f64>(
                &[1., 2., 3., 4.],
                &[2., 2., 2., 2.]
            ))
        );
    }
    #[test]
    fn implicit_f64() {
        assert_eq!(
            euclidian_distance(&[1., 2., 3., 4.], &[2., 2., 2., 2.]),
            f64::sqrt(6.)
        );
    }
    #[test]
    fn explicit_f32() {
        assert_eq!(
            euclidian_distance::<f32>(&[1., 2., 3., 4.], &[2., 2., 2., 2.]),
            f32::sqrt(6.)
        );
    }

    fn recalculate_updates_centroids {

    }
}
