pub mod subs{
  pub mod client{ 
    pub mod directly{ pub mod action{
      use crate::test::tester;
      pub trait Core{
        type Ret;
        type State: crate::core::app::client::abstracts::Core;
        fn run<TRetExtra, TAction: Fn(Self::State)->Result<(Self::State, TRetExtra), tester::common::Error>>(x: Self::Ret, action: TAction)->Result<(Self::Ret, TRetExtra), tester::common::Error>; 
      }
    }}
    pub mod helpers{ pub mod pages{
      pub mod home_st{ pub mod pilih{ 
        use crate::test::tester;
        pub trait Core{
          type Ret;
          type State: crate::core::app::client::abstracts::Core;
          fn run<TRetExtra, TAction: Fn(Self::State, usize)->Result<(Self::State, TRetExtra), tester::common::Error>>(x: Self::Ret, action: TAction)->Result<(Self::Ret, TRetExtra), tester::common::Error>;
        }
      }}
      pub mod home_st_buat{ pub mod menugaskan{
        use crate::test::tester;
        pub trait Core{
          type Ret;
          type State: crate::core::app::client::abstracts::Core;
          fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>;
        }
      }}
    }}
  }
}
pub trait Core : 
    subs::client::directly::action::Core<Ret=<Self as Core>::Ret, State=<Self as Core>::State>
  + subs::client::helpers::pages::home_st::pilih::Core<Ret=<Self as Core>::Ret, State=<Self as Core>::State>
  + subs::client::helpers::pages::home_st_buat::menugaskan::Core<Ret=<Self as Core>::Ret, State=<Self as Core>::State>
  + crate::test::tester::scenarios::main::implmnts::with_step_runner::implmnts::parts::stateful::abstracts::Core<Ret=<Self as Core>::Ret>{
    type Ret;
    type State: crate::core::app::client::abstracts::Core;
  }