# estudos-de-loops-condicionais-em-Rust

## 📘 Projeto: Estudo de Loops e Condicionais em Rust

Este projeto foi criado para praticar e entender melhor loops for, condicionais if/else e intervalos numéricos em Rust.

O programa faz uma contagem de 1 até 20, classificando cada número como:

 - "número pequeno" (menor que 10)

 - "chegamos no 10!"

- "número grande" (maior que 10)

Ao final, exibe uma mensagem informando que a contagem terminou.

## 🧠 Conceitos praticados

1. Laço de repetição: for i in 1..=20

2. Estruturas condicionais: if, else if, else

3. Impressão formatada com println!

4. Operadores de comparação

## 📂 Código utilizado

fn main() {

    for i in 1..=20 {
        if i < 10 {
            println!("numero pequeno: {}", i);
        } else if i == 10 {
            println!("chegamos no 10!");
        } else {
            println!("numero grande: {}", i);
        }
    }

    println!("contagem finalizada!")
}

## 🚀 Como executar

Certifique-se de ter Rust instalado.

Compile com:

rustc main.rs


Execute:

./main

## 🎯 Objetivo

Aprender na prática como Rust trabalha com loops e decisões lógicas, criando um código simples, claro e fácil de entender.
