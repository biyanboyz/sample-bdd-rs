pub mod subs{ pub mod steps{
  pub mod dengan{
    pub mod user_yang_terdaftar_berjumlah{
      use crate::test::tester;
      pub trait Core{ type Ret; fn run(x: Self::Ret, val: usize)->Result<Self::Ret, tester::common::Error>; }
    }
    pub mod yang_berwenang_sebagai_pembuat_surat_tugas_adalah{
      use crate::test::tester;
      pub trait Core{ type Ret; fn run(x: Self::Ret, val: String)->Result<Self::Ret, tester::common::Error>; }
    }
    pub mod alur_otorisasi_berupa{
      use crate::test::tester;
      pub trait Core{ type Ret; fn run<TVal: Iterator<Item=String>>(x: Self::Ret, val: TVal)->Result<Self::Ret, tester::common::Error>; }
    }
  }
  pub mod jika{
    pub mod buat_surat_tugas{
      use crate::test::tester;
      #[derive(Clone, Debug, Eq, PartialEq)] pub struct Input{
        pub menugaskan: Vec::<String>,
        pub durasi: usize,
        pub tanggal: crate::common::tanggal::Core,
      }
      pub trait Core{ type Ret; fn run(x: Self::Ret, val: Input)->Result<Self::Ret, tester::common::Error>; }
    }
    pub mod sekarang_tanggal{
      use crate::test::tester;
      pub trait Core{ type Ret; fn run(x: Self::Ret, val: crate::common::tanggal::Core)->Result<Self::Ret, tester::common::Error>; }
    }
    pub mod otorisasi{
      use crate::test::tester;
      pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
    }
    pub mod user_laporan{
      use crate::test::tester;
      pub trait Core{ type Ret; fn run(x: Self::Ret, val: String)->Result<Self::Ret, tester::common::Error>; }
    }
    pub mod otorisasi_laporan{
      use crate::test::tester;
      pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
    }
  }
  pub mod maka{
    pub mod surat_tugas_akan_berwarna{
      use crate::test::tester;
      pub trait Core{ type Ret; fn run(x: Self::Ret, val: &crate::core::domain::st::Warna)->Result<Self::Ret, tester::common::Error>; }
    }
  }
}}
pub trait Core : 
  subs::steps::dengan::user_yang_terdaftar_berjumlah::Core<Ret=<Self as Core>::Ret>
+ subs::steps::dengan::yang_berwenang_sebagai_pembuat_surat_tugas_adalah::Core<Ret=<Self as Core>::Ret>
+ subs::steps::dengan::alur_otorisasi_berupa::Core<Ret=<Self as Core>::Ret>
+ subs::steps::jika::buat_surat_tugas::Core<Ret=<Self as Core>::Ret>
+ subs::steps::jika::sekarang_tanggal::Core<Ret=<Self as Core>::Ret>
+ subs::steps::jika::otorisasi::Core<Ret=<Self as Core>::Ret>
+ subs::steps::jika::user_laporan::Core<Ret=<Self as Core>::Ret>
+ subs::steps::jika::otorisasi_laporan::Core<Ret=<Self as Core>::Ret>
+ subs::steps::maka::surat_tugas_akan_berwarna::Core<Ret=<Self as Core>::Ret>{
    type Ret;
}