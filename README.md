# Test task for WiseTech

_Probably correctly_ solved test task for WiseTech company ([probably this][org]).

> Задача: написать алгоритм, принимающий на вход строку разделенную пробелами, и длину строки в символах.
> Необходимо разбить исходный текст на строки и выравнять по указанной длине строки с помощью пробелов.
> Расстояние между словами нужно заполнять равным количеством пробелов, если же это не возможно, то добавляем
> еще по пробелу между словами слева направо. Если в строке помещается только 1 слово, то дополнить строку
> пробелами справа. Результат вернуть в виде единой строки, где полученный список равных по ширине строк склеен
> с помощью символа перевода строки.
>
> Реализовать максимально производительное решение при сохранении читабельности кода, такого чтобы его можно было использовать в продакшене и поддерживать в дальнейшем.


## History

1. [Initial commit][first] is just task received by recruiter
2. I asked clarifying questions about the details and specialization of the algorithm and the environment, since a "maximum performance" implementation is required.
3. [Second commit][second] is a dumbest non-specialized solution, before getting any answers to my questions.
4. TODO: [Third commit][third] is improved version, after I've got answers.


## Solution

There is two almost identical solutions:
- for [UTF-8][fn-utf]
- for [ASCII][fn-ascii] (little bit faster)
- also there should be ascii "over-bytes" solution without any kind of validation,
  and another one with proper unicode support,
  and generalized writer that cavers each of variants above,
  wrapped into lazy iterator, but not today.

There is no parallelization as well as no SIMD-based pattern-search, just mostly accurate & careful impl without any deps but core, alloc and std.

Run tests:
```
cargo test -- --nocapture
```

Run benches:
```
cargo bench
```

My results on M1 on single tread
```text
test transform_utf   ... bench:   1,056,950 ns/iter (+/- 9,296)
test transform_ascii ... bench:     975,087 ns/iter (+/- 9,091)
```



[org]: https://github.com/WiseTechGlobal
[first]: https://github.com/boozook/WiseTech/commit/deea7a3c34d71a409152873295eda378af6c690e
[second]: #
[third]: #

[fn-utf]: https://github.com/boozook/WiseTech/blob/main/src/lib.rs#L44
[fn-ascii]: https://github.com/boozook/WiseTech/blob/main/src/ascii.rs#L5
