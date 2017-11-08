#[derive(Clone, Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}
