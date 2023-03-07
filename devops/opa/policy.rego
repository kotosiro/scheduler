package kotosiro

default authorize = false

authorize {
    is_read
}

is_read {
    input.action == "get"
}

is_read {
    input.action == "list"
}