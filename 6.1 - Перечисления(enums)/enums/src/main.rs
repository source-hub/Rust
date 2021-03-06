// Rust поддерживает перечисления(enumerations):
#[derive(Debug)] // Нужно только для отладки.
enum IpAddrKind {
    // V4 и V6 называют вариантами(variants) перечисления.
    V4,
    V6,
}

// С каждым вариантом перечисления можно ассоциировать значение определённого типа(даже тип
// перечисления). Тип каждого из вариантов необъязательно должен быть одинаковым:
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8), // tuple struct
    V6(String),
}

// Ещё один пример определения перечисления:
#[derive(Debug)]
enum Message {
    Quit, // Вариант не имеет ассоциированного значения.
    Move { x: i32, y: i32 }, // Анонимная структура.
    Write(String), // Тип String.
    ChargeColor(i32, i32, i32), // tuple struct из трёх i32.
}

// Для перечислений также как и для структур можно определять методы:
impl Message {
    fn call(&self) {
        // Do something.
    }
}

fn main() {
    // Создать экземпляры перечисления можно так:
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    // Создание экземпляра перечисления с значением:
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);

    //--------------------------------------------------------------------------------
    // Перечисление Option из стандартной библиотеки
    //--------------------------------------------------------------------------------

    // Перечисление Option является одним из наиболее часто используемых перечислений благодаря
    // тому, что оно может обозначить есть ли у нас есть некоторое значение или нет(так называемая
    // null feature). В языках, где Null есть, переменная в каждый момент времени может быть равна
    // Null или не Null и обращение с Null значением как с не Null приводит к разного рода ошибкам.
    // Option определён в следующем виде:
    //
    //     enum Option<T> {
    //         Some(T),
    //         None
    //     }
    //
    // Само перечисление Option и его варианты можно использовать без явного указания пространства
    // имён(они находятся в prelude).

    // Создание экземпляров Option:
    let some_number = Some(42);
    let absent_number: Option<i32> = None;
    println!("{:?}", some_number);
    println!("{:?}", absent_number);

    // В Rust если значение не имеет тип Option<T> мы можем предположить, что это значение не
    // является null и им можно безопасно пользоваться. Если же значение имеет тип Option<T>, то
    // нам нужно писать код, который будет работать и с случаем, когда значение есть и когда нет.
    // Получить значение, которое хранится в варианте Some можно различными методами перечисления
    // Option, но лучше всего работать с переменными такого типа, используя выражение match,
    // которое будет рассмотренно дальше.
}

// Функция, которая принимает на вход аргумент типа IpAddrKind.
fn route(ip_type: IpAddrKind) {
    println!("{:?}", ip_type);
}
