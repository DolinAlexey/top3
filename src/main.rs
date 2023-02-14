//1) Напиши программу, в которой пользователь вводит 6 слов. Положи эти слова в какую-либо структуру данных, раздельно по словам. Напиши ф-цию, которая отсортирует слова по длине (по убыванию длины), и выведет top-3 длинных слова.
// Например, я ввел: kata cat tinker cereals he a
// Выводит: cereals tinker kata

use std::io;

fn main() {
    let mut words = Vec::new();
    let mut input = String::new();
// вводим строку со словами, используем пробел в качестве разделителя слов и полученные слова помещаем в вектор
    println!("Ввведи шесть слов: ");
    io::stdin().read_line(&mut input).unwrap();

    let mut counter = input.split_whitespace();
    while words.len() < 6 {
        match counter.next() {
            Some(word) => words.push(word),
            None => break,
        }
    }
// пересортируем пузырьком значения в векторе по возрастанию и выведем последних три элемента с конца
    let len = words.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if words[j].len() > words[j + 1].len() {
                let temp = words[j];
                words[j] = words[j + 1];
                words[j + 1] = temp;
            }
        }
    }
    println!("{} {} {}", words[words.len()-1], words[words.len()-2], words[words.len()-3]);
}