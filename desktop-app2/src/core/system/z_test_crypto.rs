#[cfg(test)]
mod tests {
    use crate::core::system::crypto::{deobfuscate, obfuscate};

    #[test]
    fn test_deobfuscate() 
    {
        let encripted= obfuscate("En un lugar de la mancha");
        assert_eq!("En un lugar de la mancha", deobfuscate(&encripted));

        let encripted= obfuscate("Hola");
        assert_eq!("Hola", deobfuscate(&encripted));

        let encripted= obfuscate("🎃");
        assert_eq!("🎃", deobfuscate(&encripted));

        let text= r"Hola aca
Estoy como es.
";
        let encripted= obfuscate(&text);
        assert_eq!(text, deobfuscate(&encripted));

        let encripted= obfuscate(&text);
        assert_ne!("No", deobfuscate(&encripted));
    }

}