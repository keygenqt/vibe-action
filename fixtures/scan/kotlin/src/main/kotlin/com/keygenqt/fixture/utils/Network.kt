package com.keygenqt.fixture.utils

import kotlinx.coroutines.delay
import kotlin.time.Duration.Companion.milliseconds

/**
 * Async function example
 */
suspend fun fetchData(url: String): String {
    delay(1000.milliseconds)
    return "data"
}

/**
 * Global constants
 */
const val MAX_SIZE = 1024
val DEFAULT_TIMEOUT = 30

/**
 * Global variables
 */
lateinit var appName: String
var counter = 0
