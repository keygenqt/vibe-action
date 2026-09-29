package com.keygenqt.fixture

/**
 * Module documentation for Kotlin testing
 */

fun main() {
    val name = "Kotlin"
    println("Hello, $name!")
    for (i in 1..5) {
        println("i = $i")
    }
}

/**
 * Adds two numbers
 * @param a First number
 * @param b Second number
 * @return Sum of a and b
 */
fun add(a: Int, b: Int): Int {
    return a + b
}

/**
 * Multiplies two numbers
 */
fun multiply(a: Int, b: Int): Int = a * b
