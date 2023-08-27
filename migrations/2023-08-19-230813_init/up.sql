-- Your SQL goes here

-- Create the "students" table
CREATE TABLE students (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    birth_date DATE NOT NULL,
    password TEXT NOT NULL
);

-- Create the "teachers" table
CREATE TABLE teachers (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    password TEXT NOT NULL
);

-- Create the "questions" table
CREATE TABLE questions (
    id SERIAL PRIMARY KEY,
    text TEXT NOT NULL
);

-- Create the "classes" table
CREATE TABLE classes (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    teacher_id INTEGER NOT NULL,
    FOREIGN KEY (teacher_id) REFERENCES teachers (id)
);

-- Create the "answers" table
CREATE TABLE answers (
    id SERIAL PRIMARY KEY,
    question_id INTEGER NOT NULL,
    student_id INTEGER NOT NULL,
    text TEXT NOT NULL,
    FOREIGN KEY (question_id) REFERENCES questions (id),
    FOREIGN KEY (student_id) REFERENCES students (id)
);

-- Create the "members" table
CREATE TABLE members (
    id SERIAL PRIMARY KEY,
    student_id INTEGER NOT NULL,
    class_id INTEGER NOT NULL,
    FOREIGN KEY (student_id) REFERENCES students (id),
    FOREIGN KEY (class_id) REFERENCES classes (id)
);

-- Create the "results" table
CREATE TABLE results (
    id SERIAL PRIMARY KEY
);

