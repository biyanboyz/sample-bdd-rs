pub mod default{
  pub mod flow{
    use super::super::super::abstracts;
    use crate::test::tester;
    use crate::test::tester::scenarios::main::implmnts::with_step_runner::implmnts::parts::run_in_context_of::abstracts as run_in_context_of;
    pub struct Core<T: abstracts::Core>(pub T);
    impl<T: abstracts::Core> run_in_context_of::Core for Core<T>{
      type Ret = Self;
      type State = <T as abstracts::Core>::State;
      fn run<TAction: Fn(Self::State)->Result<Self::State, tester::common::Error>>(x: Self::Ret, target: &run_in_context_of::subs::Target, actions: &TAction)->
      Result<Self::Ret, tester::common::Error>{
      }
    }
  }
}