pub trait Core{
  type Ret;
  fn run(x: Self::Ret)->Result<Self::Ret, crate::test::tester::common::Error>;
}