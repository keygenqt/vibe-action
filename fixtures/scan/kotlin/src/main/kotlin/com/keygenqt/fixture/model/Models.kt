package com.keygenqt.fixture.model

/**
 * User data class
 */
data class User(
    val name: String,
    val age: Int
)

/**
 * Color enum
 */
enum class Color {
    RED, GREEN, BLUE
}

/**
 * Result sealed class
 */
sealed class Result {
    data class Success(val data: String) : Result()
    data class Error(val message: String) : Result()
}
