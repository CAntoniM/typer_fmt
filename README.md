# typer_fmt 

this is a application for turning text on provided as command line args into
output written by a keyboard

## Why use this

1. most of the existing solutions are single platform only
2. most of the existing soltuion do not allow resolving data from external
   sources.
3. This allow me to implement a really simple syntax.

## Examples

typer_fmt takes formatted strings from either standard in or as args on a
command and then resolves them down to the a string that is then typed by a
virtual keyboard.

When talking about the parsing that takes place we should talk about the two 
main stages of the parsing.

### Engio syntax

The first is done by eniko as part of its virtual 
keyboard implementaion. this syntax looks like the following

```
{+CTRL}a{-CTRL}{+SHIFT}Hello World{-SHIFT}
```

***Note:*** enigo does not seem to publish there exact syntax so beyond 
            although the assumption is that they are using pure upercase
            versions of the keys found here 
            https://docs.rs/enigo/0.1.2/enigo/keycodes/enum.Key.html

### Resolver Syntax 

The second level of syntax that is layered ontop of engio. that allows for the
insertion of data from external sources into the string. these resolvers follow
the pattern below

```
?<resolver>(<args>)
```

with resolver being the name of one of the below resolvers and the args being 
a string which the syntax of which is specified by the particular resolver

#### Env resolver

this resolver allows you to insert enviroment variables into the programme and
follow a very simple syntax with the name of the enviroment variable being given
as the argument

```
?env(SHELL)

will resolve to (on most linux distros)

/bin/bash
```

#### Now resolver

This allows the user to display the current date and time with the args being
handed off to rusts chrono library and therefore following the pattern shown
here https://docs.rs/chrono/0.4.24/chrono/format/strftime/index.html#specifiers

e.g.

```
?now(%d/%m/%Y)

will resolve to (given it is 6th of april 2023)

06/04/2023
```

