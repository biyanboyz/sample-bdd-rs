pub mod default{
  pub mod flow{
    use super::super::super::abstracts;
    use crate::test::tester;
    use crate::test::tester::scenarios::main::implmnts::{with_step_runner, with_step_runner::implmnts::parts::helperables::abstracts as helperables};
    use crate::core::app::client::abstracts::subs::pages;
    pub struct Core<
      T: abstracts::Core<Ret=T, State=TTState>, 
      TTState: crate::test::tester::scenarios::main::implmnts::with_step_runner::implmnts::parts::helperables::abstracts::Core<Ret=TTState, State=TTStateState>,
      TTStateState: crate::core::app::client::abstracts::Core<Ret=TTStateState>,
      THolder,
      TIn:   with_step_runner::abstracts::subs::steps::dengan::user_yang_terdaftar_berjumlah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::yang_berwenang_sebagai_pembuat_surat_tugas_adalah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::alur_otorisasi_berupa::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::jika::sekarang_tanggal::Core<Ret=TIn>,
      TInFnAccessGet: Fn(T)->(THolder, TIn),
      TInFnAccessSet: Fn(THolder, TIn)->T,
    >(pub T, pub TInFnAccessGet, pub TInFnAccessSet);
    impl<
      T: abstracts::Core<Ret=T, State=TTState>, 
      TTState: crate::test::tester::scenarios::main::implmnts::with_step_runner::implmnts::parts::helperables::abstracts::Core<Ret=TTState, State=TTStateState>,
      TTStateState: crate::core::app::client::abstracts::Core<Ret=TTStateState>,
      THolder,
      TIn:   with_step_runner::abstracts::subs::steps::dengan::user_yang_terdaftar_berjumlah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::yang_berwenang_sebagai_pembuat_surat_tugas_adalah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::alur_otorisasi_berupa::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::jika::sekarang_tanggal::Core<Ret=TIn>,
      TInFnAccessGet: Fn(T)->(THolder, TIn),
      TInFnAccessSet: Fn(THolder, TIn)->T,
    > with_step_runner::abstracts::Core for Core<T, TTState, TTStateState, THolder, TIn, TInFnAccessGet, TInFnAccessSet>{
      type Ret = Self;
    }
    impl<
      T: abstracts::Core<Ret=T, State=TTState>, 
      TTState: crate::test::tester::scenarios::main::implmnts::with_step_runner::implmnts::parts::helperables::abstracts::Core<Ret=TTState, State=TTStateState>,
      TTStateState: crate::core::app::client::abstracts::Core<Ret=TTStateState>,
      THolder,
      TIn:   with_step_runner::abstracts::subs::steps::dengan::user_yang_terdaftar_berjumlah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::yang_berwenang_sebagai_pembuat_surat_tugas_adalah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::alur_otorisasi_berupa::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::jika::sekarang_tanggal::Core<Ret=TIn>,
      TInFnAccessGet: Fn(T)->(THolder, TIn),
      TInFnAccessSet: Fn(THolder, TIn)->T,
    > with_step_runner::abstracts::subs::steps::dengan::user_yang_terdaftar_berjumlah::Core for Core<T, TTState, TTStateState, THolder, TIn, TInFnAccessGet, TInFnAccessSet>{
      type Ret = Self;
      fn run(x: Self::Ret, val: usize)->Result<Self::Ret, tester::common::Error>{
        let ret = x.1(x.0);
        let ret = (ret.0, <TIn as with_step_runner::abstracts::subs::steps::dengan::user_yang_terdaftar_berjumlah::Core>::run(ret.1, val));
        let ret = match ret.1{ Ok(ret0)=>Ok(x.2(ret.0, ret0)), Err(ret)=>Err(ret) };
        let ret = match ret{ Ok(ret0)=>Ok(Self(ret0, x.1, x.2)), Err(ret)=>Err(ret) };
        ret
      }
    }
    impl<
      T: abstracts::Core<Ret=T, State=TTState>, 
      TTState: crate::test::tester::scenarios::main::implmnts::with_step_runner::implmnts::parts::helperables::abstracts::Core<Ret=TTState, State=TTStateState>,
      TTStateState: crate::core::app::client::abstracts::Core<Ret=TTStateState>,
      THolder,
      TIn:   with_step_runner::abstracts::subs::steps::dengan::user_yang_terdaftar_berjumlah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::yang_berwenang_sebagai_pembuat_surat_tugas_adalah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::alur_otorisasi_berupa::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::jika::sekarang_tanggal::Core<Ret=TIn>,
      TInFnAccessGet: Fn(T)->(THolder, TIn),
      TInFnAccessSet: Fn(THolder, TIn)->T,
    > with_step_runner::abstracts::subs::steps::dengan::yang_berwenang_sebagai_pembuat_surat_tugas_adalah::Core  for Core<T, TTState, TTStateState, THolder, TIn, TInFnAccessGet, TInFnAccessSet>{
      type Ret = Self;
      fn run(x: Self::Ret, val: String)->Result<Self::Ret, tester::common::Error>{
        let ret = x.1(x.0);
        let ret = (ret.0, <TIn as with_step_runner::abstracts::subs::steps::dengan::yang_berwenang_sebagai_pembuat_surat_tugas_adalah::Core>::run(ret.1, val));
        let ret = match ret.1{ Ok(ret0)=>Ok(x.2(ret.0, ret0)), Err(ret)=>Err(ret) };
        let ret = match ret{ Ok(ret0)=>Ok(Self(ret0, x.1, x.2)), Err(ret)=>Err(ret) };
        ret
      }
    }
    impl<
      T: abstracts::Core<Ret=T, State=TTState>, 
      TTState: crate::test::tester::scenarios::main::implmnts::with_step_runner::implmnts::parts::helperables::abstracts::Core<Ret=TTState, State=TTStateState>,
      TTStateState: crate::core::app::client::abstracts::Core<Ret=TTStateState>,
      THolder,
      TIn:   with_step_runner::abstracts::subs::steps::dengan::user_yang_terdaftar_berjumlah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::yang_berwenang_sebagai_pembuat_surat_tugas_adalah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::alur_otorisasi_berupa::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::jika::sekarang_tanggal::Core<Ret=TIn>,
      TInFnAccessGet: Fn(T)->(THolder, TIn),
      TInFnAccessSet: Fn(THolder, TIn)->T,
    > with_step_runner::abstracts::subs::steps::dengan::alur_otorisasi_berupa::Core  for Core<T, TTState, TTStateState, THolder, TIn, TInFnAccessGet, TInFnAccessSet>{
      type Ret = Self;
      fn run<TVal: Iterator<Item=String>>(x: Self::Ret, val: TVal)->Result<Self::Ret, tester::common::Error>{
        let ret = x.1(x.0);
        let ret = (ret.0, <TIn as with_step_runner::abstracts::subs::steps::dengan::alur_otorisasi_berupa::Core>::run(ret.1, val));
        let ret = match ret.1{ Ok(ret0)=>Ok(x.2(ret.0, ret0)), Err(ret)=>Err(ret) };
        let ret = match ret{ Ok(ret0)=>Ok(Self(ret0, x.1, x.2)), Err(ret)=>Err(ret) };
        ret
      }
    }
    impl<
      T: abstracts::Core<Ret=T, State=TTState>, 
      TTState: crate::test::tester::scenarios::main::implmnts::with_step_runner::implmnts::parts::helperables::abstracts::Core<Ret=TTState, State=TTStateState>,
      TTStateState: crate::core::app::client::abstracts::Core<Ret=TTStateState>,
      THolder,
      TIn:   with_step_runner::abstracts::subs::steps::dengan::user_yang_terdaftar_berjumlah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::yang_berwenang_sebagai_pembuat_surat_tugas_adalah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::alur_otorisasi_berupa::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::jika::sekarang_tanggal::Core<Ret=TIn>,
      TInFnAccessGet: Fn(T)->(THolder, TIn),
      TInFnAccessSet: Fn(THolder, TIn)->T,
    > with_step_runner::abstracts::subs::steps::jika::buat_surat_tugas::Core  for Core<T, TTState, TTStateState, THolder, TIn, TInFnAccessGet, TInFnAccessSet>{
      type Ret = Self;
      fn run(x: Self::Ret, val: with_step_runner::abstracts::subs::steps::jika::buat_surat_tugas::Input)->Result<Self::Ret, tester::common::Error>{
        let ret = Ok(x.0);
        let ret = {
          let ret = match ret{ Ok(ret)=>T::run(ret, &abstracts::subs::Target::PembuatSuratTugas, &|x|{
            let ret = Ok(x);
            let ret = {
              let ret = match ret{ Ok(ret)=><T::State as helperables::subs::client::directly::action::Core>::run(ret, |x|{
                let ret = Ok(x);
                let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home::visit::Core>::run(ret), Err(x)=>Err(x) };
                let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home::button_st::input_click::Core>::run(ret), Err(x)=>Err(x) };
                let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st::button_buat::input_click::Core>::run(ret), Err(x)=>Err(x) };
                let ret = match ret{ Ok(ret)=>Ok((ret, ())), Err(x)=>Err(x) };
                ret
              }), Err(x)=>Err(x) };
              let ret = match ret{ Ok(ret)=>Ok(ret.0), Err(x)=>Err(x) };
              ret
            };
            let ret = match ret{ Ok(ret)=><T::State as helperables::subs::client::helpers::pages::home_st_buat::menugaskan::Core>::run(ret), Err(x)=>Err(x) };
            let ret = {
              let ret = match ret{ Ok(ret)=><T::State as helperables::subs::client::directly::action::Core>::run(ret, |x|{
                let ret = Ok(x);
                let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_buat::bagian_tanggal::bagian_tanggal::input_isi::Core>::run(ret, val.tanggal.tanggal), Err(x)=>Err(x) };
                let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_buat::bagian_tanggal::bagian_bulan::input_isi::Core>::run(ret, val.tanggal.bulan), Err(x)=>Err(x) };
                let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_buat::bagian_tanggal::bagian_tahun::input_isi::Core>::run(ret, val.tanggal.tahun), Err(x)=>Err(x) };
                let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_buat::bagian_durasi::input_isi::Core>::run(ret, val.durasi), Err(x)=>Err(x) };
                let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_buat::button_ok::input_click::Core>::run(ret), Err(x)=>Err(x) };
                let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_buat_informasi::button_ok::input_click::Core>::run(ret), Err(x)=>Err(x) };
                let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_buat_konfirmasi::button_ok::input_click::Core>::run(ret), Err(x)=>Err(x) };
                let ret = match ret{ Ok(ret)=>Ok((ret, ())), Err(x)=>Err(x) };
                ret
              }), Err(x)=>Err(x) };
              let ret = match ret{ Ok(ret)=>Ok(ret.0), Err(x)=>Err(x) };
              ret
            };
            ret
          }), Err(x)=>Err(x) };
          let ret = match ret{ Ok(ret)=>Ok(Self(ret, x.1, x.2)), Err(x)=>Err(x) };
          ret
        };
        ret
      }
    }
    impl<
      T: abstracts::Core<Ret=T, State=TTState>, 
      TTState: crate::test::tester::scenarios::main::implmnts::with_step_runner::implmnts::parts::helperables::abstracts::Core<Ret=TTState, State=TTStateState>,
      TTStateState: crate::core::app::client::abstracts::Core<Ret=TTStateState>,
      THolder,
      TIn:   with_step_runner::abstracts::subs::steps::dengan::user_yang_terdaftar_berjumlah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::yang_berwenang_sebagai_pembuat_surat_tugas_adalah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::alur_otorisasi_berupa::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::jika::sekarang_tanggal::Core<Ret=TIn>,
      TInFnAccessGet: Fn(T)->(THolder, TIn),
      TInFnAccessSet: Fn(THolder, TIn)->T,
    > with_step_runner::abstracts::subs::steps::jika::sekarang_tanggal::Core  for Core<T, TTState, TTStateState, THolder, TIn, TInFnAccessGet, TInFnAccessSet>{
      type Ret = Self;
      fn run(x: Self::Ret, val: crate::common::tanggal::Core)->Result<Self::Ret, tester::common::Error>{
        let ret = x.1(x.0);
        let ret = (ret.0, <TIn as with_step_runner::abstracts::subs::steps::jika::sekarang_tanggal::Core>::run(ret.1, val));
        let ret = match ret.1{ Ok(ret0)=>Ok(x.2(ret.0, ret0)), Err(ret)=>Err(ret) };
        let ret = match ret{ Ok(ret0)=>Ok(Self(ret0, x.1, x.2)), Err(ret)=>Err(ret) };
        ret
      }
    }
    impl<
      T: abstracts::Core<Ret=T, State=TTState>, 
      TTState: crate::test::tester::scenarios::main::implmnts::with_step_runner::implmnts::parts::helperables::abstracts::Core<Ret=TTState, State=TTStateState>,
      TTStateState: crate::core::app::client::abstracts::Core<Ret=TTStateState>,
      THolder,
      TIn:   with_step_runner::abstracts::subs::steps::dengan::user_yang_terdaftar_berjumlah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::yang_berwenang_sebagai_pembuat_surat_tugas_adalah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::alur_otorisasi_berupa::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::jika::sekarang_tanggal::Core<Ret=TIn>,
      TInFnAccessGet: Fn(T)->(THolder, TIn),
      TInFnAccessSet: Fn(THolder, TIn)->T,
    > with_step_runner::abstracts::subs::steps::jika::otorisasi::Core  for Core<T, TTState, TTStateState, THolder, TIn, TInFnAccessGet, TInFnAccessSet>{
      type Ret = Self;
      fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>{
        let ret = Ok(x.0);
        let ret = match ret{ Ok(ret)=>T::run(ret, &abstracts::subs::Target::Otorisator, &|x|{
          let ret = Ok(x);
          let ret = {
            let ret = match ret{ Ok(ret)=><T::State as helperables::subs::client::directly::action::Core>::run(ret, |x|{
              let ret = Ok(x);
              let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home::visit::Core>::run(ret), Err(x)=>Err(x) };
              let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home::button_st::input_click::Core>::run(ret), Err(x)=>Err(x) };
              let ret = match ret{ Ok(ret)=>Ok((ret, ())), Err(x)=>Err(x) };
              ret
            }), Err(x)=>Err(x) };
            let ret = match ret{ Ok(ret)=>Ok(ret.0), Err(x)=>Err(x) };
            ret
          };
          let ret = {
            let ret = match ret{ Ok(ret)=><T::State as helperables::subs::client::helpers::pages::home_st::pilih::Core>::run(ret, |x, index|{
              let ret = Ok(x);
              let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st::table::baris::kolom_tindakan::button_otorisasi::input_click::Core>::run(ret, index), Err(x)=>Err(x) };
              let ret = match ret{ Ok(ret)=>Ok((ret, ())), Err(x)=>Err(x) };
              ret
            }), Err(x)=>Err(x) };
            let ret = match ret{ Ok(ret)=>Ok(ret.0), Err(x)=>Err(x) };
            ret
          };
          let ret = match ret{ Ok(ret)=><T::State as helperables::subs::client::directly::action::Core>::run(ret, |x|{
            let ret = Ok(x);
            let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_otorisasi::bagian_status::radio_setujui::input_click::Core>::run(ret), Err(x)=>Err(x) };
            let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_otorisasi::button_ok::input_click::Core>::run(ret), Err(x)=>Err(x) };
            let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_otorisasi_konfirmasi::button_ok::input_click::Core>::run(ret), Err(x)=>Err(x) };
            let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_otorisasi_informasi::button_ok::input_click::Core>::run(ret), Err(x)=>Err(x) };
            let ret = match ret{ Ok(ret)=>Ok((ret, ())), Err(x)=>Err(x) };
            ret
          }), Err(x)=>Err(x) };
          let ret = match ret{ Ok(ret)=>Ok(ret.0), Err(x)=>Err(x) };
          ret
        }), Err(x)=>Err(x) };
        let ret = match ret{ Ok(ret0)=>Ok(Self(ret0, x.1, x.2)), Err(x)=>Err(x) };
        ret
      }
    }
    impl<
      T: abstracts::Core<Ret=T, State=TTState>, 
      TTState: crate::test::tester::scenarios::main::implmnts::with_step_runner::implmnts::parts::helperables::abstracts::Core<Ret=TTState, State=TTStateState>,
      TTStateState: crate::core::app::client::abstracts::Core<Ret=TTStateState>,
      THolder,
      TIn:   with_step_runner::abstracts::subs::steps::dengan::user_yang_terdaftar_berjumlah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::yang_berwenang_sebagai_pembuat_surat_tugas_adalah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::alur_otorisasi_berupa::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::jika::sekarang_tanggal::Core<Ret=TIn>,
      TInFnAccessGet: Fn(T)->(THolder, TIn),
      TInFnAccessSet: Fn(THolder, TIn)->T,
    > with_step_runner::abstracts::subs::steps::jika::user_laporan::Core  for Core<T, TTState, TTStateState, THolder, TIn, TInFnAccessGet, TInFnAccessSet>{
      type Ret = Self;
      fn run(x: Self::Ret, val: String)->Result<Self::Ret, tester::common::Error>{
        let ret = Ok(x.0);
        let ret = match ret{ Ok(ret)=>T::run(ret, &abstracts::subs::Target::User(val), &|x|{
          let ret = Ok(x);
          let ret = {
            let ret = match ret{ Ok(ret)=><T::State as helperables::subs::client::directly::action::Core>::run(ret, |x|{
              let ret = Ok(x);
              let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home::visit::Core>::run(ret), Err(x)=>Err(x) };
              let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home::button_st::input_click::Core>::run(ret), Err(x)=>Err(x) };
              let ret = match ret{ Ok(ret)=>Ok((ret, ())), Err(x)=>Err(x) };
              ret
            }), Err(x)=>Err(x) };
            let ret = match ret{ Ok(ret)=>Ok(ret.0), Err(x)=>Err(x) };
            ret
          };
          let ret = {
            let ret = match ret{ Ok(ret)=><T::State as helperables::subs::client::helpers::pages::home_st::pilih::Core>::run(ret, |x, index|{
              let ret = Ok(x);
              let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st::table::baris::kolom_tindakan::button_lapor::input_click::Core>::run(ret, index), Err(x)=>Err(x) };
              let ret = match ret{ Ok(ret)=>Ok((ret, ())), Err(x)=>Err(x) };
              ret
            }), Err(x)=>Err(x) };
            let ret = match ret{ Ok(ret)=>Ok(ret.0), Err(x)=>Err(x) };
            ret
          };
          let ret = {
            let ret = match ret{ Ok(ret)=><T::State as helperables::subs::client::directly::action::Core>::run(ret, |x|{
              let ret = Ok(x);
              let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_lapor::bagian_dokumen::input_isi::Core>::run(ret, String::from("Test")), Err(x)=>Err(x) };
              let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_lapor::button_ok::input_click::Core>::run(ret), Err(x)=>Err(x) };
              let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_lapor_konfirmasi::button_ok::input_click::Core>::run(ret), Err(x)=>Err(x) };
              let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_lapor_informasi::button_ok::input_click::Core>::run(ret), Err(x)=>Err(x) };
              let ret = match ret{ Ok(ret)=>Ok((ret, ())), Err(x)=>Err(x) };
              ret
            }), Err(x)=>Err(x) };
            let ret = match ret{ Ok(ret)=>Ok(ret.0), Err(x)=>Err(x) };
            ret
          };
          ret
        }), Err(x)=>Err(x) };
        let ret = match ret{ Ok(ret0)=>Ok(Self(ret0, x.1, x.2)), Err(x)=>Err(x) };
        ret
      }
    }
    impl<
      T: abstracts::Core<Ret=T, State=TTState>, 
      TTState: crate::test::tester::scenarios::main::implmnts::with_step_runner::implmnts::parts::helperables::abstracts::Core<Ret=TTState, State=TTStateState>,
      TTStateState: crate::core::app::client::abstracts::Core<Ret=TTStateState>,
      THolder,
      TIn:   with_step_runner::abstracts::subs::steps::dengan::user_yang_terdaftar_berjumlah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::yang_berwenang_sebagai_pembuat_surat_tugas_adalah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::alur_otorisasi_berupa::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::jika::sekarang_tanggal::Core<Ret=TIn>,
      TInFnAccessGet: Fn(T)->(THolder, TIn),
      TInFnAccessSet: Fn(THolder, TIn)->T,
    > with_step_runner::abstracts::subs::steps::jika::otorisasi_laporan::Core  for Core<T, TTState, TTStateState, THolder, TIn, TInFnAccessGet, TInFnAccessSet>{
      type Ret = Self;
      fn run(x: Self::Ret)->Result<Self::Ret, tester::common::Error>{
        let ret = Ok(x.0);
        let ret = match ret{ Ok(ret)=>T::run(ret, &abstracts::subs::Target::Otorisator, &|x|{
          let ret = Ok(x);
          let ret = {
            let ret = match ret{ Ok(ret)=><T::State as helperables::subs::client::directly::action::Core>::run(ret, |x|{
              let ret = Ok(x);
              let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home::visit::Core>::run(ret), Err(x)=>Err(x) };
              let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home::button_st::input_click::Core>::run(ret), Err(x)=>Err(x) };
              let ret = match ret{ Ok(ret)=>Ok((ret, ())), Err(x)=>Err(x) };
              ret
            }), Err(x)=>Err(x) };
            let ret = match ret{ Ok(ret)=>Ok(ret.0), Err(x)=>Err(x) };
            ret
          };
          let ret = {
            let ret = match ret{ Ok(ret)=><T::State as helperables::subs::client::helpers::pages::home_st::pilih::Core>::run(ret, |x, index|{
              let ret = Ok(x);
              let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st::table::baris::kolom_tindakan::button_otorisasi_laporan::input_click::Core>::run(ret, index), Err(x)=>Err(x) };
              let ret = match ret{ Ok(ret)=>Ok((ret, ())), Err(x)=>Err(x) };
              ret
            }), Err(x)=>Err(x) };
            let ret = match ret{ Ok(ret)=>Ok(ret.0), Err(x)=>Err(x) };
            ret
          };
          let ret = {
            let ret = {
              let ret = match ret{ Ok(ret)=><T::State as helperables::subs::client::directly::action::Core>::run(ret, |x|{
                let ret = Ok(x);
                let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_otorisasi_laporan::bagian_status::radio_setujui::input_click::Core>::run(ret), Err(x)=>Err(x) };
                let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_otorisasi_laporan::button_ok::input_click::Core>::run(ret), Err(x)=>Err(x) };
                let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_otorisasi_laporan_konfirmasi::button_ok::input_click::Core>::run(ret), Err(x)=>Err(x) };
                let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st_otorisasi_laporan_informasi::button_ok::input_click::Core>::run(ret), Err(x)=>Err(x) };
                let ret = match ret{ Ok(ret)=>Ok((ret, ())), Err(x)=>Err(x) };
                ret
              }), Err(x)=>Err(x) };
              ret
            };
            let ret = match ret{ Ok(ret)=>Ok(ret.0), Err(x)=>Err(x) };
            ret
          };
          ret
        }), Err(x)=>Err(x) };
        let ret = match ret{ Ok(ret0)=>Ok(Self(ret0, x.1, x.2)), Err(x)=>Err(x) };
        ret
      }
    }
    impl<
      T: abstracts::Core<Ret=T, State=TTState>, 
      TTState: crate::test::tester::scenarios::main::implmnts::with_step_runner::implmnts::parts::helperables::abstracts::Core<Ret=TTState, State=TTStateState>,
      TTStateState: crate::core::app::client::abstracts::Core<Ret=TTStateState>,
      THolder,
      TIn:   with_step_runner::abstracts::subs::steps::dengan::user_yang_terdaftar_berjumlah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::yang_berwenang_sebagai_pembuat_surat_tugas_adalah::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::dengan::alur_otorisasi_berupa::Core<Ret=TIn>
           + with_step_runner::abstracts::subs::steps::jika::sekarang_tanggal::Core<Ret=TIn>,
      TInFnAccessGet: Fn(T)->(THolder, TIn),
      TInFnAccessSet: Fn(THolder, TIn)->T,
    > with_step_runner::abstracts::subs::steps::maka::surat_tugas_akan_berwarna::Core  for Core<T, TTState, TTStateState, THolder, TIn, TInFnAccessGet, TInFnAccessSet>{
      type Ret = Self;
      fn run(x: Self::Ret, val: &crate::core::domain::st::Warna)->Result<Self::Ret, tester::common::Error>{
        let ret = Ok(x.0);
        let ret = match ret{ Ok(ret)=>T::run(ret, &abstracts::subs::Target::Otorisator, &|x|{
          let ret = Ok(x);
          let ret = {
            let ret = match ret{ Ok(ret)=><T::State as helperables::subs::client::directly::action::Core>::run(ret, |x|{
              let ret = Ok(x);
              let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home::visit::Core>::run(ret), Err(x)=>Err(x) };
              let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home::button_st::input_click::Core>::run(ret), Err(x)=>Err(x) };
              let ret = match ret{ Ok(ret)=>Ok((ret, ())), Err(x)=>Err(x) };
              ret
            }), Err(x)=>Err(x) };
            let ret = match ret{ Ok(ret)=>Ok(ret.0), Err(x)=>Err(x) };
            ret
          };
          let ret = match ret{ Ok(ret)=><T::State as helperables::subs::client::helpers::pages::home_st::pilih::Core>::run(ret, |x, index|{
            let ret = Ok(x);
            let ret = match ret{ Ok(ret)=><<T::State as helperables::subs::client::directly::action::Core>::State as pages::home_st::table::baris::warna::get::Core>::run(ret, index), Err(x)=>Err(x) };
            ret
          }), Err(x)=>Err(x) };
          let ret = match ret{ Ok(ret)=>{
            if &ret.1 == val{ Ok(ret) } else { Err(tester::common::Error::Line(format!("{}:{}:{}", file!(), line!(), column!()))) }
          }, Err(x)=>Err(x) };
          let ret = match ret{ Ok(ret)=>Ok(ret.0), Err(x)=>Err(x) };
          ret
        }), Err(x)=>Err(x) };
        let ret = match ret{ Ok(ret0)=>Ok(Self(ret0, x.1, x.2)), Err(x)=>Err(x) };
        ret
      }
    }
  }
}