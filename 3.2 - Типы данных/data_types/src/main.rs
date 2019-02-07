fn main() {
    // Rust является статически типизированным языком, т.е. тип любой переменной должен быть
    // известен на этапе компиляции. Любое значение в Rust имеет тип. Компилятор иногда может сам
    // выводить тип переменной, исходя из выражения справа от неё, но иногда нужно и явно указывать
    // тип:
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);
    // Тип переменной guess должен быть указан явно, т.к. перевести строку "42" можно к различным
    // целочисленным типам.

    // Далее будут рассмотрены две группы типов: скалярные(scalar) и составные(compound).

    //--------------------------------------------------------------------------------
    // Скалярные типы
    //--------------------------------------------------------------------------------

    // К скалярным типам относятся целочисленный, вещественный, булев и символьный типы.

    //----------------------------------------
    // Целочисленные типы
    //----------------------------------------

    // В Rust есть следующие целочисленные типы:
    //
    //   - i8,    u8
    //   - i16,   u16
    //   - i32,   u32
    //   - i64,   u64
    //   - i128,  u128
    //   - isize, usize

    // Приставка 'i' означает знаковый тип, 'u' - беззнаковый.
    // Размеры типов isize и usize зависит от архитектуры.
    // Поддерживаются литералы в 10-м, 2-м, 8-м и 16-ричном виде:
    println!("{} {} {} {}", 255, 0xFF, 0o377, 0b11111111);

    // Для удобства чтения группы цифр можно выделять с помощью '_':
    println!("{}", 65_535);

    // Если к переменной типа u8 равной 255 добавить 1, то произойдёт так называемое целочисленное
    // переполнение(integer overflow). В зависимости от режима компиляции Rust обрабатывает эту
    // ситуацию по-разному. В режиме отладки будет паника(panic) - понятие, которое используется в
    // Rust для обозначения выхода из программы с ошибкой. При сборке релизной версии проверка на
    // переполнение не выполняется, а осуществляется циклический переход к началу. Т.е. в нашем
    // случае при добавлении к 255 единицы мы получим 0:
    let mut n: u8 = 255;
    n = n + 1;
    println!("n: {}", n);

    // Не стоит полагаться на переполнение, т.к. обычно это считается ошибкой. Если мы явно хотим
    // именно такое поведение, то нужно пользоватья типом Wrapping из стандартной библиотеки.

    //----------------------------------------
    // Вещественные типы
    //----------------------------------------

    // К вещественным типам относятся f32 и f64 - соответствуют стандарту IEEE-754. Если тип не
    // указать, то применяется тип f64:
    let x = 13.3; // f64
    let y: f32 = 0.5; // f32
    println!("x: {}, y: {}", x, y);

    //----------------------------------------
    // Булев тип
    //----------------------------------------

    // Булев тип представлен типом bool и литералами true и false:
    let flag: bool = true;
    println!("flag: {}", flag == false);

    //----------------------------------------
    // Символьный тип
    //----------------------------------------

    // Символьный тип представлен типом char. Литералы символов необходимо заключить в одинарные
    // кавычки. Примеры:
    let c = 'Z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);

    // char в Rust представляет собой скалярное значение Unicode, т.е. с помощью него можно хранить
    // не только ASCII символы.

    //--------------------------------------------------------------------------------
    // Составные типы
    //--------------------------------------------------------------------------------

    // Составные типы могут объединять несколько значений в один тип. Rust поддерживает два
    // базовых составных типа: кортеж(tuple) и массив(array).

    //----------------------------------------
    // Кортеж
    //----------------------------------------

    // Кортеж - это наиболее общий способ группирования вместе значений необъязательно одинакового
    // типа. Имеет фиксированную длину, которую нельзя менять после создания:

    // Объявление кортежа с явным указания типа.
    let tup: (i32, f64, u8) = (10, 19.2, 4);
    // Объявление кортежа без указания типа.
    let primes = (2, 3, 5);

    // Для доступа к элементам кортежа можно провести операцию деструктуризации:
    let (p1, p2, p3) = primes;
    println!("p1, p2, p3: {}, {}, {}", p1, p2, p3);

    // Для доступа к отдельным элементам кортежа можно воспользоваться '.', за которым должен идти
    // индекс элемента. Счёт идёт с 0:
    println!("1-st element of tup: {}", tup.0);

    //----------------------------------------
    // Массив
    //----------------------------------------

    // Массивы как и кортежи имеют фиксированную длину, но в отличие от них, элементы массива
    // должны быть одинакового типа. Пример массива:
    let array = [4, 3, 2, 1];

    // Массивы бывают полезны, когда нам нужны данные, которые точно будут храниться в стеке, а не
    // в куче или когда нужен фиксированный набор данных одинакового типа.
    // Объявление массива с явным указанием типа имеет следующий вид:
    let fib: [i32; 5] = [1, 1, 2, 3, 5];

    // В этом объявлении создаётся массив размера 5 из элементов типа i32.
    // Обращение к элементам массива происходит через квадратные скобки, внутри которых
    // указывается индекс. Счёт идёт 0:
    println!("array[0]: {}, fib[2]: {}", array[0], fib[2]);

    // Если обратиться к элементу массива, которого в нём нет, то во время выполнения программы
    // возникнет паника и выполнение программы остановится с выводом ошибки. Ошибки компиляции при
    // этом не будет.
    // Следующей код приведёт к выводу ошибки 'index out of bounds...':

    // let index = 2019;
    // let tmp = fib[index];

    // Проверка не выхода индекса за размеры массива является одной из мер безопасности, которую
    // предоставляет Rust. Другой какой-нибудь системный язык спокойно дал бы вычитать значение из
    // памяти и пошёл бы работать дальше.
}
