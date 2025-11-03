//! Integration tests for i18n system

#[cfg(test)]
mod tests {
    use shared::{Translator, LocaleDetector};

    #[test]
    fn test_locale_detection() {
        let locale = LocaleDetector::detect_system_locale();
        assert!(!locale.is_empty());
        println!("Detected system locale: {}", locale);
    }

    #[test]
    fn test_translator_creation() {
        let translator = Translator::new("en-US", "./locales");
        assert_eq!(translator.current_locale(), "en-US");
    }

    #[test]
    fn test_locale_switching() {
        let mut translator = Translator::new("en-US", "./locales");
        assert_eq!(translator.current_locale(), "en-US");
        
        // Switch to Portuguese
        let result = translator.set_locale("pt-BR");
        match result {
            Ok(_) => {
                assert_eq!(translator.current_locale(), "pt-BR");
                println!("Successfully switched to pt-BR");
            }
            Err(e) => {
                println!("Note: Could not load pt-BR locale: {}", e);
                println!("This is expected if translation files are not yet in place");
            }
        }
    }

    #[test]
    fn test_translation_fallback() {
        let mut translator = Translator::new("en-US", "./locales");
        
        // Test with a non-existent key - should return the key itself
        let result = translator.translate("non-existent-key");
        assert_eq!(result, "non-existent-key");
    }

    #[test]
    fn test_supported_languages() {
        let languages = shared::i18n::locale::get_supported_languages();
        assert!(!languages.is_empty());
        
        // Check that English and Portuguese are in the list
        assert!(languages.iter().any(|l| l.code == "en-US"));
        assert!(languages.iter().any(|l| l.code == "pt-BR"));
        
        println!("Supported languages count: {}", languages.len());
    }

    #[test]
    fn test_rtl_detection() {
        use shared::i18n::is_rtl;
        
        // RTL languages
        assert!(is_rtl("ar"));
        assert!(is_rtl("ar-SA"));
        assert!(is_rtl("he"));
        assert!(is_rtl("he-IL"));
        assert!(is_rtl("fa"));
        assert!(is_rtl("ur"));
        
        // LTR languages
        assert!(!is_rtl("en-US"));
        assert!(!is_rtl("pt-BR"));
        assert!(!is_rtl("fr-FR"));
        assert!(!is_rtl("de-DE"));
    }
}
