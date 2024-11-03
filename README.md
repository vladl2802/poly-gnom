# poly-gnom

Моя попытка в реализацию обощенного интерфейса многочленов.

Написано в качестве тестового задания по условию [отсюда](https://church-of-church-hse.github.io/testtask/).

Задача очень похожа на символьные вычисления, с решениями которой я не знаком, поэтому я придумывал свои решения.

## Основные решения

- Компиляция выражений в рантайме (в основном потому что на расте написать что-то подобное в компайл тайме кажется невозможно без макросов, а с ними смысла не много).

- На коэффициенты многочлена не накладывается дополнительных ограничений помимо наличия конкретного значения (это планируется поменять, об этом далее).

- Полностью обобщенный интерфейс многочленов: от пользователя требуется реализовать необходимые трейты и классы, после чего свободно строить и вычислять многочлены (пример такой реализации есть в текстах и он планировался не один, подробнее далее).

- Два ключевых класса: типы значений и сами значения. Первые нужны для того, чтобы не делать выражения полностью динамически типизированными. Вторые нужны для вычислений многочленов в конктреных точках.

- От операций над значениями и типами требуется только ассоциативность. Коммутативность не является обязательным свойством. Сейчас не реализована некоммутативное сложение. (Но это не критично, т.к. упрощение тоже не реализовано) 

## Подробнее про реализацию

Рассмотрим в качестве типов: скаляры, вектора и матрицы. Ну и значения им сооветствующие.

Рассмотрим внутреннее представление многочлена $5(2A + B)v + 3u + 2yw$ 

(на самом деле в текущей реализации $5(2A + \underline{1}B)v + 3u + 2yw$ и эта единица важна, потому что сложить два многочлена так просто нельзя сейчас, подробнее будет далее)

![Дерево представления многочлена $5(2A + B)v + 3u + 2yw$](/Poly-tree.svg)

Переменными (или же символами) здесь являются $y, v, u, A, B, v$. Их типы пока что не проставлены, а значит и тип всего выражения мы не знаем.

Пускай $y$ будет скаляром, $v, u$ будут векторами, а $A, B$ будут матрицами.

В таком случае тип всего многочлена будет вектором.

### Чуть подробнее про символы

Любая переменная в многочлене всегда задается некоторым символом. Если два символа равны, то и переменные являются одинаковыми.

Поэтому вводится __SymbolsProvider__, который хранит внутри себя все возможные символы, которые могут быть использованы.

Также понятно, что иногда хочется, чтобы переменная всегда имела один тип. Поэтому ей можно (но не обязательно) его сопоставить.

### Чуть подробнее про арифметические операции Values и Types

Для значений возвращаемый тип почти всегда Option\<Values\>. За исключением унарного минуса.

Для типов возвращаемое значение в общем смысле это рантайм трейт (трейт тут не языковой, однако выполняет очень похожую функцию). Сейчас это явно так только для умножения, т.к. до остальных операций я не добрался.

Для умножения (Types $\times$ Types $\rightarrow$ MulTraits) возвращаемый тип, это

```
MulTraits {
    result: Option<Types>
    is_commutative: bool
}
```

Свойство коммутативности нужно при упрощении внутри термов.

У сложения то же свойство коммутативности нужно при упрощение внутри уже внутри многочлена.

Для нуля и единицы (в каком-то смысле это тоже арифметичекие операции) по хорошему нужен трейт, который будет говорить, существует ли они для фиксированного типа.

### Возможные операции над многочленом

Вместо переменных можно подставлять значения, другие переменные и другие многочлены.

При этом в случае, если переменной сопоставлен ожидаемый тип, то подстановка может быть неудачной, если тип подставляемого не совпадает.

Также у многочлена можно узнать тип его значения (если его возможно вывести). А также попытаться вычислить значение многочлена, если в нем нет переменных.

### Чуть-чуть про степени

Первая степень любого элемента всегда возвращает сам элемент и не делает никаких дополнительных проверок.

Нулевая и степени больше первой требуют, чтобы тип умел умножаться и в результате получался тот же тип.

Для нулевой степени от требования на результирующий тип не понятно как отказываться. Я склоняюсь к тому, что разумного определения нет.

Для степеней больше первой требование не выглядит как обязательное. Главное, чтобы сохранялась ассоциативность. (Оно например обязательно для бинарного возведения в степень, которым я пользуюсь. Но тут можно разделить на две реализации.)

## Что позволяет это делать

### Наивные скалярно-векторно-матриные многочлены

Это тот же пример, что и приведенный выше, с 3 типами: скаляр, вектор и матрица. Также это единственное, что реализовано в коде.

### Строго типизированные скалярно-векторно-матриные многочлены

Сейчас реализовано и выше написано про случай, когда типов всего 3: скаляр, вектор и матрица.

При реализации например нулевой/единичной матрицы/вектора возникает проблема, что мы не знаем его размерность. А значит надо либо запретить создание нулевой и единичной матрицы и вектора, либо делать их ленивыми: нулевая матрица может умножаться на любую матрицу и вектор и тд.

Я в текущей реализации пошел простым путем и запретил, т.к. это просто быстрее. Второй вариант тоже можно реализовать. Но конечно более красивым было бы добавить в тип вектора и матрица их значения.

В таком случае ошибка с этапа подстановки значений может быть выявлена уже на этапе типизации многочлена (RUST WAAAAAAY).

### Функции в качестве значений

Этот пример строится поверх предыдущего. И он был у меня голове основным, когда я писал эту реализацию.

Ничего не мешает к уже существующим значениям добавить функции одного аргумента. Например синус, косинус, определитель, след и тд.

```
Types {
    Basic: mat_vec::Types
    Funcs: Basic -> Basic
}
```

Тогда выражения могут быть представлены в виде многочлена: 

$\det(A - xE)$

$\sin(fAx + gAx)$ - где $f, g$ какие функции из вектора в число.

### Строки в качестве значений

Все прошлые элементы были из какого-нибудь кольца. Но это не обязательно для значения многочлена. Необязательно уметь умножать строки на строки. Даже не обязательно иметь коммутативное сложение. 

Поэтому к типам можно добавить строки, а также добавить функции, которые будут вычислять из строк остальные типы.

```
Types {
    Values: func_mat_vec::Types
    Strings: String
    Parsers: String -> Values
}
```

## Имеющееся TODO

Которое я знаю как делать, но еще не реализовал в силу нехватки времени.

### Многочлены нельзя легко сложить и умножить

Рассмотрим два не типизированных многочлена $A$ и $B$.  

Разумным кажется их перемножить, однако это не так просто, т.к. $AB$ может не иметь известно типа. И в таком случае мы не можем легко сказать, какое именно значение для коэффициента нужно поставить (не знаем что именно за единица нужна).

Сейчас их можно перемножить только создав многочлен $1xy$, где для $x$ и $y$ не обязательно задавать типы, и подставить в него $A$ и $B$. Также можно сделать для сложения.

В связи с этим я хочу разделить многочлен на два случая: c выведенными типами и просто символьный многочлен. И для символьного многочлена разрешить не иметь коэффициентов.

Это требует не малого переписывания и обдумывания как именно стоит это правильно написать.

Например при таком переписывание можно заодно начать требоват, что если типы корректно вывелись, то при подстановке будет либо паника, либо значие выведенного типа.

### Упрощение многочлена

Упрощение тоже не написано и тут нет какой-то явной причины кроме нехватки времени.

Это можно сделать даже в отсутствии попарной коммутативности. Но я пока что не знаю алгоритма лучше, чем сортировка пузырьком.

### Дописать описанные тесты

Реализован лишь не строгий скалярно-векторно-матричный многочлен, остальные примеры тоже хотелось бы написать конечно. Но это не малое количество работы.

Стоит также добавить какие-то менее разумные тесты, которые тем ни менее попадают в ограничения.

### Генератор операций над Values и Types

Сейчас весь этот код надо писать руками (можно посмотреть как это выглядит в /tests/simple_mat_vec/values.rs и /tests/simple_mat_vec/types.rs). Хотя по факту, в нем не очень много содержательного. Особенно для Values. В связи с этим хочется генерировать куски макросами.

Однако я еще не продумывал, какими именно это кусками стоит генерировать.

### Причесать код, дописать тесты, документацию

Например сейчас ошибки почти ни о чем не говорят. Можно было бы сделать так, чтобы они показывали контекст. 

На это у меня опять же не хватило времени.......