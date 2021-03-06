fn main() {
    //--------------------------------------------------------------------------------
    // Слайсы
    //--------------------------------------------------------------------------------

    // Ещё одним типом, который не владеет значением(кроме ссылок), является срез(slice). Срез
    // позволяет ссылаться на непрерывный участок коллекции вместо всей коллекции.

    // Объясним работу срезов на следующем примере. Допустим у нас есть задача написать функцию,
    // которая возвращает первое слово переданной ей строки(считаем, что слова делятся только одним
    // пробелом). Для этого можно воспользоваться функцией first_word_not_good, которая возвращает
    // индекс конца первого слова.
    let mut msg = String::from("hello");
    msg.push_str(" world");
    let word = first_word_not_good(&msg);
    println!("word: {}", word);

    // Но у этого подхода есть недостаток: если строка измениться, переменная word будет хранить
    // неактуальные данные. Переменные word и s никак не синхронизированы между собой. Для решения
    // этой проблемы можно применить строковые срезы(string slices). Создать срез можно так:

    // Создание среза похоже на взятие ссылки, только после переменной в квадратных скобках
    // указывается диапазон от начального индекса до конечного, не включая конечный.
    let h = &msg[0..5]; // hello
    let w = &msg[6..11]; // world
    println!("h: {}, w: {}", h, w);

    // Если мы явно хотим включить конечный символ, то можно воспользоваться '..=':
    let h2 = &msg[0..=4];
    println!("h2: {}", h2);

    // Также можно создавать и такие срезы:
    let h3 = &msg[..5]; // Если не указать начало, то считаем его равным 0.
    let w2 = &msg[6..]; // Если не указать конец, то считаем его равным длине строки.
    let msg_slice = &msg[..]; // Срез всей строки.
    println!("h3: {}, w2: {}, msg_slice: {}", h3, w2, msg_slice);

    // Срез строк должен происходить по границе UTF-8 символов. Если это не так, то будет ошибка
    // во время выполнения.
    // let ru = String::from("Победа");
    // let ru_slice = &ru[0..5];
    // println!("{}", ru_slice);

    // Теперь воспользуемся версией функции, которая возвращает срез.
    let first_word = first_word_good(&msg);
    println!("first_word: {}", first_word);

    // Преимуществом такого подхода является то, что теперь слайс и исходная переменная связаны
    // между собой и компилятор не позволит изменить msg:

    // Ошибка компиляции.
    // msg.clear();

    // Ошибка происходит из-за того, что методу clear нужна изменяемая ссылка. Для передачи
    // в этот метод именно такой ссылки необходимо сначала создать такую ссылку в текущей области
    // видимости. Но в текущий области видимости уже есть неизменяемая ссылка на msg - first_word.
    // Следуя правилу заимствования, нельзя создавать изменяемую ссылку в какой-либо области
    // видимости, если там уже есть неизменяемая. Это и является причиной ошибки.
    println!("first_word: {}", first_word);

    //--------------------------------------------------------------------------------
    // Строковые литералы являются срезами
    //--------------------------------------------------------------------------------

    // Строковые литералы - это срезы:
    let literal = "string literal";
    println!("literal: {}", literal);

    // Типом среза строкового литерала является &str. Поэтому и нельзя менять литералы - &str это
    // неизменяемая ссылка.

    //--------------------------------------------------------------------------------
    // Другие виды срезов
    //--------------------------------------------------------------------------------

    // До сих пор мы рассматривали только срезы строк, но понятие среза можно применить к любым
    // коллекциям, например, массивам:
    let array = [1, 2, 3, 4];
    // Срез array_slice имеет тип &[i32].
    let array_slice = &array[1..3];
    println!("array_slice[0]: {}", array_slice[0]);
}

fn first_word_not_good(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// &str обозначает слайс строки.
fn first_word_good(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
