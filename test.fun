variables {
    (name)
    (age)
}
messages {
    @(ask)
    @(display)
}
chunks {
    |receive @(ask)| {
        [set (name) to (input ("What is your name?"))]
        [set (age) to (input number ("What is your age?"))]
    }
}