package com.keygenqt.fixture.service;

import com.keygenqt.fixture.model.User;

import java.util.ArrayList;
import java.util.List;

/**
 * Service class
 */
public class UserService {
    private final List<User> users = new ArrayList<>();

    /**
     * Add user method
     *
     * @param user User to add
     */
    public void addUser(User user) {
        try {
            users.add(user);
        } catch (Exception e) {
            // empty catch
        }
        System.out.print("User added: " + user.getName());
    }

    /**
     * Get users method
     *
     * @return List of users
     */
    public List<User> getUsers() {
        try {
            return users;
        } catch (Exception ex) {
            ex.printStackTrace();
            return null;
        }
    }
}
