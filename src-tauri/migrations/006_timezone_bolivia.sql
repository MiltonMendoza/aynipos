-- Migration: Change all datetime defaults from UTC to Bolivia time (UTC-4)
-- Bolivia does not observe DST, so -4 hours is always correct.

-- We cannot ALTER DEFAULT in SQLite, but new rows will use the updated
-- DEFAULT from the application code. This migration exists as a marker
-- that the timezone change was applied.

-- No structural changes needed — SQLite DEFAULT expressions are evaluated
-- at INSERT time, and we handle them in application code.
