-- AyniPOS Migration V10: Business type and system features config
-- Seeds default business_type and feature flags to keep existing installations as Pharmacies (100% backward compatible)

INSERT OR IGNORE INTO settings (key, value) VALUES
    ('business_type', 'pharmacy'),
    ('feature_expiry', '1'),
    ('feature_dose', '1'),
    ('feature_lots', '1'),
    ('feature_suppliers', '1');
