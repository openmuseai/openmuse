// Java: import android.provider.Settings.Secure
// Java: import android.view.View

// Java field: InputMethodType[] special_inputType
static SPECIAL_INPUT_TYPE: std::sync::OnceLock<Vec<InputMethodType>> = std::sync::OnceLock::new();

fn get_special_inputType() -> &'static Vec<InputMethodType> {
    SPECIAL_INPUT_TYPE.get_or_init(|| {
        vec![
            InputMethodType::InputMethodType_baiduinput_mz,
            InputMethodType::InputMethodType_baiduinput,
            InputMethodType::InputMethodType_simeji,
            InputMethodType::InputMethodType_samsung,
            InputMethodType::InputMethodType_googlepinyin,
        ]
    })
}

// Java: public enum InputMethodType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMethodType {
    InputMethodType_unknown,
    InputMethodType_swype,
    InputMethodType_gokeyboard,
    InputMethodType_swiftKey,
    InputMethodType_atok,
    InputMethodType_nfx,
    InputMethodType_googlejapanese,
    InputMethodType_baiduinput,
    InputMethodType_baidupadinput,
    InputMethodType_sogouinput,
    InputMethodType_iflytek,
    InputMethodType_simeji,
    InputMethodType_motorola,
    InputMethodType_hangulkeyboard,
    InputMethodType_diopencn,
    InputMethodType_samsung,
    InputMethodType_atokmobile,
    InputMethodType_googlekorean,
    InputMethodType_sonyericsson,
    InputMethodType_huawei,
    InputMethodType_meizu,
    InputMethodType_smartkeyboard,
    InputMethodType_qqpinyin,
    InputMethodType_chubao,
    InputMethodType_lime,
    InputMethodType_googlelatin,
    InputMethodType_slideit,
    InputMethodType_aitype,
    InputMethodType_scutgpen,
    InputMethodType_openwnneasy,
    InputMethodType_perfect,
    InputMethodType_more,
    InputMethodType_googlepinyin,
    InputMethodType_multiling,
    InputMethodType_tswipepro,
    InputMethodType_asus_xt9,
    InputMethodType_asus,
    InputMethodType_baiduinput_mz,
    InputMethodType_nokia,
    InputMethodType_ginger,
    InputMethodType_fleksy,
    InputMethodType_germanKeyboard,
    InputMethodType_baiduinput_huawei,
    InputMethodType_baiduinput_mi,
}

// Java: public enum InputMethodType
impl InputMethodType {
    // Java: InputMethodType()

