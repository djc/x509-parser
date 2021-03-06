//! X.509 helper objects definitions and registry
//!
//! Most definitions taken from OpenSSL: /usr/include/openssl/objects.h

use crate::error::NidError;
use der_parser::oid::Oid;
use lazy_static::lazy_static;
use oid_registry::*;
use std::collections::HashMap;

lazy_static! {
    static ref OID_REGISTRY: OidRegistry<'static> = {
        let reg = OidRegistry::default().with_all_crypto().with_x509();
        // OIDs not in the default registry can be added here
        reg
    };
    static ref ABBREV_MAP: HashMap<Oid<'static>, &'static str> = {
        let mut m = HashMap::new();
        m.insert(OID_X509_COMMON_NAME, "CN");
        m.insert(OID_X509_COUNTRY_NAME, "C");
        m.insert(OID_X509_LOCALITY_NAME, "L");
        m.insert(OID_X509_STATE_OR_PROVINCE_NAME, "ST");
        m.insert(OID_X509_ORGANIZATION_NAME, "O");
        m.insert(OID_X509_ORGANIZATIONAL_UNIT, "OU");
        m.insert(OID_DOMAIN_COMPONENT, "DC");
        m.insert(OID_PKCS9_EMAIL_ADDRESS, "Email");
        m
    };
}

/// Return the abbreviation (for ex. CN for Common Name), or if not found, the OID short name
pub fn oid2abbrev<'a>(oid: &'a Oid) -> Result<&'a str, NidError> {
    if let Some(abbrev) = ABBREV_MAP.get(oid) {
        return Ok(abbrev);
    }
    OID_REGISTRY
        .get(oid)
        .map(|entry| entry.sn())
        .ok_or(NidError)
}

/// Returns the short name corresponding to the OID
pub fn oid2sn<'a>(oid: &'a Oid) -> Result<&'a str, NidError> {
    OID_REGISTRY.get(oid).map(|ref o| o.sn()).ok_or(NidError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use der_parser::oid;

    // This test is meant to check syntax of pattern matching with OID objects
    #[test]
    fn test_oid_match() {
        let oid = oid!(1.2.840.113549.1.1.5);
        if oid == OID_PKCS1_SHA1WITHRSA {
            // ok
        }
        // matching is not possible with Cow constants in pattern,
        // see https://rust-lang.github.io/rfcs/1445-restrict-constants-in-patterns.html
        //
        // match oid {
        //     OID_PKCS1_SHA1WITHRSA => (),
        //     _ => (),
        // }
    }
}
