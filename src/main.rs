pub mod test{
  pub mod testers{
    pub mod common{
      pub mod sut{
        pub mod core{
          pub mod client{
            pub mod abstracts{
              pub trait Core : 
                  pages::home::visit::Core
                + pages::home::button_st::input_click::Core
                + pages::home_st::visit::Core
                + pages::home_st::button_buat::input_click::Core
                + pages::home_st::table::baris::button_otorisasi::input_click::Core
                + pages::home_st::table::baris::button_lapor::input_click::Core
                + pages::home_st::table::baris::button_otorisasi_laporan::input_click::Core
                + pages::home_st_buat::visit::Core
                + pages::home_st_buat::bagian_menugaskan::button_tambah::input_click::Core
                + pages::home_st_buat::bagian_tanggal::bagian_tahun::input_isi::Core
                + pages::home_st_buat::bagian_tanggal::bagian_bulan::input_isi::Core
                + pages::home_st_buat::bagian_tanggal::bagian_tanggal::input_isi::Core
                + pages::home_st_buat::bagian_durasi::input_isi::Core
                + pages::home_st_buat::button_ok::input_click::Core
                + pages::home_st_buat_menugaskan::visit::Core
                + pages::home_st_buat_menugaskan::table::baris::button_tambah::input_click::Core
                + pages::home_st_buat_konfirmasi::visit::Core
                + pages::home_st_buat_konfirmasi::button_ok::input_click::Core
                + pages::home_st_buat_informasi::visit::Core
                + pages::home_st_buat_informasi::button_ok::input_click::Core
                + pages::home_st_otorisasi::visit::Core
                + pages::home_st_otorisasi::bagian_status::radio_setuju::input_click::Core
                + pages::home_st_otorisasi::button_ok::input_click::Core
                + pages::home_st_otorisasi_konfirmasi::visit::Core
                + pages::home_st_otorisasi_konfirmasi::button_ok::input_click::Core
                + pages::home_st_otorisasi_informasi::visit::Core
                + pages::home_st_otorisasi_informasi::button_ok::input_click::Core
                + pages::home_st_lapor::visit::Core
                + pages::home_st_lapor::bagian_dokumen::input_isi::Core
                + pages::home_st_lapor::button_ok::input_click::Core
                + pages::home_st_lapor_konfirmasi::visit::Core
                + pages::home_st_lapor_konfirmasi::button_ok::input_click::Core
                + pages::home_st_lapor_informasi::visit::Core
                + pages::home_st_lapor_informasi::button_ok::input_click::Core
                + pages::home_st_otorisasilaporan::visit::Core
                + pages::home_st_otorisasilaporan::bagian_status::radio_setuju::input_click::Core
                + pages::home_st_otorisasilaporan::button_ok::input_click::Core
                + pages::home_st_otorisasilaporan_konfirmasi::visit::Core
                + pages::home_st_otorisasilaporan_konfirmasi::button_ok::input_click::Core
                + pages::home_st_otorisasilaporan_informasi::visit::Core
                + pages::home_st_otorisasilaporan_informasi::button_ok::input_click::Core
              {}
              #[derive(Clone, Debug, Eq, PartialEq)] pub enum Address{
                Home,
                Home_St,
                Home_St_Buat,
                Home_St_Buat_Menugaskan,
                Home_St_Buat_Konfirmasi,
                Home_St_Buat_Informasi,
                Home_St_Otorisasi,
                Home_St_Otorisasi_Konfirmasi,
                Home_St_Otorisasi_Informasi,
                Home_St_Lapor,
                Home_St_Lapor_Konfirmasi,
                Home_St_Lapor_Informasi,
                Home_St_OtorisasiLaporan,
                Home_St_OtorisasiLaporan_Konfirmasi,
                Home_St_OtorisasiLaporan_Informasi,
              }
              pub mod pages{
                pub mod home{
                  pub mod visit{ pub trait Core{ fn run(&mut self); }}
                  pub mod button_st{
                    pub mod input_click{ pub trait Core{ fn run(&mut self); }}
                  }
                }
                pub mod home_st{
                  pub mod visit{
                    pub trait Core{ fn run(&mut self);}
                  }
                  pub mod button_buat{
                    pub mod input_click{ pub trait Core{ fn run(&mut self); }}
                  }
                  pub mod table{ pub mod baris{ 
                    pub mod button_otorisasi{ pub mod input_click{ pub trait Core{ fn run(&mut self, index: usize); }}}
                    pub mod button_lapor{ pub mod input_click{ pub trait Core{ fn run(&mut self, index: usize); }}}
                    pub mod button_otorisasi_laporan{ pub mod input_click{ pub trait Core{ fn run(&mut self, index: usize); }}}
                  }}
                }
                pub mod home_st_buat{
                  pub mod visit{
                    pub trait Core{ fn run(&mut self); }
                  }
                  pub mod bagian_menugaskan{
                    pub mod button_tambah{
                      pub mod input_click{ pub trait Core{ fn run(&mut self); }}
                    }
                  }
                  pub mod bagian_tanggal{
                    pub mod bagian_tahun{
                      pub mod input_isi{ pub trait Core{ fn run(&mut self, val: usize); }}
                    }
                    pub mod bagian_bulan{
                      pub mod input_isi{ pub trait Core{ fn run(&mut self, val: u8); }}
                    }
                    pub mod bagian_tanggal{
                      pub mod input_isi{ pub trait Core{ fn run(&mut self, val: u8); }}
                    }
                  }
                  pub mod bagian_durasi{
                    pub mod input_isi{ pub trait Core{ fn run(&mut self, val: usize); }}
                  }
                  pub mod button_ok{
                    pub mod input_click{ pub trait Core{ fn run(&mut self); }}
                  }
                }
                pub mod home_st_buat_menugaskan{
                  pub mod visit{
                    pub trait Core{ fn run(&mut self); }
                  }
                  pub mod table{pub mod baris{
                    pub mod button_tambah{pub mod input_click{ pub trait Core{ fn run(&mut self, index: usize); }}}
                  }}
                }
                pub mod home_st_buat_konfirmasi{
                  pub mod visit{
                    pub trait Core{ fn run(&mut self); }
                  }
                  pub mod button_ok{
                    pub mod input_click{ pub trait Core{ fn run(&mut self); }}
                  }
                }
                pub mod home_st_buat_informasi{
                  pub mod visit{
                    pub trait Core{ fn run(&mut self); }
                  }
                  pub mod button_ok{
                    pub mod input_click{ pub trait Core{ fn run(&mut self); }}
                  }
                }
                pub mod home_st_otorisasi{
                  pub mod visit{
                    pub trait Core{ fn run(&mut self); }
                  }
                  pub mod bagian_status{
                    pub mod radio_setuju{
                      pub mod input_click{ pub trait Core{ fn run(&mut self); }}
                    }
                  }
                  pub mod button_ok{
                    pub mod input_click{ pub trait Core{ fn run(&mut self); }}
                  }
                }
                pub mod home_st_otorisasi_konfirmasi{
                  pub mod visit{
                    pub trait Core{ fn run(&mut self); }
                  }
                  pub mod button_ok{
                    pub mod input_click{ pub trait Core{ fn run(&mut self); }}
                  }
                }
                pub mod home_st_otorisasi_informasi{
                  pub mod visit{
                    pub trait Core{ fn run(&mut self); }
                  }
                  pub mod button_ok{
                    pub mod input_click{ pub trait Core{ fn run(&mut self); }}
                  }
                }
                pub mod home_st_lapor{
                  pub mod visit{
                    pub trait Core{ fn run(&mut self); }
                  }
                  pub mod bagian_dokumen{
                    pub mod input_isi{ pub trait Core{ fn run(&mut self, val: String); }}
                  }
                  pub mod button_ok{
                    pub mod input_click{ pub trait Core{ fn run(&mut self); }}
                  }
                }
                pub mod home_st_lapor_konfirmasi{
                  pub mod visit{
                    pub trait Core{ fn run(&mut self); }
                  }
                  pub mod button_ok{
                    pub mod input_click{ pub trait Core{ fn run(&mut self); }}
                  }
                }
                pub mod home_st_lapor_informasi{
                  pub mod visit{
                    pub trait Core{ fn run(&mut self); }
                  }
                  pub mod button_ok{
                    pub mod input_click{ pub trait Core{ fn run(&mut self); }}
                  }
                }
                pub mod home_st_otorisasilaporan{
                  pub mod visit{
                    pub trait Core{ fn run(&mut self); }
                  }
                  pub mod bagian_status{
                    pub mod radio_setuju{
                      pub mod input_click{ pub trait Core{ fn run(&mut self); }}
                    }
                  }
                  pub mod button_ok{
                    pub mod input_click{ pub trait Core{ fn run(&mut self); }}
                  }
                }
                pub mod home_st_otorisasilaporan_konfirmasi{
                  pub mod visit{
                    pub trait Core{ fn run(&mut self); }
                  }
                  pub mod button_ok{
                    pub mod input_click{ pub trait Core{ fn run(&mut self); }}
                  }
                }
                pub mod home_st_otorisasilaporan_informasi{
                  pub mod visit{
                    pub trait Core{ fn run(&mut self); }
                  }
                  pub mod button_ok{
                    pub mod input_click{ pub trait Core{ fn run(&mut self); }}
                  }
                } 
              }
            }
            pub mod implmnts{}
          }
          pub mod server{
            pub mod abstracts{
              pub trait Core<TSession> :
                  users::read::Core
                + st::read::Core
                + st::buat::Core
                + st::lapor::Core
                + st::otorisasi::Core
                + st::otorisasi_laporan::Core
              {}
              pub mod users{
                pub mod read{
                  pub mod result{
                    pub struct Core{
                      pub id: String,
                      pub nama: String,
                    }
                  }
                  pub trait Core{
                    fn run(&mut self)->Vec<result::Core>;
                  }
                }
              }
              pub mod st{
                pub mod read{
                  pub mod result{ 
                    pub enum Warna{
                      Putih,
                      Merah,
                      Kuning,
                      Hijau,
                    }
                    pub struct Core{
                      pub id: String,
                      pub warna: Warna,
                    }
                  }
                  pub trait Core{
                    fn run(&mut self)->Vec<result::Core>;
                  }
                }
                pub mod buat{
                  pub trait Core{ fn run(&mut self, session: String, args: args::Core); }
                  pub mod args{
                    pub struct Core{
                      pub menugaskan: Vec<menugaskan::Core>,
                      pub tanggal: (u8, u8, usize),
                      pub durasi: usize
                    }
                    pub mod menugaskan{
                      #[derive(Clone, Debug, Eq, PartialEq)] pub struct Core{
                        pub id: String,
                        pub nama: String,
                      }
                    }
                  }
                }
                pub mod otorisasi{
                  pub trait Core{
                    fn run(&mut self, session: String, args: Args);
                  }
                  pub struct Args{
                    pub id: String,
                    pub status: bool,
                  }
                }
                pub mod lapor{
                  pub trait Core{
                    fn run(&mut self, session: String, args: Args);
                  }
                  pub struct Args{
                    pub id: String,
                    pub dokumen: String,
                  }
                }
                pub mod otorisasi_laporan{
                  pub trait Core{
                    fn run(&mut self, session: String, args: Args);
                  }
                  pub struct Args{
                    pub id: String,
                    pub status: bool,
                  }
                }
              }
            }
            pub mod implmnts{}
          }
        }
      }
    }
    pub mod features{ pub mod core{
      pub mod abstracts{
        pub trait Core{ fn run(&mut self); }
      }
      pub mod implmnts{
        pub mod components{ pub mod step{
          pub mod abstracts{
            pub struct Core<TState: state::Core>{
              pub background: Vec<Box<Fn()->background::Choices>>,
              pub steps: Vec<Box<Fn()->steps::Choices>>,
              pub state: TState,
            }
            impl<TState: state::Core> crate::test::testers::features::core::abstracts::Core for Core<TState>{
              fn run(&mut self){
                for v in &self.background{
                  let v = v();
                  match v{
                    background::Choices::TanggalMulai(v)=>state::background::subs::tanggal_mulai::Core::run(&mut self.state, &v),
                    background::Choices::UserAdanya(v)=>state::background::subs::user_adanya::Core::run(&mut self.state, &v),
                    background::Choices::UserDenganPeran(v)=>state::background::subs::user_dengan_peran::Core::run(&mut self.state, &v),
                    background::Choices::StBerlajur(v)=>state::background::subs::st_berlajur::Core::run(&mut self.state, &v),
                    background::Choices::StDenganTanggal(v)=>state::background::subs::st_dengan_tanggal::Core::run(&mut self.state, &v),
                    background::Choices::StDenganDurasi(v)=>state::background::subs::st_dengan_durasi::Core::run(&mut self.state, &v),
                    background::Choices::StDenganMenugaskan(v)=>state::background::subs::st_dengan_menugaskan::Core::run(&mut self.state, &v),
                  }
                };
                for v in &self.steps{
                  let v = v();
                  match v{
                    steps::Choices::JikaBuatSuratTugas=>state::steps::subs::jika_buat_surat_tugas::Core::run(&mut self.state),
                    steps::Choices::JikaOtorisasi=>state::steps::subs::jika_otorisasi::Core::run(&mut self.state),
                    steps::Choices::JikaBeberapaHariBerikutnyaHanya1UserYangMelapor=>state::steps::subs::jika_beberapa_hari_berikutnya_hanya1_user_yang_melapor::Core::run(&mut self.state),
                    steps::Choices::MakaBeberapaHariBerikutnyaSuratTugasMerah=>state::steps::subs::maka_beberapa_hari_berikutnya_surat_tugas_merah::Core::run(&mut self.state),
                    steps::Choices::JikaUserLainIkutMelapor=>state::steps::subs::jika_user_lain_ikut_melapor::Core::run(&mut self.state),
                    steps::Choices::MakaSuratTugasKuning=>state::steps::subs::maka_surat_tugas_kuning::Core::run(&mut self.state),
                    steps::Choices::JikaOtorisasiLaporan=>state::steps::subs::jika_otorisasi_laporan::Core::run(&mut self.state),
                    steps::Choices::MakaSuratTugasHijau=>state::steps::subs::maka_surat_tugas_hijau::Core::run(&mut self.state),
                  }
                };
              }
            }
            pub mod background{
              #[derive(Clone, Debug, Eq, PartialEq)] pub enum Choices{
                TanggalMulai(subs::TanggalMulai),
                UserAdanya(subs::UserAdanya),
                UserDenganPeran(subs::UserDenganPeran),
                StBerlajur(subs::StBerlajur),
                StDenganTanggal(subs::StDenganTanggal),
                StDenganDurasi(subs::StDenganDurasi),
                StDenganMenugaskan(subs::StDenganMenugaskan),
              }
              pub mod subs{
                #[derive(Clone, Debug, Eq, PartialEq)] pub struct TanggalMulai(pub (usize, u8, u8,));
                #[derive(Clone, Debug, Eq, PartialEq)] pub struct UserAdanya(pub usize);
                #[derive(Clone, Debug, Eq, PartialEq)] pub struct UserDenganPeran(pub Vec<()/*TODO*//*crate::tester::abstracts::subs::specifically_at_client::state::state::server::users::peran::Core*/>);
                #[derive(Clone, Debug, Eq, PartialEq)] pub struct StBerlajur(pub Vec<String>);
                #[derive(Clone, Debug, Eq, PartialEq)] pub struct StDenganTanggal(pub (usize, u8, u8));
                #[derive(Clone, Debug, Eq, PartialEq)] pub struct StDenganDurasi(pub usize);
                #[derive(Clone, Debug, Eq, PartialEq)] pub struct StDenganMenugaskan(pub Vec<String>);
              }
            }
            pub mod steps{
              #[derive(Clone, Debug, Eq, PartialEq)] pub enum Choices{
                JikaBuatSuratTugas,
                JikaOtorisasi,
                JikaBeberapaHariBerikutnyaHanya1UserYangMelapor,
                MakaBeberapaHariBerikutnyaSuratTugasMerah,
                JikaUserLainIkutMelapor,
                MakaSuratTugasKuning,
                JikaOtorisasiLaporan,
                MakaSuratTugasHijau,
              }
            }
            pub mod state{
              pub trait Core : 
                  background::Core
                + steps::Core
              {}
              pub mod background{
                pub trait Core : 
                  subs::tanggal_mulai::Core
                + subs::user_adanya::Core
                + subs::user_dengan_peran::Core
                + subs::st_berlajur::Core
                + subs::st_dengan_tanggal::Core
                + subs::st_dengan_durasi::Core
                + subs::st_dengan_menugaskan::Core{}
                pub mod subs{
                  pub mod tanggal_mulai{ pub trait Core{ fn run(&mut self, v: &super::super::super::super::background::subs::TanggalMulai); }}
                  pub mod user_adanya{ pub trait Core{ fn run(&mut self, v: &super::super::super::super::background::subs::UserAdanya); }}
                  pub mod user_dengan_peran{ pub trait Core{ fn run(&mut self, v: &super::super::super::super::background::subs::UserDenganPeran); }}
                  pub mod st_berlajur{ pub trait Core{ fn run(&mut self, v: &super::super::super::super::background::subs::StBerlajur); }}
                  pub mod st_dengan_tanggal{ pub trait Core{ fn run(&mut self, v: &super::super::super::super::background::subs::StDenganTanggal); }}
                  pub mod st_dengan_durasi{ pub trait Core{ fn run(&mut self, v: &super::super::super::super::background::subs::StDenganDurasi); }}
                  pub mod st_dengan_menugaskan{ pub trait Core{ fn run(&mut self, v: &super::super::super::super::background::subs::StDenganMenugaskan); }}
                }
              }
              pub mod steps{
                pub trait Core :
                    subs::jika_buat_surat_tugas::Core
                  + subs::jika_otorisasi::Core
                  + subs::jika_beberapa_hari_berikutnya_hanya1_user_yang_melapor::Core
                  + subs::maka_beberapa_hari_berikutnya_surat_tugas_merah::Core
                  + subs::jika_user_lain_ikut_melapor::Core
                  + subs::maka_surat_tugas_kuning::Core
                  + subs::jika_otorisasi_laporan::Core
                  + subs::maka_surat_tugas_hijau::Core{}
                pub mod subs{
                  pub mod jika_buat_surat_tugas{ pub trait Core{ fn run(&mut self); }}
                  pub mod jika_otorisasi{ pub trait Core{ fn run(&mut self); }}
                  pub mod jika_beberapa_hari_berikutnya_hanya1_user_yang_melapor{ pub trait Core{ fn run(&mut self); }}
                  pub mod maka_beberapa_hari_berikutnya_surat_tugas_merah{ pub trait Core{ fn run(&mut self); }}
                  pub mod jika_user_lain_ikut_melapor{ pub trait Core{ fn run(&mut self); }}
                  pub mod maka_surat_tugas_kuning{ pub trait Core{ fn run(&mut self); }}
                  pub mod jika_otorisasi_laporan{ pub trait Core{ fn run(&mut self); }}
                  pub mod maka_surat_tugas_hijau{ pub trait Core{ fn run(&mut self); }}
                }
              }
            }
          }
          pub mod implmnts{
            pub mod components{
              pub mod run_in_context_of{
                pub mod abstracts{
                  pub struct Core<TState: state::core::core::Core>{
                    pub state: TState,
                  }
                  pub mod state{
                    pub mod core{
                      pub mod core{
                        pub trait Core{ fn run<TState: crate::test::testers::common::sut::core::client::abstracts::Core, TFn: Fn(&mut TState)>(&mut self, context: &context::Core, fns: &TFn); }
                        pub mod context{ pub enum Core{
                          PembuatSuratTugas,
                          Otorisator,
                          PelaporPertama,
                          PelaporLainSelainYangPertama,
                          Semuanya,
                        }}
                      }
                      pub mod subs{}
                    }
                    pub mod subs{
                      pub mod selaku{}
                      pub mod specifically_at_client{}
                    }
                  }
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::Core for Core<TState>{}
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::background::Core for Core<TState>{}
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::background::subs::tanggal_mulai::Core for Core<TState>{
                    fn run(&mut self, v: &crate::test::testers::features::core::implmnts::components::step::abstracts::background::subs::TanggalMulai){
                      state::core::subs::system::server::tanggal::set(&mut self, v);
                    }
                  }
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::background::subs::user_adanya::Core for Core<TState>{
                    fn run(&mut self, v: &crate::test::testers::features::core::implmnts::components::step::abstracts::background::subs::UserAdanya){
                      state::core::subs::system::server::users::add(&mut self, v);
                    }
                  }
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::background::subs::user_dengan_peran::Core for Core<TState>{
                    fn run(&mut self, v: &crate::test::testers::features::core::implmnts::components::step::abstracts::background::subs::UserDenganPeran){
                      state::core::subs::system::server::users::peran::set(&mut self, v);
                    }
                  }
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::background::subs::st_berlajur::Core for Core<TState>{
                    fn run(&mut self, v: &crate::test::testers::features::core::implmnts::components::step::abstracts::background::subs::StBerlajur){
                      state::core::subs::system::server::st::lajur::set(&mut self, v);
                    }
                  }
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::background::subs::st_dengan_tanggal::Core for Core<TState>{
                    fn run(&mut self, v: &crate::test::testers::features::core::implmnts::components::step::abstracts::background::subs::StDenganTanggal){
                      state::core::subs::st::dengan::tanggal::set(self.state, v);
                    }
                  }
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::background::subs::st_dengan_durasi::Core for Core<TState>{
                    fn run(&mut self, v: &crate::test::testers::features::core::implmnts::components::step::abstracts::background::subs::StDenganDurasi){
                      state::core::subs::st::dengan::durasi::set(self.state, v);
                    }
                  }
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::background::subs::st_dengan_menugaskan::Core for Core<TState>{
                    fn run(&mut self, v: &crate::test::testers::features::core::implmnts::components::step::abstracts::background::subs::StDenganMenugaskan){
                      state::core::subs::st::dengan::menugaskan::set(self.state, v);
                    }
                  }
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::steps::Core for Core<TState>{}
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::steps::subs::jika_buat_surat_tugas::Core for Core<TState>{
                    fn run(&mut self){
                      state::core::core::Core::run(&mut self.state, &state::core::core::context::Core::PembuatSuratTugas, &|v|{
                        crate::test::testers::common::sut::core::client::abstracts::pages::home::visit::Core::run(v);
                        crate::test::testers::common::sut::core::client::abstracts::pages::home::button_st::input_click::Core::run(v);
                        crate::test::testers::common::sut::core::client::abstracts::pages::home_st::button_buat::input_click::Core::run(v);
                        state::core::subs::helpers::steps::pages::home_st_buat::menugaskan::Core::run(v, |v, n|{
                          crate::test::testers::common::sut::core::client::abstracts::pages::home_st_buat::bagian_menugaskan::button_tambah::input_click::Core::run(v);
                          crate::test::testers::common::sut::core::client::abstracts::pages::home_st_buat_menugaskan::table::baris::button_tambah::input_click::Core::run(v, n);
                        });
                        crate::test::testers::common::sut::core::client::abstracts::pages::home_st_buat::bagian_tanggal::bagian_tanggal::input_isi::Core::run(v, state::core::subs::st::dengan::tanggal::tanggal::get(v));
                        crate::test::testers::common::sut::core::client::abstracts::pages::home_st_buat::bagian_tanggal::bagian_bulan::input_isi::Core::run(v, state::core::subs::st::dengan::tanggal::bulan::get(v));
                        crate::test::testers::common::sut::core::client::abstracts::pages::home_st_buat::bagian_tanggal::bagian_tahun::input_isi::Core::run(v, state::core::subs::st::dengan::tanggal::tahun::get(v));
                        crate::test::testers::common::sut::core::client::abstracts::pages::home_st_buat::bagian_durasi::input_isi::Core::run(v, state::core::subs::st::dengan::durasi::get(v));
                        crate::test::testers::common::sut::core::client::abstracts::pages::home_st_buat::button_ok::input_click::Core::run(v);
                        crate::test::testers::common::sut::core::client::abstracts::pages::home_st_buat_konfirmasi::button_ok::input_click::Core::run(v);
                        crate::test::testers::common::sut::core::client::abstracts::pages::home_st_buat_informasi::button_ok::input_click::Core::run(v);
                      });
                    }
                  }
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::steps::subs::jika_otorisasi::Core for Core<TState>{
                    fn run(&mut self){
                      state::core::core::Core::run(&mut self.state, &state::core::core::context::Core::Otorisator, &|v|{
                        crate::test::testers::common::sut::core::client::abstracts::pages::home::visit::Core::run(v);
                        crate::test::testers::common::sut::core::client::abstracts::pages::home::button_st::input_click::Core::run(v);
                        { let n = state::core::subs::helpers::steps::pages::home_st::target::Core::run(v); crate::test::testers::common::sut::core::client::abstracts::pages::home_st::table::baris::button_otorisasi::input_click::Core::run(v, n); };
                        crate::test::testers::common::sut::core::client::abstracts::pages::home_st_otorisasi::bagian_status::radio_setujui::input_check::Core::run(v);
                        crate::test::testers::common::sut::core::client::abstracts::pages::home_st_otorisasi::button_ok::input_click::Core::run(v);
                        crate::test::testers::common::sut::core::client::abstracts::pages::home_st_otorisasi_konfirmasi::button_ok::input_click::Core::run(v);
                        crate::test::testers::common::sut::core::client::abstracts::pages::home_st_otorisasi_informasi::button_ok::input_click::Core::run(v);
                      });
                    }
                  }
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::steps::subs::jika_beberapa_hari_berikutnya_hanya1_user_yang_melapor::Core for Core<TState>{
                    fn run(&mut self){
                      state::core::subs::helpers::background::tanggal::majukan::ke_hari_h::Core::run(v);
                      state::core::subs::helpers::steps::specific::lapor::Core::run(v, &state::core::core::context::Core::PelaporPertama);
                      // state::core::core::Core::run(&mut self.state, &state::core::context::Core::PelaporPertama, &|v|{
                      //   crate::test::testers::common::sut::core::client::abstracts::pages::home::visit::Core::run(v);
                      //   crate::test::testers::common::sut::core::client::abstracts::pages::home::button_st::input_click::Core::run(v);
                      //   { let n = state::subs::helpers::pages::home_st::target::Core::run(v); crate::test::testers::common::sut::core::client::abstracts::pages::home_st::table::baris::button_lapor::input_click::Core::run(v, n); };
                      //   state::subs::helpers::pages::home_st_lapor::isi_laporan::Core::run(v);
                      //   crate::test::testers::common::sut::core::client::abstracts::pages::home_st_lapor::button_ok::input_click::Core::run(v);
                      //   crate::test::testers::common::sut::core::client::abstracts::pages::home_st_lapor_konfirmasi::button_ok::input_click::Core::run(v);
                      //   crate::test::testers::common::sut::core::client::abstracts::pages::home_st_lapor_informasi::button_ok::input_click::Core::run(v);
                      // });
                    }
                  }
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::steps::subs::maka_beberapa_hari_berikutnya_surat_tugas_merah::Core for Core<TState>{
                    fn run(&mut self){
                      state::core::subs::helpers::steps::specific::cek_warna::Core::run(v, MERAH);
                      // state::subs::helpers::background::tanggal::majukan::ke_hari_telat::Core::run(v);
                      // state::core::core::Core::run(&mut self.state, &state::core::context::Core::Semuanya, &|v|{
                      //   crate::test::testers::common::sut::core::client::abstracts::pages::home::visit::Core::run(v);
                      //   crate::test::testers::common::sut::core::client::abstracts::pages::home::button_st::input_click::Core::run(v);
                      //   { 
                      //     let ret = state::subs::helpers::pages::home_st::target::Core::run(v);
                      //     let ret = crate::test::testers::common::sut::core::client::abstracts::pages::home_st::table::baris::get::Core::run(v, ret);
                      //     let ret = ret.warna;
                      //     assert_eq!(ret, MERAH);
                      //   };
                      // });
                    }
                  }
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::steps::subs::jika_user_lain_ikut_melapor::Core for Core<TState>{
                    fn run(&mut self){
                      state::core::subs::helpers::steps::specific::lapor::Core::run(v, &state::core::core::context::Core::PelaporLainSelainYangPertama);
                    }
                  }
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::steps::subs::maka_surat_tugas_kuning::Core for Core<TState>{
                    fn run(&mut self){
                      state::core::subs::helpers::steps::specific::cek_warna::Core::run(v, KUNING);
                    }
                  }
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::steps::subs::jika_otorisasi_laporan::Core for Core<TState>{
                    fn run(&mut self){
                      state::core::core::Core::run(&mut self.state, &state::core::core::context::Core::Otorisator, &|v|{
                        crate::test::testers::common::sut::core::client::abstracts::pages::home::visit::Core::run(v);
                        crate::test::testers::common::sut::core::client::abstracts::pages::home::button_st::input_click::Core::run(v);
                        { let n = state::subs::helpers::steps::pages::home_st::target::Core::run(v); crate::test::testers::common::sut::core::client::abstracts::pages::home_st::table::baris::button_otorisasi::input_click::Core::run(v, n); };
                        crate::test::testers::common::sut::core::client::abstracts::pages::home_st_otorisasilaporan::bagian_status::radio_setuju::input_click::Core::run(v);
                        crate::test::testers::common::sut::core::client::abstracts::pages::home_st_otorisasilaporan::button_ok::input_click::Core::run(v);
                        crate::test::testers::common::sut::core::client::abstracts::pages::home_st_otorisasilaporan_konfirmasi::button_ok::input_click::Core::run(v);
                        crate::test::testers::common::sut::core::client::abstracts::pages::home_st_otorisasilaporan_informasi::button_ok::input_click::Core::run(v);
                      });
                    }
                  }
                  impl<TState: state::core::core::Core> crate::test::testers::features::core::implmnts::components::step::abstracts::state::steps::subs::maka_surat_tugas_hijau::Core for Core<TState>{
                    fn run(&mut self){
                      state::core::subs::helpers::steps::specific::cek_warna::Core::run(v, HIJAU);
                    }
                  }
                }
                pub mod implmnts{
                  pub mod collection_of_clients_and_single_server{
                    pub struct Core;
                    impl crate::test::testers::features::core::implmnts::components::step::implmnts::components::run_in_context_of::abstracts::state::core::core::Core for Core{
                      fn run<TState: crate::test::testers::common::sut::core::client::abstracts::Core, TFn: Fn(&mut TState)>(&mut self, context: &crate::test::testers::features::core::implmnts::components::step::implmnts::components::run_in_context_of::abstracts::state::core::core::context::Core, fns: &TFn){
                        use crate::test::testers::features::core::implmnts::components::step::implmnts::components::run_in_context_of;
                        fn pembuatSuratTugas(x: &mut Self, fns: &TFn){
                          run_in_context_of::abstracts::state::subs::selaku::Core::run(x, PembuatSuratTugas, x.test.users.peran.pembuatSuratTugas, fns);
                        }
                        fn otorisator(x: &mut Self, fns: &TFn){
                          x.test.st.lajur.iter().for_each(|v|{ run_in_context_of::abstracts::state::subs::selaku::Core::run(x, Otorisator, v, fns)});
                        }
                        match context{
                          run_in_context_of::abstracts::state::core::core::context::Core::PembuatSuratTugas=> pembuatSuratTugas(&mut self, fns),
                          run_in_context_of::abstracts::state::core::core::context::Core::Otorisator=>otorisator(&mut self, fns),
                          run_in_context_of::abstracts::state::core::core::context::Core::PelaporPertama=>{
                            let n = self.test.st.target.menugaskan.iter().nth(0).unwrap();
                            run_in_context_of::abstracts::state::subs::selaku::Core::run(self, Ditugaskan, n, fns);
                          },
                          run_in_context_of::abstracts::state::core::core::context::Core::PelaporLainSelainYangPertama=>{
                            self.test.st.target.menugaskan.iter().enumerate().for_each(|v| if v.0!=0 { run_in_context_of::abstracts::state::subs::selaku::Core::run(self, Ditugaskan, v.1, fns) });
                          },
                          run_in_context_of::abstracts::state::core::core::context::Core::Semuanya=>{
                            pembuatSuratTugas(self, fns);
                            otorisator(self, fns);
                            self.test.st.target.menugaskan.iter().for_each(|v| run_in_context_of::abstracts::state::subs::selaku::Core::run(self, Ditugaskan, v, fns));
                          },
                        }
                      }
                    }
                    impl crate::test::testers::features::core::implmnts::components::step::implmnts::components::run_in_context_of::abstracts::state::subs::selaku::Core for Core{
                      fn run(&mut self, context: (/*todo*/), username: (/*todo*/), fns: (/*todo*/)){
                        crate::test::testers::features::core::implmnts::components::step::implmnts::components::run_in_context_of::abstracts::state::subs::specifically_at_client::Core::run(self, username, fns)
                      }
                    }
                    impl crate::test::testers::features::core::implmnts::components::step::implmnts::components::run_in_context_of::abstracts::state::subs::specifically_at_client::Core for Core{
                      fn run(&mut self, username: (/*todo*/), fns: (/*todo*/)){
                        let ret = self.state;
                        let ret = (self.convert_to)(ret);
                        let ret = { fns(ret); ret };
                        let ret = (self.convert_from)(ret);
                        self.state = ret;
                      }
                    }
                  }
                }
              }
            }
            pub mod composite{
              pub mod defaults{}
            }
          }
        }}
        pub mod composite{
          pub mod defaults{ pub fn core(){
            todo!()
          }}
        }
      }
    }}
  }
  pub mod suites{ 
    #[test]
    fn test(){
      /*crate::test::testers::features::core::implmnts::composite::defaults::core().run();*/todo!()
    }
  }
}

fn main(){
  todo!()
}