package kotosiro

default authorize = false

authorize {
    is_read
}

is_read {
    input.action == "Get"
}

is_read {
    input.action == "List"
}