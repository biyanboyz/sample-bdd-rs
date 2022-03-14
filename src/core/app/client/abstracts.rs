pub mod subs{
  pub mod pages{
    pub mod home{
      pub mod visit{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
      }
      pub mod button_st{ pub mod input_click{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
      }}
    }
    pub mod home_st{
      pub mod button_buat{ pub mod input_click{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
      }}
      pub mod table{ pub mod baris{
        pub struct Data{
          pub id: String,
          pub warna: crate::core::domain::st::Warna,
        }
        pub mod warna{ pub mod get{
          use crate::test::tester;
          pub trait Core{ type Ret; fn run(x: Self::Ret, index: usize)->Result<(Self::Ret, crate::core::domain::st::Warna), tester::common::Error>; }
        }}
        pub mod kolom_tindakan{
          pub mod button_otorisasi{ pub mod input_click{
            use crate::test::tester;
            pub trait Core{ type Ret; fn run(x: Self::Ret, index: usize)->Result<Self::Ret, tester::common::Error>; }
          }}
          pub mod button_lapor{ pub mod input_click{
            use crate::test::tester;
            pub trait Core{ type Ret; fn run(x: Self::Ret, index: usize)->Result<Self::Ret, tester::common::Error>; }
          }}
          pub mod button_otorisasi_laporan{ pub mod input_click{
            use crate::test::tester;
            pub trait Core{ type Ret; fn run(x: Self::Ret, index: usize)->Result<Self::Ret, tester::common::Error>; }
          }}
        }
      }}
    }
    pub mod home_st_buat{
      pub mod bagian_menugaskan{
        pub mod button_tambah{ pub mod input_click{
          use crate::test::tester;
          pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
        }}
      }
      pub mod bagian_tanggal{
        pub mod bagian_tanggal{ pub mod input_isi{
          use crate::test::tester;
          pub trait Core{ type Ret; fn run(x: Self::Ret, index: u8)->Result<Self::Ret, tester::common::Error>; }
        }}
        pub mod bagian_bulan{ pub mod input_isi{
          use crate::test::tester;
          pub trait Core{ type Ret; fn run(x: Self::Ret, index: u8)->Result<Self::Ret, tester::common::Error>; }
        }}
        pub mod bagian_tahun{ pub mod input_isi{
          use crate::test::tester;
          pub trait Core{ type Ret; fn run(x: Self::Ret, index: usize)->Result<Self::Ret, tester::common::Error>; }
        }}
      }
      pub mod bagian_durasi{ pub mod input_isi{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret, index: usize)->Result<Self::Ret, tester::common::Error>; }
      }}
      pub mod button_ok{ pub mod input_click{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
      }}
    }
    pub mod home_st_buat_menugaskan{
      pub mod table{
        pub mod baris{
          pub use crate::core::domain::pegawai::Core as Data;
          pub mod get{
            use crate::test::tester;
            pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
          }
          pub mod kolom_tindakan{ pub mod button_tambah{ pub mod input_click{ 
            use crate::test::tester;
            pub trait Core{ type Ret; fn run(x: Self::Ret, index: usize)->Result<Self::Ret, tester::common::Error>; }
          }}}
        }
        pub mod nav{
          pub mod button_next{ pub mod input_click{
            use crate::test::tester;
            pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
          }}
        }
      }
    }
    pub mod home_st_buat_konfirmasi{
      pub mod button_ok{ pub mod input_click{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
      }}
    }
    pub mod home_st_buat_informasi{
      pub mod button_ok{ pub mod input_click{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
      }}
    }
    pub mod home_st_otorisasi{
      pub mod bagian_status{ pub mod radio_setujui{ pub mod input_click{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
      }}}
      pub mod button_ok{ pub mod input_click{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
      }}
    }
    pub mod home_st_otorisasi_konfirmasi{
      pub mod button_ok{ pub mod input_click{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
      }}
    }
    pub mod home_st_otorisasi_informasi{
      pub mod button_ok{ pub mod input_click{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
      }}
    }
    pub mod home_st_lapor{
      pub mod bagian_dokumen{ pub mod input_isi{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret, val: String)->Result<Self::Ret, tester::common::Error>; }
      }}
      pub mod button_ok{ pub mod input_click{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
      }}
    }
    pub mod home_st_lapor_konfirmasi{
      pub mod button_ok{ pub mod input_click{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
      }}
    }
    pub mod home_st_lapor_informasi{
      pub mod button_ok{ pub mod input_click{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
      }}
    }
    pub mod home_st_otorisasi_laporan{
      pub mod bagian_status{ pub mod radio_setujui{ pub mod input_click{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
      }}}
      pub mod button_ok{ pub mod input_click{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
      }}
    }
    pub mod home_st_otorisasi_laporan_konfirmasi{
      pub mod button_ok{ pub mod input_click{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
      }}
    }
    pub mod home_st_otorisasi_laporan_informasi{
      pub mod button_ok{ pub mod input_click{
        use crate::test::tester;
        pub trait Core{ type Ret; fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>; }
      }}
    }
  }
}
// pub trait Core : 
//   subs::pages::home::visit::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home::button_st::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st::button_buat::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st::table::baris::warna::get::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st::table::baris::kolom_tindakan::button_otorisasi::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st::table::baris::kolom_tindakan::button_lapor::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st::table::baris::kolom_tindakan::button_otorisasi_laporan::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_buat::bagian_menugaskan::button_tambah::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_buat::bagian_tanggal::bagian_tanggal::input_isi::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_buat::bagian_tanggal::bagian_bulan::input_isi::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_buat::bagian_tanggal::bagian_tahun::input_isi::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_buat::bagian_durasi::input_isi::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_buat::button_ok::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_buat_konfirmasi::button_ok::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_buat_informasi::button_ok::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_buat_menugaskan::table::baris::get::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_buat_menugaskan::table::baris::kolom_tindakan::button_tambah::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_buat_menugaskan::table::nav::button_next::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_otorisasi::bagian_status::radio_setujui::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_otorisasi::button_ok::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_otorisasi_konfirmasi::button_ok::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_otorisasi_informasi::button_ok::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_lapor::bagian_dokumen::input_isi::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_lapor::button_ok::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_lapor_konfirmasi::button_ok::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_lapor_informasi::button_ok::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_otorisasi_laporan::bagian_status::radio_setujui::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_otorisasi_laporan::button_ok::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_otorisasi_laporan_konfirmasi::button_ok::input_click::Core<Ret=<Self as Core>::Ret>
// + subs::pages::home_st_otorisasi_laporan_informasi::button_ok::input_click::Core<Ret=<Self as Core>::Ret>{
//   type Ret;
// }
pub trait Core : 
  subs::pages::home::visit::Core<Ret=Self>
