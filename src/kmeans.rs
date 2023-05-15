
pub trait KmeansStrategy {
    fn update(&self);
}

pub struct KmeansRunner<T: KmeansStrategy> {
    kmeans_strategy: T,
}

impl<T: KmeansStrategy> KmeansRunner<T> {
    pub fn new(kmeans_strategy: T) -> Self {
        Self {kmeans_strategy}
    }

    pub fn run(&self) -> String {
        self.kmeans_strategy.update();
        String::from("DONE")
    }
}