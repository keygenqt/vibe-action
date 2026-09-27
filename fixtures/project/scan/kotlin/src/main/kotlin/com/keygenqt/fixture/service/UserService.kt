package com.keygenqt.fixture.service

import com.keygenqt.fixture.model.User

/**
 * Service class
 */
class UserService {
    private val users = mutableListOf<User>()

    /**
     * Add user method
     */
    fun addUser(user: User) {
        users.add(user)
    }

    /**
     * Get users method
     */
    fun getUsers(): List<User> {
        return users
    }
}

/**
 * Repository interface
 */
interface Repository {
    fun getData(): String
    fun saveData(data: String)
}
