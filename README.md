# nosqlattack

nosqlattack is an application that tries to automate some stuff when testing
for injection attacks in JS and NoSQL web aplications. Some of the functionality
is targeted directly at the database, but most of the functionlity is targeted at
the web forms and it tries to insert either JS or change the database query.

## Application Overview

The application is written in Rust and is written with modularity in mind. There
are two main functions:

1. Authentication attacks against the DB
2. Injection attacks against a web application

### Authentication attacks

Some high level traits are defined. Each specific DB-attack implementation must
create these traits. Currently, only MongoDB and CouchDB are defined.

### Injection attacks

All attacks are defined in an .ini file, so new attacks can be created without
re-compiling the application. The .ini file is parsed to create 1 or more
attacks out of each defined attack. See more details in data/inject.ini.

The functionality of the parser is a bit limited, but it can still create some
decent injection attacks.

## TODO

- Analyze web form so we don't have to type that manually.
- Specify exploit code .ini file so we can display an example of exploit code
  for manual verification.
- Implement blind NoSQL injection for dumping the entire DB.
 - See "ServerSide JavaScript Injection: Attacking NoSQL and Node.js" by Bryan Sullivan
- Inject in header fields as well
- Add vulnerable example application

