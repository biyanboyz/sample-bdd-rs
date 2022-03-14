use crate::test::tester::scenarios::main::implmnts::with_step_runner::abstracts as with_step_runner;

pub mod subs{
  pub mod st{
    pub mod set{
      use crate::test::tester;
      pub trait Core{ type Ret; fn run(x: Self::Ret, val: super::super::super::with_step_runner::subs::steps::jika::buat_surat_tugas::Input)->Result<Self::Ret, tester::common::Error>; }
    }
    pub mod get{
      use crate::test::tester;
      pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<(Self::Ret, super::super::super::with_step_runner::subs::steps::jika::buat_surat_tugas::Input), tester::common::Error>; }
    }
  }
  pub mod state{
    pub mod dengan{
      pub mod user_yang_terdaftar_berjumlah{
        pub mod set{
          use crate::test::tester;
          pub trait Core{ type Ret ; fn run(x: Self::Ret, val: usize)->Result<Self::Ret, tester::common::Error>; }
        }
        pub mod get{
          use crate::test::tester;
          pub trait Core{ type Ret ; fn run(x: Self::Ret)->Result<(Self::Ret, usize), tester::common::Error>; }
        }
      }
      pub mod yang_berwenang_sebagai_pembuat_surat_tugas_adalah{
        pub mod set{
          use crate::test::tester;
          pub trait Core{ type Ret ; fn run(x: Self::Ret, val: String)->Result<Self::Ret, tester::common::Error>; }
        }
        pub mod get{
          use crate::test::tester;
          pub trait Core{ type Ret ; fn run(x: Self::Ret)->Result<(Self::Ret, String), tester::common::Error>; }
        }
      }
      pub mod alur_otorisasi_berupa{
        pub mod set{
          use crate::test::tester;
          pub trait Core{ type Ret ; fn run(x: Self::Ret, val: Vec<String>)->Result<Self::Ret, tester::common::Error>; }
        }
        pub mod get{
          use crate::test::tester;
          pub trait Core{ type Ret ; fn run(x: Self::Ret)->Result<(Self::Ret, Vec<String>), tester::common::Error>; }
        }
      }
    }
    pub mod jika{
      pub mod buat_surat_tugas{
        pub use crate::test::tester::scenarios::main::implmnts::with_step_runner::abstracts::subs::steps::jika::buat_surat_tugas as parent;
        pub mod set{
          use crate::test::tester;
          pub trait Core{ type Ret ; fn run(x: Self::Ret, val: super::parent::Input)->Result<Self::Ret, tester::common::Error>; }
        }
        pub mod get{
          use crate::test::tester;
          pub trait Core{ type Ret ; fn run(x: Self::Ret)->Result<(Self::Ret, super::parent::Input), tester::common::Error>; }
        }
      }
      pub mod sekarang_tanggal{
        pub mod set{
          use crate::test::tester;
          pub trait Core{ type Ret ; fn run(x: Self::Ret, val: crate::common::tanggal::Core)->Result<Self::Ret, tester::common::Error>; }
        }
        pub mod get{
          use crate::test::tester;
          pub trait Core{ type Ret ; fn run(x: Self::Ret)->Result<(Self::Ret, crate::common::tanggal::Core), tester::common::Error>; }
        }
      }
    }
  }
  pub mod peran{ pub mod each{
    pub mod pembuat_surat_tugas{
      pub mod set{
        use crate::test::tester;
        pub trait Core{ type Ret ; fn run(x: Self::Ret, val: Vec<String>)->Result<Self::Ret, tester::common::Error>; }
      }
      pub mod get{
        use crate::test::tester;
        pub trait Core{ type Ret ; fn run(x: Self::Ret)->Result<(Self::Ret, Vec<String>), tester::common::Error>; }
      }
    }
    pub mod otorisator{
      pub mod set{
        use crate::test::tester;
        pub trait Core{ type Ret ; fn run(x: Self::Ret, val: Vec<String>)->Result<Self::Ret, tester::common::Error>; }
      }
      pub mod get{
        use crate::test::tester;
        pub trait Core{ type Ret ; fn run(x: Self::Ret)->Result<(Self::Ret, Vec<String>), tester::common::Error>; }
      }
    }
    pub mod users{
      pub mod set{
        use crate::test::tester;
        pub trait Core{ type Ret ; fn run(x: Self::Ret, val: Vec<String>)->Result<Self::Ret, tester::common::Error>; }
      }
      pub mod get{
        use crate::test::tester;
        pub trait Core{ type Ret ; fn run(x: Self::Ret)->Result<(Self::Ret, Vec<String>), tester::common::Error>; }
      }
    }
  }}
}
pub trait Core :
  subs::st::set::Core<Ret=<Self as Core>::Ret>
+ subs::st::get::Core<Ret=<Self as Core>::Ret>
+ subs::state::dengan::user_yang_terdaftar_berjumlah::set::Core<Ret=<Self as Core>::Ret>
+ subs::state::dengan::user_yang_terdaftar_berjumlah::get::Core<Ret=<Self as Core>::Ret>
+ subs::state::dengan::user_yang_terdaftar_berjumlah::set::Core<Ret=<Self as Core>::Ret>
+ subs::state::dengan::user_yang_terdaftar_berjumlah::get::Core<Ret=<Self as Core>::Ret>
+ subs::state::dengan::yang_berwenang_sebagai_pembuat_surat_tugas_adalah::set::Core<Ret=<Self as Core>::Ret>
+ subs::state::dengan::yang_berwenang_sebagai_pembuat_surat_tugas_adalah::get::Core<Ret=<Self as Core>::Ret>
+ subs::state::dengan::alur_otorisasi_berupa::set::Core<Ret=<Self as Core>::Ret>
+ subs::state::dengan::alur_otorisasi_berupa::get::Core<Ret=<Self as Core>::Ret>
+ subs::state::jika::buat_surat_tugas::set::Core<Ret=<Self as Core>::Ret>
+ subs::state::jika::buat_surat_tugas::get::Core<Ret=<Self as Core>::Ret>
+ subs::state::jika::sekarang_tanggal::set::Core<Ret=<Self as Core>::Ret>
+ subs::state::jika::sekarang_tanggal::get::Core<Ret=<Self as Core>::Ret>
+ subs::peran::each::pembuat_surat_tugas::set::Core<Ret=<Self as Core>::Ret>
+ subs::peran::each::pembuat_surat_tugas::get::Core<Ret=<Self as Core>::Ret>
+ subs::peran::each::otorisator::set::Core<Ret=<Self as Core>::Ret>
+ subs::peran::each::otorisator::get::Core<Ret=<Self as Core>::Ret>
+ subs::peran::each::users::set::Core<Ret=<Self as Core>::Ret>
+ subs::peran::each::users::get::Core<Ret=<Self as Core>::Ret>{
  type Ret;
}