# FunScript

A fun little programming language with the rare programming paradigm called the actor model.
It is inspired by the visual programming lanuage Scratch which was made for children.

## Program Structure
Every program is defined in the following structure:
- global data
- global messages
- actor
    - local data
    - local messages *(public to other actors)*
    - procedures
        - name
        - parameters
        - body
    - functions
        - name
        - parameters
        - expression
    - events
        - trigger/message/condition
        - in case of a trigger or message, the supplied information
        - body
