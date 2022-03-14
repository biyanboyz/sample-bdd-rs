#[derive(Clone, Debug, Eq, PartialEq)] pub enum Target{
  PembuatSuratTugas,
  Otorisator,
  User,
}
pub trait Core{
  type Ret;
  type State: crate::test::tester::scenarios::main::implmnts::with_step_runner::implmnts::parts::helperables::abstracts::Core;
  fn run<TAction: Fn(Self::State)->Self::State>(x: Self::Ret, name: &str, target: &Target, actions: &TAction);
}