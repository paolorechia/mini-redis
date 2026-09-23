# Mini Redis

Yet another one. This is a toy/learning project. The goal is to warm up in the Rust programming language again, while refreshing some basic concepts.

What we'll build is essentially a mini Redis (not yet ready for production) with no AI generated code. The purpose is to practice.

We'll implement the following operations:

SET key value
GET key
DELETE key
EXISTS key

And serve it over HTTP. Refer to the diagrams for more information. We'll only accept strings, both for keys and values.


# Using d2

This project uses d2 for diagrams (https://www.d2lang.com/tour/install/).

# Using the Taskfile.yml

This project uses Taskfile to manage common tasks, see more info at: https://taskfile.dev.


# Serialization Protocol

The protocol is dead simple. A new message frame begins with:

#####START#####

Ends with:

#####END#####

And has a maximum limit of 1024 bytes, including the above start/end markers