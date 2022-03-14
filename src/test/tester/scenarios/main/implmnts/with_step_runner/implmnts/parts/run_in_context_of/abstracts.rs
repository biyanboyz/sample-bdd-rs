use crate::test::tester;
pub mod subs{
  #[derive(Clone, Debug, Eq, PartialEq)] pub enum Target{
    PembuatSuratTugas,
    Otorisator,
    User(String),
    Semua,
  }
}
pub trait Core{
  type Ret;
  type State: crate::test::tester::scenarios::main::implmnts::with_step_runner::implmnts::parts::helperables::abstracts::Core;
  fn run<TAction: Fn(Self::State)->Result<Self::State, tester::common::Error>>(x: Self::Ret, target: &subs::Target, actions: &TAction)->
  Result<Self::Ret, tester::common::Error>;
}