#[cfg(test)]
mod tests {
    use crate::kmeans_utils::{euclidian_distance, recalculate, squared_distance};
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
    #[test]
    fn recalculate_mutates_centroids() {
        let data = vec![2.,2.,5.,5.,8.,8.];
        let mut centroids = vec![1., 1., 10., 10.];
        let mut labels = [0,0,0];
        recalculate(&data, &mut centroids, &mut labels, 2, 2);
        println!("{:?}",centroids);
        println!("{:?}",labels);
        assert_ne!(centroids, vec![1., 1., 10., 10.]);
    }
    #[test]
    fn recalculate_only_changes_clusters_with_points() {
        let data = vec![2.,2.,5.,5.,8.,8.];
        let mut centroids = vec![1., 1., 10., 10.];
        let mut labels = [0,0,0];
        recalculate(&data, &mut centroids, &mut labels, 2, 2);

        assert_ne!(centroids, vec![3.5, 3.5, 10., 10.]);
    }

    #[test]
    fn recalculate_does_not_mutate_labels() {
        let data = vec![2.,2.,5.,5.,8.,8.];
        let mut centroids = vec![1., 1., 10., 10.];
        let mut labels = [0,0,0];
        recalculate(&data, &mut centroids, &mut labels, 2, 2);

        assert_eq!(labels, [0,0,0]);
    }
    #[test]
    fn recalculate_does_not_mutate_data() {
        let data = vec![2.,2.,5.,5.,8.,8.];
        let mut centroids = vec![1., 1., 10., 10.];
        let mut labels = [0,0,0];
        recalculate(&data, &mut centroids, &mut labels, 2, 2);

        assert_eq!(data, vec![2., 2., 5., 5., 8.,8.]);
    }

}
