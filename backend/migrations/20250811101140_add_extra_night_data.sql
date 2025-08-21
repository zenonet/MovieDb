-- Add migration script here
ALTER TABLE nights
ADD note text,
ADD place text,
ADD screen text,
ADD display_device text,
ADD disk_player text,
ADD receiver text,
ADD sound_format text,
ADD memento_id char(20)