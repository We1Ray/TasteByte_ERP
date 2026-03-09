INSERT INTO number_ranges (object_type, prefix, current_number, pad_length)
VALUES ('CUST', 'CUST', 0, 8)
ON CONFLICT (object_type) DO NOTHING;
