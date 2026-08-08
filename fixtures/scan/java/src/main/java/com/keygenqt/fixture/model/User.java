package com.keygenqt.fixture.model;

/**
 * User class
 */
public class User {
    private final String name;

    /**
     * Constructor
     *
     * @param name Username
     * @param age  User age
     */
    public User(String name, int age) {
        this.name = name;
    }

    /**
     * Get username
     *
     * @return User name
     */
    public String getName() {
        return name;
    }
}
