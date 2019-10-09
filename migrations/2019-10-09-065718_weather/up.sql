-- Your SQL goes here
-- TODO: fix this (create database if not exist)
-- SELECT 'CREATE DATABASE the-tensox;' WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'the-tensox')
CREATE TABLE weather(
  id SERIAL PRIMARY KEY,
  x int NOT NULL,
  y int NOT NULL,
  sun INT NOT NULL
)