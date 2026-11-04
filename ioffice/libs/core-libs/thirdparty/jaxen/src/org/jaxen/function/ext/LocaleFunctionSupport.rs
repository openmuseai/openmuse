// Java: import java.util.List
// Java: import java.util.Locale
// Java: import java.util.StringTokenizer
// Java: import org.jaxen.Function
use crate::org::jaxen::Function::Function;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
// Java: import org.jaxen.function.StringFunction
use crate::org::jaxen::function::StringFunction::StringFunction;
use crate::org::jaxen::{Object, List};

// Java: public class LocaleFunctionSupport implements Function
pub struct LocaleFunctionSupport {
}

// Java: public class LocaleFunctionSupport implements Function
impl LocaleFunctionSupport {
    // Java: LocaleFunctionSupport()
    pub fn new() -> Self {
      // Java: super();
      LocaleFunctionSupport {}
    }

    // Java: Locale getLocale(Object value, Navigator navigator)
    pub fn getLocale(&self, _value: &Object, _navigator: Option<&dyn Navigator>) -> Option<String> {
      // Java: if (value instanceof Locale)
      // Java: return (Locale)value;
      // Java: else
      // Java: if (value instanceof List)
      // Java: List list = (List)value;
      // Java: if (!list.isEmpty())
      // Java: return this.getLocale(list.get(0), navigator);
      // Java: else
      // Java: String text = StringFunction.evaluate(value, navigator);
      // Java: if (text != null && text.length() > 0)
      // Java: return this.findLocale(text);
      // Java: return null;
      todo!("LocaleFunctionSupport::getLocale - requires Locale support")
    }

    // Java: Locale findLocale(String localeText)
    pub fn findLocale(&self, _localeText: &str) -> Option<String> {
      // Java: StringTokenizer tokens = new StringTokenizer(localeText, "-");
      // Java: if (tokens.hasMoreTokens())
      // Java: String language = tokens.nextToken();
      // Java: if (!tokens.hasMoreTokens())
      // Java: return this.findLocaleForLanguage(language);
      // Java: else
      // Java: String country = tokens.nextToken();
      // Java: if (!tokens.hasMoreTokens())
      // Java: return new Locale(language, country);
      // Java: else
      // Java: String variant = tokens.nextToken();
      // Java: return new Locale(language, country, variant);
      // Java: else
      // Java: return null;
      todo!("LocaleFunctionSupport::findLocale - requires Locale support")
    }

    // Java: Locale findLocaleForLanguage(String language)
    pub fn findLocaleForLanguage(&self, _language: &str) -> Option<String> {
      // Java: Locale[] locales = Locale.getAvailableLocales();
      // Java: int i = 0;
      // Java: for(int size = locales.length; i < size; ++i)
      // Java: Locale locale = locales[i];
      // Java: if (language.equals(locale.getLanguage()))
      // Java: String country = locale.getCountry();
      // Java: if (country == null || country.length() == 0)
      // Java: String variant = locale.getVariant();
      // Java: if (variant == null || variant.length() == 0)
      // Java: return locale;
      // Java: return null;
      todo!("LocaleFunctionSupport::findLocaleForLanguage - requires Locale support")
    }
}