+ subs::pages::home::button_st::input_click::Core<Ret=Self>
+ subs::pages::home_st::button_buat::input_click::Core<Ret=Self>
+ subs::pages::home_st::table::baris::warna::get::Core<Ret=Self>
+ subs::pages::home_st::table::baris::kolom_tindakan::button_otorisasi::input_click::Core<Ret=Self>
+ subs::pages::home_st::table::baris::kolom_tindakan::button_lapor::input_click::Core<Ret=Self>
+ subs::pages::home_st::table::baris::kolom_tindakan::button_otorisasi_laporan::input_click::Core<Ret=Self>
+ subs::pages::home_st_buat::bagian_menugaskan::button_tambah::input_click::Core<Ret=Self>
+ subs::pages::home_st_buat::bagian_tanggal::bagian_tanggal::input_isi::Core<Ret=Self>
+ subs::pages::home_st_buat::bagian_tanggal::bagian_bulan::input_isi::Core<Ret=Self>
+ subs::pages::home_st_buat::bagian_tanggal::bagian_tahun::input_isi::Core<Ret=Self>
+ subs::pages::home_st_buat::bagian_durasi::input_isi::Core<Ret=Self>
+ subs::pages::home_st_buat::button_ok::input_click::Core<Ret=Self>
+ subs::pages::home_st_buat_konfirmasi::button_ok::input_click::Core<Ret=Self>
+ subs::pages::home_st_buat_informasi::button_ok::input_click::Core<Ret=Self>
+ subs::pages::home_st_buat_menugaskan::table::baris::get::Core<Ret=Self>
+ subs::pages::home_st_buat_menugaskan::table::baris::kolom_tindakan::button_tambah::input_click::Core<Ret=Self>
+ subs::pages::home_st_buat_menugaskan::table::nav::button_next::input_click::Core<Ret=Self>
+ subs::pages::home_st_otorisasi::bagian_status::radio_setujui::input_click::Core<Ret=Self>
+ subs::pages::home_st_otorisasi::button_ok::input_click::Core<Ret=Self>
+ subs::pages::home_st_otorisasi_konfirmasi::button_ok::input_click::Core<Ret=Self>
+ subs::pages::home_st_otorisasi_informasi::button_ok::input_click::Core<Ret=Self>
+ subs::pages::home_st_lapor::bagian_dokumen::input_isi::Core<Ret=Self>
+ subs::pages::home_st_lapor::button_ok::input_click::Core<Ret=Self>
+ subs::pages::home_st_lapor_konfirmasi::button_ok::input_click::Core<Ret=Self>
+ subs::pages::home_st_lapor_informasi::button_ok::input_click::Core<Ret=Self>
+ subs::pages::home_st_otorisasi_laporan::bagian_status::radio_setujui::input_click::Core<Ret=Self>
+ subs::pages::home_st_otorisasi_laporan::button_ok::input_click::Core<Ret=Self>
+ subs::pages::home_st_otorisasi_laporan_konfirmasi::button_ok::input_click::Core<Ret=Self>
+ subs::pages::home_st_otorisasi_laporan_informasi::button_ok::input_click::Core<Ret=Self>{
  type Ret;
}