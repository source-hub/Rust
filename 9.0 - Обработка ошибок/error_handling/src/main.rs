fn main() {
    // При программировании ошибки неизбежны, но Rust позволяет поймать многие из них на этапе
    // компиляции. Rust глобально делит все ошибки на две категории:
    //
    //     - устраняемые(recoverable) - пользователь уведомляется о наличии ошибки и дальше он
    //                                  решает, что делать. Для обозначения таких ошибок
    //                                  используется тип Result<T, E>.
    //
    //     - неустраняемые(unrecoverable) - ошибки, которые всегда являются симптомами багов; для
    //                                      остановки выполнения программы используется макрос
    //                                      panic!.
    //
    // В Rust нет исключений.
}
