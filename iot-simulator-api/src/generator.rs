pub trait GeneratorPlugin<P, O> {
    fn generate(&mut self, params: P) -> O;
}

pub trait StatefulGeneratorPlugin<P, O>: GeneratorPlugin<P, O> {}

pub trait StatelessGeneratorPlugin<P, O>: GeneratorPlugin<P, O> {}