    // Java: static InputMethodType getInputMethodType(View view)
    pub fn getInputMethodType(_view: Option<&dyn std::any::Any>) -> InputMethodType {
      // Java: if (null == view)
      if _view.is_none()
      {
         // Java: return InputMethodType_unknown;
         return InputMethodType::InputMethodType_unknown;
      }
      // Java: else
      else
      {
         // Java: String inputMethodManagerCurID = Secure.getString(view.getContext().getContentResolver(), "default_input_method");
         let _inputMethodManagerCurID: Option<String> = todo!();
         // Java: if (inputMethodManagerCurID == null)
         if _inputMethodManagerCurID.is_none()
         {
            // Java: return InputMethodType_unknown;
            return InputMethodType::InputMethodType_unknown;
         }
         // Java: else if (!inputMethodManagerCurID.startsWith("com.swype.android.inputmethod") && !inputMethodManagerCurID.startsWith("com.nuance.swype"))
         else if !_inputMethodManagerCurID.as_ref().unwrap().starts_with("com.swype.android.inputmethod") && !_inputMethodManagerCurID.as_ref().unwrap().starts_with("com.nuance.swype")
         {
            // Java: if (inputMethodManagerCurID.startsWith("com.nuance.swype.nokia"))
            let id = _inputMethodManagerCurID.as_ref().unwrap();
            if id.starts_with("com.nuance.swype.nokia")
            {
               // Java: return InputMethodType_nokia;
               return InputMethodType::InputMethodType_nokia;
            }
            // Java: else if (!inputMethodManagerCurID.startsWith("com.jb.gokeyboard") && !inputMethodManagerCurID.startsWith("com.jb.emoji.gokeyboard"))
            else if !id.starts_with("com.jb.gokeyboard") && !id.starts_with("com.jb.emoji.gokeyboard")
            {
               // Java: if (inputMethodManagerCurID.startsWith("com.touchtype.swiftkey"))
               if id.starts_with("com.touchtype.swiftkey")
               {
                  // Java: return InputMethodType_swiftKey;
                  return InputMethodType::InputMethodType_swiftKey;
               }
               // Java: else if (inputMethodManagerCurID.startsWith("com.justsystems.atokmobile.service"))
               else if id.starts_with("com.justsystems.atokmobile.service")
               {
                  // Java: return InputMethodType_atok;
                  return InputMethodType::InputMethodType_atok;
               }
               // Java: else if (inputMethodManagerCurID.startsWith("com.sony.nfx.inputmethod"))
               else if id.starts_with("com.sony.nfx.inputmethod")
               {
                  // Java: return InputMethodType_nfx;
                  return InputMethodType::InputMethodType_nfx;
               }
               // Java: else if (inputMethodManagerCurID.startsWith("com.google.android.inputmethod.japanese"))
               else if id.starts_with("com.google.android.inputmethod.japanese")
               {
                  // Java: return InputMethodType_googlejapanese;
                  return InputMethodType::InputMethodType_googlejapanese;
               }
               // Java: else if (inputMethodManagerCurID.startsWith("com.baidu.input"))
               else if id.starts_with("com.baidu.input")
               {
                  // Java: if (inputMethodManagerCurID.startsWith("com.baidu.input_mz"))
                  if id.starts_with("com.baidu.input_mz")
                  {
                     // Java: return InputMethodType_baiduinput_mz;
                     return InputMethodType::InputMethodType_baiduinput_mz;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.baidu.input_huawei"))
                  else if id.starts_with("com.baidu.input_huawei")
                  {
                     // Java: return InputMethodType_baiduinput_huawei;
                     return InputMethodType::InputMethodType_baiduinput_huawei;
                  }
                  // Java: else
                  else
                  {
                     // Java: return inputMethodManagerCurID.startsWith("com.baidu.input_mi") ? InputMethodType_baiduinput_mi : InputMethodType_baiduinput;
                     return if id.starts_with("com.baidu.input_mi") { InputMethodType::InputMethodType_baiduinput_mi } else { InputMethodType::InputMethodType_baiduinput };
                  }
               }
               // Java: else if (inputMethodManagerCurID.startsWith("com.baidu.padinput"))
               else if id.starts_with("com.baidu.padinput")
               {
                  // Java: return InputMethodType_baidupadinput;
                  return InputMethodType::InputMethodType_baidupadinput;
               }
               // Java: else if (inputMethodManagerCurID.startsWith("com.sohu.inputmethod.sogou"))
               else if id.starts_with("com.sohu.inputmethod.sogou")
               {
                  // Java: return InputMethodType_sogouinput;
                  return InputMethodType::InputMethodType_sogouinput;
               }
               // Java: else if (inputMethodManagerCurID.startsWith("com.iflytek.inputmethod"))
               else if id.starts_with("com.iflytek.inputmethod")
               {
                  // Java: return InputMethodType_iflytek;
                  return InputMethodType::InputMethodType_iflytek;
               }
               // Java: else if (inputMethodManagerCurID.startsWith("com.adamrocker.android.input.simeji"))
               else if id.starts_with("com.adamrocker.android.input.simeji")
               {
                  // Java: return InputMethodType_simeji;
                  return InputMethodType::InputMethodType_simeji;
               }
               // Java: else if (inputMethodManagerCurID.startsWith("com.motorola.inputmethod.entry"))
               else if id.starts_with("com.motorola.inputmethod.entry")
               {
                  // Java: return InputMethodType_motorola;
                  return InputMethodType::InputMethodType_motorola;
               }
               // Java: else if (inputMethodManagerCurID.startsWith("com.socialnmobile.hangulkeyboard"))
               else if id.starts_with("com.socialnmobile.hangulkeyboard")
               {
                  // Java: return InputMethodType_hangulkeyboard;
                  return InputMethodType::InputMethodType_hangulkeyboard;
               }
               // Java: else if (inputMethodManagerCurID.startsWith("com.diotek.ime.diopencn"))
               else if id.starts_with("com.diotek.ime.diopencn")
               {
                  // Java: return InputMethodType_diopencn;
                  return InputMethodType::InputMethodType_diopencn;
               }
               // Java: else if (!inputMethodManagerCurID.startsWith("com.samsung.inputmethod") && !inputMethodManagerCurID.startsWith("com.sec.android.inputmethod/.SamsungKeypad"))
               else if !id.starts_with("com.samsung.inputmethod") && !id.starts_with("com.sec.android.inputmethod/.SamsungKeypad")
               {
                  // Java: if (inputMethodManagerCurID.startsWith("com.justsystems.atokmobile.tv.service"))
                  if id.starts_with("com.justsystems.atokmobile.tv.service")
                  {
                     // Java: return InputMethodType_atokmobile;
                     return InputMethodType::InputMethodType_atokmobile;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.google.android.inputmethod.korean"))
                  else if id.starts_with("com.google.android.inputmethod.korean")
                  {
                     // Java: return InputMethodType_googlekorean;
                     return InputMethodType::InputMethodType_googlekorean;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.sonyericsson.textinput"))
                  else if id.starts_with("com.sonyericsson.textinput")
                  {
                     // Java: return InputMethodType_sonyericsson;
                     return InputMethodType::InputMethodType_sonyericsson;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.huawei.inputmethod.hwpal"))
                  else if id.starts_with("com.huawei.inputmethod.hwpal")
                  {
                     // Java: return InputMethodType_huawei;
                     return InputMethodType::InputMethodType_huawei;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.meizu.input"))
                  else if id.starts_with("com.meizu.input")
                  {
                     // Java: return InputMethodType_meizu;
                     return InputMethodType::InputMethodType_meizu;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("net.cdeguet.smartkeyboard"))
                  else if id.starts_with("net.cdeguet.smartkeyboard")
                  {
                     // Java: return InputMethodType_smartkeyboard;
                     return InputMethodType::InputMethodType_smartkeyboard;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.cootek.smartinput"))
                  else if id.starts_with("com.cootek.smartinput")
                  {
                     // Java: return InputMethodType_chubao;
                     return InputMethodType::InputMethodType_chubao;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.tencent.qqpinyin"))
                  else if id.starts_with("com.tencent.qqpinyin")
                  {
                     // Java: return InputMethodType_qqpinyin;
                     return InputMethodType::InputMethodType_qqpinyin;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("net.toload.main"))
                  else if id.starts_with("net.toload.main")
                  {
                     // Java: return InputMethodType_lime;
                     return InputMethodType::InputMethodType_lime;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.google.android.inputmethod.latin"))
                  else if id.starts_with("com.google.android.inputmethod.latin")
                  {
                     // Java: return InputMethodType_googlelatin;
                     return InputMethodType::InputMethodType_googlelatin;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.dasur.slideit.vt.lite"))
                  else if id.starts_with("com.dasur.slideit.vt.lite")
                  {
                     // Java: return InputMethodType_slideit;
                     return InputMethodType::InputMethodType_slideit;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.aitype.android"))
                  else if id.starts_with("com.aitype.android")
                  {
                     // Java: return InputMethodType_aitype;
                     return InputMethodType::InputMethodType_aitype;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("net.hciilab.scutgPen.IME"))
                  else if id.starts_with("net.hciilab.scutgPen.IME")
                  {
                     // Java: return InputMethodType_scutgpen;
                     return InputMethodType::InputMethodType_scutgpen;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("jp.ne.neko.freewing"))
                  else if id.starts_with("jp.ne.neko.freewing")
                  {
                     // Java: return InputMethodType_openwnneasy;
                     return InputMethodType::InputMethodType_openwnneasy;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("free.inputmethod.latin.perfectkeyboard"))
                  else if id.starts_with("free.inputmethod.latin.perfectkeyboard")
                  {
                     // Java: return InputMethodType_perfect;
                     return InputMethodType::InputMethodType_perfect;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.insyde.ime"))
                  else if id.starts_with("com.insyde.ime")
                  {
                     // Java: return InputMethodType_more;
                     return InputMethodType::InputMethodType_more;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.google.android.inputmethod.pinyin"))
                  else if id.starts_with("com.google.android.inputmethod.pinyin")
                  {
                     // Java: return InputMethodType_googlepinyin;
                     return InputMethodType::InputMethodType_googlepinyin;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.klye.ime.latin"))
                  else if id.starts_with("com.klye.ime.latin")
                  {
                     // Java: return InputMethodType_multiling;
                     return InputMethodType::InputMethodType_multiling;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.thaicomcenter.android.tswipepro"))
                  else if id.starts_with("com.thaicomcenter.android.tswipepro")
                  {
                     // Java: return InputMethodType_tswipepro;
                     return InputMethodType::InputMethodType_tswipepro;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.nuance.xt9.input/.IME"))
                  else if id.starts_with("com.nuance.xt9.input/.IME")
                  {
                     // Java: return InputMethodType_asus_xt9;
                     return InputMethodType::InputMethodType_asus_xt9;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.asus.ime/.IME"))
                  else if id.starts_with("com.asus.ime/.IME")
                  {
                     // Java: return InputMethodType_asus;
                     return InputMethodType::InputMethodType_asus;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.gingersoftware.android.keyboard"))
                  else if id.starts_with("com.gingersoftware.android.keyboard")
                  {
                     // Java: return InputMethodType_ginger;
                     return InputMethodType::InputMethodType_ginger;
                  }
                  // Java: else if (inputMethodManagerCurID.startsWith("com.syntellia.fleksy"))
                  else if id.starts_with("com.syntellia.fleksy")
                  {
                     // Java: return InputMethodType_fleksy;
                     return InputMethodType::InputMethodType_fleksy;
                  }
                  // Java: else
                  else
                  {
                     // Java: return inputMethodManagerCurID.startsWith("de.ubuntix.android.mygermankeyboard/.MyGermanKeyboard") ? InputMethodType_germanKeyboard : InputMethodType_unknown;
                     return if id.starts_with("de.ubuntix.android.mygermankeyboard/.MyGermanKeyboard") { InputMethodType::InputMethodType_germanKeyboard } else { InputMethodType::InputMethodType_unknown };
                  }
               }
               // Java: else
               else
               {
                  // Java: return InputMethodType_samsung;
                  return InputMethodType::InputMethodType_samsung;
               }
            }
            // Java: else
            else
            {
               // Java: return InputMethodType_gokeyboard;
               return InputMethodType::InputMethodType_gokeyboard;
            }
         }
         // Java: else
         else
         {
            // Java: return InputMethodType_swype;
            return InputMethodType::InputMethodType_swype;
         }
      }
    }

}
