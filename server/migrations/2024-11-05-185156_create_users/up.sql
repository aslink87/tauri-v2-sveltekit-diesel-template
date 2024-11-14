CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    loginname VARCHAR NOT NULL,
    passwordhash VARCHAR(97) NOT NULL
);

INSERT INTO public.users (id, loginname, passwordhash) VALUES
(
    1, 'john',
    '$argon2id$v=19$m=19456,t=2,p=1$GvilldCdAoRgp5LVyNr0pQ$lQJ2GioOTB4CnTagMOvWUIGpR9i5ftdphvbFmxOCVJA'
);
