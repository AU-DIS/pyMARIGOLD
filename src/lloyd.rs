pub struct LloydStrategy;

impl crate::kmeans::KmeansStrategy for LloydStrategy {
    fn update(&self) {
        println!("Calling rust from python!")
    }
}
